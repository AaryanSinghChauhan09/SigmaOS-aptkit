// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-aptkit: aptkit component model.
//!
//! Real implementation: aptkit_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AptkitItem {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AptkitError {
    EmptyAptkitItemName,
    DuplicateAptkitItem(String),
    EmptyEventSubject,
}

impl fmt::Display for AptkitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyAptkitItemName => write!(f, "aptkit_item name must not be empty"),
            Self::DuplicateAptkitItem(id) => write!(f, "duplicate aptkit_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for AptkitError {}

pub struct Aptkit {
    pub name: String,
    aptkit_items: BTreeMap<String, AptkitItem>,
    events: BTreeMap<String, Event>,
}

impl Aptkit {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            aptkit_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_aptkit_item(&mut self, item: AptkitItem) -> Result<(), AptkitError> {
        if item.name.trim().is_empty() {
            return Err(AptkitError::EmptyAptkitItemName);
        }
        let id = item.id.clone();
        if self.aptkit_items.contains_key(&id) {
            return Err(AptkitError::DuplicateAptkitItem(id.clone()));
        }
        self.aptkit_items.insert(id, item);
        Ok(())
    }

    pub fn aptkit_item_count(&self) -> usize {
        self.aptkit_items.len()
    }

    pub fn get_aptkit_item(&self, id: &str) -> Option<&AptkitItem> {
        self.aptkit_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), AptkitError> {
        if event.subject.trim().is_empty() {
            return Err(AptkitError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_aptkit_item(id: &str, name: &str) -> AptkitItem {
        AptkitItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Aptkit::new("Aptkit");
        assert_eq!(m.name, "Aptkit");
        assert_eq!(m.aptkit_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_aptkit_item() {
        let mut m = Aptkit::new("Aptkit");
        m.add_aptkit_item(sample_aptkit_item("a1", "Alpha")).unwrap();
        assert_eq!(m.aptkit_item_count(), 1);
        assert!(m.get_aptkit_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Aptkit::new("Aptkit");
        assert!(m.add_aptkit_item(sample_aptkit_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Aptkit::new("Aptkit");
        m.add_aptkit_item(sample_aptkit_item("x", "One")).unwrap();
        assert!(m.add_aptkit_item(sample_aptkit_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Aptkit::new("Aptkit");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
