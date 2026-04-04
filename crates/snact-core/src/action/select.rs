//! Select action: set value on <select> elements.

use snact_cdp::commands::{CallArgument, DomResolveNode, RuntimeCallFunctionOn};
use snact_cdp::CdpTransport;

use crate::element_map::ElementMap;

pub async fn execute(
    transport: &CdpTransport,
    ref_id: &str,
    value: &str,
) -> Result<(), snact_cdp::CdpTransportError> {
    let map = ElementMap::load().map_err(|e| {
        snact_cdp::CdpTransportError::ConnectionFailed(format!("Failed to load element map: {e}"))
    })?;

    let entry = map
        .get(ref_id)
        .ok_or_else(|| snact_cdp::CdpTransportError::CommandFailed {
            method: "select".into(),
            code: -1,
            message: format!("Element {ref_id} not found. Run `snact snap` first."),
        })?;

    let resolved = transport
        .send(&DomResolveNode {
            node_id: None,
            backend_node_id: Some(entry.backend_node_id),
            object_group: Some("snact".to_string()),
        })
        .await?;

    let object_id =
        resolved
            .object
            .object_id
            .ok_or_else(|| snact_cdp::CdpTransportError::CommandFailed {
                method: "select".into(),
                code: -1,
                message: "Could not resolve element to remote object".into(),
            })?;

    let js = r#"
        function(newValue) {
            this.value = newValue;
            this.dispatchEvent(new Event('change', { bubbles: true }));
        }
    "#;

    transport
        .send(&RuntimeCallFunctionOn {
            function_declaration: js.to_string(),
            object_id: Some(object_id),
            arguments: Some(vec![CallArgument {
                value: Some(serde_json::Value::String(value.to_string())),
                object_id: None,
            }]),
            return_by_value: Some(true),
            await_promise: None,
        })
        .await?;

    Ok(())
}
