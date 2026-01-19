//! Horizontal alignment elements (CoordGeom)
//!
//! LandXML 1.2 specification:
//! - Line: Straight segment
//! - Curve: Circular arc
//! - Spiral: Transition curve (Clothoid, etc.)

use serde::{Deserialize, Serialize};

use crate::models::Point2D;

/// Rotation direction for curves and spirals
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RotationDirection {
    /// Clockwise
    Cw,
    /// Counter-clockwise
    Ccw,
}

/// Spiral (transition curve) type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpiralType {
    /// Clothoid curve (Cornu spiral) - most common in road design
    Clothoid,
    /// Bloss curve
    Bloss,
    /// Cubic spiral
    Cubic,
    /// Cubic parabola
    CubicParabola,
    /// Sinusoidal curve
    Sinusoid,
    /// Cosinusoidal curve
    Cosinoid,
    /// Biquadratic parabola
    BiquadraticParabola,
    /// Radioid curve
    Radioid,
    /// Other spiral type
    Other(String),
}

impl Default for SpiralType {
    fn default() -> Self {
        SpiralType::Clothoid
    }
}

/// Straight line segment (LandXML Line element)
///
/// Reference: http://www.landxml.org/schema/LandXML-1.2/documentation/LandXML-1.2Doc_Line.html
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Line {
    /// Start point coordinates
    pub start: Point2D,
    /// End point coordinates
    pub end: Point2D,

    /// Computed or measured length (meters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<f64>,
    /// Direction angle in radians (0 = North, clockwise positive)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<f64>,
    /// Starting station (chainage)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sta_start: Option<f64>,

    /// Element name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    /// State (existing, proposed, asBuilt, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// Circular arc (LandXML Curve element)
///
/// Reference: http://www.landxml.org/schema/LandXML-1.2/documentation/LandXML-1.2Doc_Curve.html
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Curve {
    /// Start point coordinates
    pub start: Point2D,
    /// End point coordinates
    pub end: Point2D,
    /// Center point coordinates
    #[serde(skip_serializing_if = "Option::is_none")]
    pub center: Option<Point2D>,
    /// Point of intersection (PI)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pi: Option<Point2D>,

    /// Rotation direction (required)
    pub rot: RotationDirection,

    /// Radius in meters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius: Option<f64>,
    /// Arc length in meters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<f64>,
    /// Chord length in meters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chord: Option<f64>,
    /// Central angle (delta) in radians
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delta: Option<f64>,
    /// Tangent length in meters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tangent: Option<f64>,
    /// External distance in meters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external: Option<f64>,
    /// Middle ordinate in meters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mid_ord: Option<f64>,

    /// Direction at start point (radians)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir_start: Option<f64>,
    /// Direction at end point (radians)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir_end: Option<f64>,
    /// Starting station (chainage)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sta_start: Option<f64>,

    /// Element name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    /// State (existing, proposed, asBuilt, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// Spiral / Transition curve (LandXML Spiral element)
///
/// Reference: http://www.landxml.org/schema/LandXML-1.2/documentation/LandXML-1.2Doc_Spiral.html
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spiral {
    /// Start point coordinates
    pub start: Point2D,
    /// End point coordinates
    pub end: Point2D,
    /// Point of intersection (PI)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pi: Option<Point2D>,

    /// Length of spiral in meters (required)
    pub length: f64,
    /// Radius at start (None or INF means straight line)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius_start: Option<f64>,
    /// Radius at end (None or INF means straight line)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius_end: Option<f64>,
    /// Rotation direction
    pub rot: RotationDirection,
    /// Spiral type (Clothoid is most common)
    #[serde(default)]
    pub spi_type: SpiralType,

    /// Clothoid parameter A (A² = R × L)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constant: Option<f64>,

    /// Tangent angle (theta) in radians
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theta: Option<f64>,
    /// Total X coordinate offset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_x: Option<f64>,
    /// Total Y coordinate offset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_y: Option<f64>,
    /// Long tangent length
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tan_long: Option<f64>,
    /// Short tangent length
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tan_short: Option<f64>,
    /// Chord length
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chord: Option<f64>,

    /// Direction at start point (radians)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir_start: Option<f64>,
    /// Direction at end point (radians)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir_end: Option<f64>,
    /// Starting station (chainage)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sta_start: Option<f64>,

    /// Element name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    /// State (existing, proposed, asBuilt, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// Geometry element (horizontal alignment component)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GeomElement {
    Line(Line),
    Curve(Curve),
    Spiral(Spiral),
}

/// Coordinate geometry (horizontal alignment)
///
/// Contains a sequence of Line, Curve, and Spiral elements
/// that define the horizontal path of an alignment.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CoordGeom {
    /// Sequence of geometry elements
    pub elements: Vec<GeomElement>,

    /// Element name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    /// State
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_creation() {
        let line = Line {
            start: Point2D { x: 0.0, y: 0.0 },
            end: Point2D { x: 100.0, y: 0.0 },
            length: Some(100.0),
            dir: Some(std::f64::consts::FRAC_PI_2), // East
            sta_start: Some(0.0),
            name: Some("L1".to_string()),
            desc: None,
            state: None,
        };
        assert_eq!(line.length, Some(100.0));
    }

    #[test]
    fn test_curve_creation() {
        let curve = Curve {
            start: Point2D { x: 0.0, y: 0.0 },
            end: Point2D { x: 100.0, y: 100.0 },
            center: Some(Point2D { x: 100.0, y: 0.0 }),
            pi: None,
            rot: RotationDirection::Ccw,
            radius: Some(100.0),
            length: None,
            chord: None,
            delta: None,
            tangent: None,
            external: None,
            mid_ord: None,
            dir_start: None,
            dir_end: None,
            sta_start: Some(100.0),
            name: Some("C1".to_string()),
            desc: None,
            state: None,
        };
        assert_eq!(curve.rot, RotationDirection::Ccw);
    }

    #[test]
    fn test_spiral_creation() {
        let spiral = Spiral {
            start: Point2D { x: 0.0, y: 0.0 },
            end: Point2D { x: 50.0, y: 10.0 },
            pi: None,
            length: 50.0,
            radius_start: None, // Starts from straight
            radius_end: Some(200.0),
            rot: RotationDirection::Cw,
            spi_type: SpiralType::Clothoid,
            constant: Some(100.0), // A = 100
            theta: None,
            total_x: None,
            total_y: None,
            tan_long: None,
            tan_short: None,
            chord: None,
            dir_start: None,
            dir_end: None,
            sta_start: Some(200.0),
            name: Some("S1".to_string()),
            desc: None,
            state: None,
        };
        assert_eq!(spiral.spi_type, SpiralType::Clothoid);
    }

    #[test]
    fn test_serialize_deserialize() {
        let line = Line {
            start: Point2D { x: 0.0, y: 0.0 },
            end: Point2D { x: 100.0, y: 0.0 },
            length: Some(100.0),
            dir: None,
            sta_start: None,
            name: None,
            desc: None,
            state: None,
        };
        let json = serde_json::to_string(&line).unwrap();
        let parsed: Line = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.start.x, 0.0);
        assert_eq!(parsed.end.x, 100.0);
    }
}
