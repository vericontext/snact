pub mod compressor;
pub mod extractor;
pub mod filter;

use snact_cdp::CdpTransport;

use crate::element_map::ElementMap;

/// Result of a snap operation.
pub struct SnapResult {
    pub output: String,
    pub element_map: ElementMap,
    pub element_count: usize,
}

/// Execute a snap: extract interactable elements from the current page.
pub async fn execute(
    transport: &CdpTransport,
    url: Option<&str>,
    focus: Option<&str>,
    lang: &str,
) -> Result<SnapResult, snact_cdp::CdpTransportError> {
    // Set Accept-Language header to control page content language
    {
        use snact_cdp::commands::{NetworkEnable, NetworkSetExtraHTTPHeaders};
        transport.send(&NetworkEnable {}).await?;
        let mut headers = std::collections::HashMap::new();
        headers.insert("Accept-Language".to_string(), format!("{lang},en;q=0.9"));
        transport
            .send(&NetworkSetExtraHTTPHeaders { headers })
            .await?;
    }

    // Navigate if URL provided
    if let Some(url) = url {
        use snact_cdp::commands::PageNavigate;
        let nav = PageNavigate {
            url: url.to_string(),
        };
        let resp = transport.send(&nav).await?;
        if let Some(err) = resp.error_text {
            return Err(snact_cdp::CdpTransportError::CommandFailed {
                method: "Page.navigate".into(),
                code: -1,
                message: err,
            });
        }

        // Wait for page load
        transport
            .wait_for_event("Page.loadEventFired", std::time::Duration::from_secs(30))
            .await?;
    }

    // Extract elements
    let raw_elements = extractor::extract(transport).await?;

    // Filter to interactable elements
    let filtered = filter::filter_elements(raw_elements, focus);

    // Compress into output format and build element map
    let (output, element_map) = compressor::compress(filtered);
    let element_count = element_map.elements.len();

    // Persist element map
    element_map
        .save()
        .map_err(|e| snact_cdp::CdpTransportError::ConnectionFailed(e.to_string()))?;

    Ok(SnapResult {
        output,
        element_map,
        element_count,
    })
}
