pub mod compressor;
pub mod extractor;
pub mod filter;

use snact_cdp::commands::{DomGetBoxModel, DomGetDocument, DomQuerySelectorAll};
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

    // Resolve focus bounds if --focus was provided
    let focus_bounds = if let Some(selector) = focus {
        resolve_focus_bounds(transport, selector)
            .await
            .unwrap_or(None)
    } else {
        None
    };

    // Filter to interactable elements (optionally constrained to focus bounds)
    let filtered = filter::filter_elements(raw_elements, focus_bounds);

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

/// Resolve the bounding box [x, y, w, h] of the first element matching `selector`.
/// Returns None if the selector matches nothing or has no layout.
async fn resolve_focus_bounds(
    transport: &CdpTransport,
    selector: &str,
) -> Result<Option<[f64; 4]>, snact_cdp::CdpTransportError> {
    let doc = transport
        .send(&DomGetDocument {
            depth: Some(0),
            pierce: None,
        })
        .await?;
    let root_id = doc.root.node_id;

    let result = transport
        .send(&DomQuerySelectorAll {
            node_id: root_id,
            selector: selector.to_string(),
        })
        .await?;

    let Some(&node_id) = result.node_ids.first() else {
        return Ok(None);
    };

    let box_model = transport
        .send(&DomGetBoxModel {
            node_id: Some(node_id),
            backend_node_id: None,
        })
        .await?;

    let content = &box_model.model.content;
    if content.len() < 8 {
        return Ok(None);
    }

    // content quad: [x0,y0, x1,y1, x2,y2, x3,y3]
    let xs: Vec<f64> = content.iter().step_by(2).copied().collect();
    let ys: Vec<f64> = content.iter().skip(1).step_by(2).copied().collect();
    let x_min = xs.iter().cloned().fold(f64::INFINITY, f64::min);
    let y_min = ys.iter().cloned().fold(f64::INFINITY, f64::min);
    let x_max = xs.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let y_max = ys.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

    Ok(Some([x_min, y_min, x_max - x_min, y_max - y_min]))
}
