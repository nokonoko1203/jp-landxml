//! Roadways module for J-LandXML parser
//!
//! This module handles road structure data:
//! - Road definitions
//! - Pavement structures
//! - Width specifications

use serde::{Deserialize, Serialize};

/// Roadway definition
///
/// Reference: LandXML 1.2 Roadway element
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Roadway {
    /// Roadway name
    pub name: Option<String>,
    /// References to Alignment names
    pub alignment_refs: Vec<String>,
    /// Start station
    pub sta_start: Option<f64>,
    /// End station
    pub sta_end: Option<f64>,
}

/// Collection of Roadways
///
/// Reference: LandXML 1.2 Roadways element
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Roadways {
    /// List of roadways
    pub roadways: Vec<Roadway>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roadway_creation() {
        let roadway = Roadway {
            name: Some("MainRoad".to_string()),
            alignment_refs: vec!["Alignment1".to_string()],
            sta_start: Some(0.0),
            sta_end: Some(1000.0),
        };
        assert_eq!(roadway.name, Some("MainRoad".to_string()));
    }
}
