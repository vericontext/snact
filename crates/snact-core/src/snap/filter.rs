//! Filters raw elements to only interactable, visible elements.

use super::extractor::RawElement;

/// Roles that indicate an interactable element.
const INTERACTABLE_ROLES: &[&str] = &[
    "button",
    "link",
    "textbox",
    "checkbox",
    "radio",
    "combobox",
    "menuitem",
    "tab",
    "switch",
    "slider",
    "searchbox",
    "option",
    "spinbutton",
    "listbox",
];

/// Tags that indicate an interactable element (when role is missing).
const INTERACTABLE_TAGS: &[&str] = &[
    "a", "button", "input", "select", "textarea", "details", "summary",
];

/// Filter raw elements to only those that are interactable and visible.
pub fn filter_elements(elements: Vec<RawElement>, focus_selector: Option<&str>) -> Vec<RawElement> {
    let _ = focus_selector; // TODO: implement focus filtering via CDP DOM.querySelector

    elements
        .into_iter()
        .filter(|el| {
            // Must be visible
            if !el.is_visible {
                return false;
            }

            // Check if aria-hidden
            if el.attributes.get("aria-hidden").map(|v| v.as_str()) == Some("true") {
                return false;
            }

            // Check role
            if !el.role.is_empty() && INTERACTABLE_ROLES.contains(&el.role.as_str()) {
                return true;
            }

            // Check tag
            if INTERACTABLE_TAGS.contains(&el.tag.as_str()) {
                return true;
            }

            // Check for contenteditable
            if el.attributes.get("contenteditable").map(|v| v.as_str()) == Some("true") {
                return true;
            }

            // Check for role attribute on div/span
            if let Some(role_attr) = el.attributes.get("role") {
                if INTERACTABLE_ROLES.contains(&role_attr.as_str()) {
                    return true;
                }
            }

            // Check for tabindex (explicitly focusable)
            if el.attributes.contains_key("tabindex") {
                return true;
            }

            false
        })
        .collect()
}
