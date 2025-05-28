use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LandXML {
    pub version: String,
    pub coordinate_system: Option<CoordinateSystem>,
    pub surfaces: Vec<Surface>,
    pub alignments: Vec<Alignment>,
    pub features: Vec<Feature>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoordinateSystem {
    pub name: String,
    pub epsg_code: Option<String>,
    pub proj4_string: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Surface {
    pub name: String,
    pub surface_type: SurfaceType,
    pub definition: SurfaceDefinition,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SurfaceType {
    ExistingGround,
    DesignGround,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurfaceDefinition {
    pub points: Vec<Point3D>,
    pub faces: Vec<Face>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Face {
    pub p1: usize,
    pub p2: usize,
    pub p3: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alignment {
    pub name: String,
    pub coord_geom: CoordGeom,
    pub profile: Option<Profile>,
    pub cross_sections: Vec<CrossSection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoordGeom {
    pub elements: Vec<GeometryElement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GeometryElement {
    Line {
        start: Point2D,
        end: Point2D,
        length: f64,
    },
    Curve {
        center: Point2D,
        radius: f64,
        start_angle: f64,
        end_angle: f64,
        length: f64,
    },
    Spiral {
        start: Point2D,
        end: Point2D,
        radius_start: Option<f64>,
        radius_end: Option<f64>,
        length: f64,
        clothoid_param: f64,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub name: String,
    pub elements: Vec<ProfileElement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProfileElement {
    ParaCurve {
        length: f64,
        start_grade: f64,
        end_grade: f64,
    },
    CircularCurve {
        length: f64,
        radius: f64,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossSection {
    pub station: f64,
    pub left_width: f64,
    pub right_width: f64,
    pub superelevation: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feature {
    pub code: String,
    pub properties: HashMap<String, String>,
    pub geometry: Option<FeatureGeometry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FeatureGeometry {
    Point(Point3D),
    Line(Vec<Point3D>),
    Polygon(Vec<Point3D>),
}