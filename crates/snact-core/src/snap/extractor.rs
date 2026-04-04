//! Extracts raw element data from the page using DOMSnapshot + Accessibility APIs.

use std::collections::HashMap;

use snact_cdp::commands::{
    AccessibilityGetFullAXTree, AXNode, DomSnapshotCaptureSnapshot,
};
use snact_cdp::types::BackendNodeId;
use snact_cdp::CdpTransport;

/// A raw extracted element before filtering.
#[derive(Debug, Clone)]
pub struct RawElement {
    pub backend_node_id: BackendNodeId,
    pub node_index: usize,
    pub tag: String,
    pub attributes: HashMap<String, String>,
    pub role: String,
    pub name: String,
    pub value: String,
    pub bounds: Option<[f64; 4]>,
    pub is_visible: bool,
    pub ax_properties: HashMap<String, String>,
}

/// Extract all elements from the page by merging DOMSnapshot and Accessibility data.
pub async fn extract(
    transport: &CdpTransport,
) -> Result<Vec<RawElement>, snact_cdp::CdpTransportError> {
    // Enable Page events for proper load detection
    transport
        .send(&snact_cdp::commands::PageEnable {})
        .await?;

    // Capture DOM snapshot with computed styles for visibility checks
    let snapshot_cmd = DomSnapshotCaptureSnapshot {
        computed_styles: vec![
            "display".to_string(),
            "visibility".to_string(),
            "opacity".to_string(),
        ],
        include_dom_rects: Some(true),
        include_paint_order: None,
    };
    let snapshot = transport.send(&snapshot_cmd).await?;

    // Get accessibility tree
    let ax_cmd = AccessibilityGetFullAXTree {
        depth: None,
        frame_id: None,
    };
    let ax_tree = transport.send(&ax_cmd).await?;

    // Build a lookup from backendNodeId -> AXNode
    let ax_by_backend_id: HashMap<BackendNodeId, &AXNode> = ax_tree
        .nodes
        .iter()
        .filter_map(|n| n.backend_dom_node_id.map(|id| (id, n)))
        .collect();

    // Process each document in the snapshot
    let mut elements = Vec::new();

    for doc in &snapshot.documents {
        let strings = &snapshot.strings;
        let nodes = &doc.nodes;
        let layout = &doc.layout;

        // Build layout lookup: node_index -> bounds
        let mut layout_bounds: HashMap<usize, [f64; 4]> = HashMap::new();
        for (i, &node_idx) in layout.node_index.iter().enumerate() {
            if let Some(bounds) = layout.bounds.get(i) {
                if bounds.len() == 4 {
                    layout_bounds.insert(node_idx as usize, [bounds[0], bounds[1], bounds[2], bounds[3]]);
                }
            }
        }

        // Build computed styles lookup for visibility
        let mut style_values: HashMap<usize, Vec<String>> = HashMap::new();
        for (i, &node_idx) in layout.node_index.iter().enumerate() {
            if let Some(style_indices) = layout.styles.get(i) {
                let values: Vec<String> = style_indices
                    .iter()
                    .map(|&si| {
                        strings
                            .get(si as usize)
                            .cloned()
                            .unwrap_or_default()
                    })
                    .collect();
                style_values.insert(node_idx as usize, values);
            }
        }

        let node_count = nodes.node_name.len();

        for i in 0..node_count {
            let backend_id = *nodes.backend_node_id.get(i).unwrap_or(&0);
            if backend_id == 0 {
                continue;
            }

            let tag = nodes
                .node_name
                .get(i)
                .and_then(|&idx| strings.get(idx as usize))
                .cloned()
                .unwrap_or_default()
                .to_lowercase();

            // Parse attributes
            let attrs = parse_attributes(nodes.attributes.get(i), strings);

            // Get bounds
            let bounds = layout_bounds.get(&i).copied();

            // Check visibility from computed styles
            let is_visible = check_visibility(&style_values.get(&i), bounds);

            // Get accessibility info
            let (role, name, value, ax_props) = if let Some(ax_node) = ax_by_backend_id.get(&backend_id) {
                (
                    ax_value_str(&ax_node.role),
                    ax_value_str(&ax_node.name),
                    ax_value_str(&ax_node.value),
                    extract_ax_properties(ax_node),
                )
            } else {
                (String::new(), String::new(), String::new(), HashMap::new())
            };

            elements.push(RawElement {
                backend_node_id: backend_id,
                node_index: i,
                tag,
                attributes: attrs,
                role,
                name,
                value,
                bounds,
                is_visible,
                ax_properties: ax_props,
            });
        }
    }

    Ok(elements)
}

fn parse_attributes(
    attr_indices: Option<&Vec<i64>>,
    strings: &[String],
) -> HashMap<String, String> {
    let mut map = HashMap::new();
    let Some(indices) = attr_indices else {
        return map;
    };
    // Attributes come in pairs: [name_idx, value_idx, name_idx, value_idx, ...]
    for chunk in indices.chunks(2) {
        if chunk.len() == 2 {
            let key = strings.get(chunk[0] as usize).cloned().unwrap_or_default();
            let val = strings.get(chunk[1] as usize).cloned().unwrap_or_default();
            if !key.is_empty() {
                map.insert(key, val);
            }
        }
    }
    map
}

fn check_visibility(styles: &Option<&Vec<String>>, bounds: Option<[f64; 4]>) -> bool {
    // No layout = not visible
    if bounds.is_none() {
        return false;
    }

    let bounds = bounds.unwrap();
    // Zero-size = not visible
    if bounds[2] <= 0.0 || bounds[3] <= 0.0 {
        return false;
    }

    if let Some(styles) = styles {
        // styles order matches computed_styles request: [display, visibility, opacity]
        if let Some(display) = styles.first() {
            if display == "none" {
                return false;
            }
        }
        if let Some(visibility) = styles.get(1) {
            if visibility == "hidden" || visibility == "collapse" {
                return false;
            }
        }
        if let Some(opacity) = styles.get(2) {
            if let Ok(v) = opacity.parse::<f64>() {
                if v == 0.0 {
                    return false;
                }
            }
        }
    }

    true
}

fn ax_value_str(val: &Option<snact_cdp::commands::AXValue>) -> String {
    val.as_ref()
        .and_then(|v| v.value.as_ref())
        .and_then(|v| v.as_str().map(|s| s.to_string()))
        .unwrap_or_default()
}

fn extract_ax_properties(node: &AXNode) -> HashMap<String, String> {
    let mut props = HashMap::new();
    for prop in &node.properties {
        if let Some(val) = &prop.value.value {
            let val_str = match val {
                serde_json::Value::String(s) => s.clone(),
                serde_json::Value::Bool(b) => b.to_string(),
                other => other.to_string(),
            };
            props.insert(prop.name.clone(), val_str);
        }
    }
    props
}
