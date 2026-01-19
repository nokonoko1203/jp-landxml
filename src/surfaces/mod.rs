//! Surfaces module for J-LandXML parser
//!
//! This module handles TIN surface data:
//! - Surface definitions
//! - Triangle meshes
//! - Point clouds

use serde::{Deserialize, Serialize};

use crate::models::Point3D;

/// A triangular face in a TIN surface
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Triangle {
    /// Indices into the points array (3 vertices)
    pub vertices: [usize; 3],
}

/// TIN Surface definition
///
/// Reference: LandXML 1.2 Surface element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Surface {
    /// Surface name
    pub name: Option<String>,
    /// Surface description
    pub desc: Option<String>,
    /// Point cloud
    pub points: Vec<Point3D>,
    /// Triangle faces
    pub triangles: Vec<Triangle>,
}

impl Default for Surface {
    fn default() -> Self {
        Self {
            name: None,
            desc: None,
            points: Vec::new(),
            triangles: Vec::new(),
        }
    }
}

/// Collection of Surfaces
///
/// Reference: LandXML 1.2 Surfaces element
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Surfaces {
    /// List of surfaces
    pub surfaces: Vec<Surface>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_surface_creation() {
        let surface = Surface {
            name: Some("DesignSurface".to_string()),
            desc: None,
            points: vec![
                Point3D {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                Point3D {
                    x: 10.0,
                    y: 0.0,
                    z: 0.0,
                },
                Point3D {
                    x: 5.0,
                    y: 10.0,
                    z: 0.0,
                },
            ],
            triangles: vec![Triangle { vertices: [0, 1, 2] }],
        };
        assert_eq!(surface.name, Some("DesignSurface".to_string()));
        assert_eq!(surface.triangles.len(), 1);
    }
}
