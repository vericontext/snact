//! Compresses filtered elements into token-minimal output format.
//! Output format: `@e1 [button] "Sign In"`

use super::extractor::RawElement;
use crate::element_map::{ElementEntry, ElementMap};

/// Compress filtered elements into output string and element map.
pub fn compress(elements: Vec<RawElement>) -> (String, ElementMap) {
    let mut map = ElementMap::new();
    let mut lines = Vec::new();

    for el in elements {
        let selector_hint = build_selector_hint(&el);

        let entry = ElementEntry {
            backend_node_id: el.backend_node_id,
            role: if el.role.is_empty() {
                tag_to_role(&el.tag, &el.attributes)
            } else {
                el.role.clone()
            },
            name: el.name.clone(),
            selector_hint,
            tag: el.tag.clone(),
            attributes: el.attributes.clone(),
        };

        let ref_id = map.insert(entry.clone());
        let line = format_element_line(&ref_id, &entry, &el);
        lines.push(line);
    }

    (lines.join("\n"), map)
}

fn format_element_line(ref_id: &str, entry: &ElementEntry, raw: &RawElement) -> String {
    let mut parts = Vec::new();

    // @eN
    parts.push(ref_id.to_string());

    // [role] or [tag:type]
    let role_display = format_role_display(&entry.role, &entry.tag, &raw.attributes);
    parts.push(format!("[{role_display}]"));

    // "name" — accessible name or best fallback label in quotes
    let label = best_label(entry, raw);
    if !label.is_empty() {
        parts.push(format!("\"{}\"", label));
    }

    // Extra decision-relevant attributes
    let extras = format_extras(raw);
    if !extras.is_empty() {
        parts.push(extras);
    }

    parts.join(" ")
}

fn format_role_display(
    role: &str,
    tag: &str,
    attrs: &std::collections::HashMap<String, String>,
) -> String {
    match tag {
        "input" => {
            let input_type = attrs.get("type").map(|s| s.as_str()).unwrap_or("text");
            format!("input:{input_type}")
        }
        _ => {
            if role.is_empty() {
                tag.to_string()
            } else {
                role.to_string()
            }
        }
    }
}

/// Pick the best human-readable label for an element.
/// Priority: accessible name > aria-label > title > placeholder > id
fn best_label(entry: &ElementEntry, raw: &RawElement) -> String {
    if !entry.name.is_empty() {
        return truncate(&entry.name, 60);
    }
    if let Some(v) = raw.attributes.get("aria-label") {
        if !v.is_empty() {
            return truncate(v, 60);
        }
    }
    if let Some(v) = raw.attributes.get("title") {
        if !v.is_empty() {
            return truncate(v, 60);
        }
    }
    if let Some(v) = raw.attributes.get("placeholder") {
        if !v.is_empty() {
            return truncate(v, 60);
        }
    }
    String::new()
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}...", &s[..max - 3])
    }
}

fn format_extras(el: &RawElement) -> String {
    let mut extras = Vec::new();

    // id — useful for identifying elements
    if let Some(id) = el.attributes.get("id") {
        if !id.is_empty() {
            extras.push(format!("id=\"{id}\""));
        }
    }

    // placeholder (only if not already used as label)
    if let Some(ph) = el.attributes.get("placeholder") {
        if !ph.is_empty()
            && el.name.is_empty()
            && !el.attributes.contains_key("aria-label")
            && !el.attributes.contains_key("title")
        {
            // placeholder was used as label, skip in extras
        } else if !ph.is_empty() {
            extras.push(format!("placeholder=\"{ph}\""));
        }
    }

    // href for links (truncated)
    if let Some(href) = el.attributes.get("href") {
        if !href.is_empty() {
            let display = if href.len() > 50 {
                format!("{}...", &href[..47])
            } else {
                href.clone()
            };
            extras.push(format!("href=\"{display}\""));
        }
    }

    // checked state
    if el.ax_properties.get("checked").map(|v| v.as_str()) == Some("true")
        || el.attributes.contains_key("checked")
    {
        extras.push("checked".to_string());
    }

    // disabled state
    if el.ax_properties.get("disabled").map(|v| v.as_str()) == Some("true")
        || el.attributes.contains_key("disabled")
    {
        extras.push("disabled".to_string());
    }

    // current value for inputs
    if !el.value.is_empty() && el.value != el.name {
        extras.push(format!("value=\"{}\"", el.value));
    }

    extras.join(" ")
}

fn build_selector_hint(el: &RawElement) -> String {
    let mut selector = el.tag.clone();

    if let Some(id) = el.attributes.get("id") {
        if !id.is_empty() {
            return format!("#{id}");
        }
    }

    if let Some(name) = el.attributes.get("name") {
        if !name.is_empty() {
            selector.push_str(&format!("[name=\"{name}\"]"));
            return selector;
        }
    }

    if let Some(input_type) = el.attributes.get("type") {
        selector.push_str(&format!("[type=\"{input_type}\"]"));
    }

    selector
}

fn tag_to_role(tag: &str, attrs: &std::collections::HashMap<String, String>) -> String {
    match tag {
        "a" => "link".to_string(),
        "button" => "button".to_string(),
        "input" => {
            let t = attrs.get("type").map(|s| s.as_str()).unwrap_or("text");
            match t {
                "checkbox" => "checkbox".to_string(),
                "radio" => "radio".to_string(),
                "submit" | "button" | "reset" => "button".to_string(),
                _ => "textbox".to_string(),
            }
        }
        "select" => "combobox".to_string(),
        "textarea" => "textbox".to_string(),
        _ => tag.to_string(),
    }
}
