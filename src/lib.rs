//! SigmaOS-7-days-to-die: survival game model.
//!
//! Real implementation: resources, day cycle, and validation.
//! Replaces the 7-line stub.

use std::collections::BTreeMap;
use std::fmt;

/// Game resource.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameResource {
    pub id: String,
    pub name: String,
    pub resource_type: String,
    pub quantity: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameError {
    EmptyName,
    DuplicateResource(String),
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "resource name must not be empty"),
            Self::DuplicateResource(id) => write!(f, "duplicate resource {:?}", id),
        }
    }
}

impl std::error::Error for GameError {}

/// Survival game manager.
pub struct SurvivalGame {
    pub name: String,
    resources: BTreeMap<String, GameResource>,
    day: u32,
}

impl SurvivalGame {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            resources: BTreeMap::new(),
            day: 1,
        }
    }

    pub fn add_resource(&mut self, resource: GameResource) -> Result<(), GameError> {
        if resource.name.trim().is_empty() { return Err(GameError::EmptyName); }
        let id = resource.id.clone();
        if self.resources.contains_key(&id) {
            return Err(GameError::DuplicateResource(id.clone()));
        }
        self.resources.insert(id, resource);
        Ok(())
    }

    pub fn resource_count(&self) -> usize { self.resources.len() }

    pub fn get_resource(&self, id: &str) -> Option<&GameResource> { self.resources.get(id) }

    pub fn current_day(&self) -> u32 { self.day }

    pub fn advance_day(&mut self) { self.day += 1; }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_create_aaa() {
        let g = SurvivalGame::new("My Game");
        assert_eq!(g.name, "My Game");
        assert_eq!(g.resource_count(), 0);
        assert_eq!(g.current_day(), 1);
    }

    #[test]
    fn test_add_resource_aaa() {
        let mut g = SurvivalGame::new("My Game");
        let r = GameResource { id: "r1".into(), name: "food".into(), resource_type: "consumable".into(), quantity: 10 };
        g.add_resource(r).unwrap();
        assert_eq!(g.resource_count(), 1);
    }

    #[test]
    fn test_empty_name_rejected_aaa() {
        let mut g = SurvivalGame::new("My Game");
        let r = GameResource { id: "x".into(), name: "".into(), resource_type: "a".into(), quantity: 1 };
        assert!(g.add_resource(r).is_err());
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut g = SurvivalGame::new("My Game");
        let r = GameResource { id: "x".into(), name: "X".into(), resource_type: "a".into(), quantity: 1 };
        g.add_resource(r).unwrap();
        let r2 = GameResource { id: "x".into(), name: "X".into(), resource_type: "a".into(), quantity: 1 };
        assert!(g.add_resource(r2).is_err());
    }
}
