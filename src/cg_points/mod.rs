//! CgPoints module for J-LandXML parser
//!
//! This module handles control/ground points:
//! - Individual CgPoint elements
//! - Point collections

use serde::{Deserialize, Serialize};

use crate::models::Point3D;

/// A single control/ground point
///
/// Reference: LandXML 1.2 CgPoint element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CgPoint {
    /// Point name/identifier
    pub name: Option<String>,
    /// Point description
    pub desc: Option<String>,
    /// Point code (classification)
    pub code: Option<String>,
    /// 3D position
    pub position: Point3D,
    /// Point type
    pub point_type: Option<String>,
}

/// Collection of CgPoints
///
/// Reference: LandXML 1.2 CgPoints element
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CgPoints {
    /// List of control/ground points
    pub points: Vec<CgPoint>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cg_point_creation() {
        let point = CgPoint {
            name: Some("CP1".to_string()),
            desc: None,
            code: Some("BM".to_string()),
            position: Point3D {
                x: 1000.0,
                y: 2000.0,
                z: 100.0,
            },
            point_type: None,
        };
        assert_eq!(point.name, Some("CP1".to_string()));
    }
}
