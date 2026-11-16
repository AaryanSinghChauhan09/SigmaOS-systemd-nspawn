// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-systemd-nspawn: systemd nspawn component model.
//!
//! Real implementation: nspawns, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Nspawn {
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
pub enum SystemdNspawnError {
    EmptyNspawnName,
    DuplicateNspawn(String),
    EmptyEventSubject,
}

impl fmt::Display for SystemdNspawnError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyNspawnName => write!(f, "nspawn name must not be empty"),
            Self::DuplicateNspawn(id) => write!(f, "duplicate nspawn {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for SystemdNspawnError {}

pub struct SystemdNspawn {
    pub name: String,
    nspawns: BTreeMap<String, Nspawn>,
    events: BTreeMap<String, Event>,
}

impl SystemdNspawn {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            nspawns: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_nspawn(&mut self, item: Nspawn) -> Result<(), SystemdNspawnError> {
        if item.name.trim().is_empty() {
            return Err(SystemdNspawnError::EmptyNspawnName);
        }
        let id = item.id.clone();
        if self.nspawns.contains_key(&id) {
            return Err(SystemdNspawnError::DuplicateNspawn(id.clone()));
        }
        self.nspawns.insert(id, item);
        Ok(())
    }

    pub fn nspawn_count(&self) -> usize {
        self.nspawns.len()
    }

    pub fn get_nspawn(&self, id: &str) -> Option<&Nspawn> {
        self.nspawns.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), SystemdNspawnError> {
        if event.subject.trim().is_empty() {
            return Err(SystemdNspawnError::EmptyEventSubject);
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

    fn sample_nspawn(id: &str, name: &str) -> Nspawn {
        Nspawn {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = SystemdNspawn::new("SystemdNspawn");
        assert_eq!(m.name, "SystemdNspawn");
        assert_eq!(m.nspawn_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_nspawn() {
        let mut m = SystemdNspawn::new("SystemdNspawn");
        m.add_nspawn(sample_nspawn("a1", "Alpha")).unwrap();
        assert_eq!(m.nspawn_count(), 1);
        assert!(m.get_nspawn("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = SystemdNspawn::new("SystemdNspawn");
        assert!(m.add_nspawn(sample_nspawn("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = SystemdNspawn::new("SystemdNspawn");
        m.add_nspawn(sample_nspawn("x", "One")).unwrap();
        assert!(m.add_nspawn(sample_nspawn("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = SystemdNspawn::new("SystemdNspawn");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
