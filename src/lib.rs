//! jp-landxml: LandXML/J-LandXML parser for Rust
//!
//! This library provides parsing and data structures for LandXML and J-LandXML documents,
//! with special support for Japan's plane rectangular coordinate systems (1-19).

// Core modules
pub mod coordinate_system;
pub mod error;
pub mod models;

// LandXML element modules
pub mod alignments;
pub mod application;
pub mod cg_points;
pub mod project;
pub mod roadways;
pub mod surfaces;
pub mod units;

// 後方互換性のためのエイリアス
pub use alignments as alignment;
pub use coordinate_system as jlandxml;

// Re-exports from error
pub use crate::error::LandXMLError;

// Re-exports from models (common types)
pub use crate::models::*;

// Re-exports from coordinate_system (J-LandXML extensions)
pub use crate::coordinate_system::{
    CoordinateSystem, CoordinateSystemInfo, CoordinateSystemMapper, CoordinateSystemNameParser,
    CoordinateSystemValidator, HorizontalDatum, JLandXml, JLandXmlCoordinateSystem,
    JLandXmlDocument, JLandXmlParser, JLandXmlProperty, JapanPlaneCoordinateSystem,
    ParsingStats, ValidationWarning, VerticalDatum,
};

// Re-exports from alignments
pub use crate::alignments::{
    CoordGeom, Curve, GeomElement, Line, RotationDirection, Spiral, SpiralType,
};

// Re-exports from units
pub use crate::units::{AngularUnit, LinearUnit, Units};

// Re-exports from surfaces
pub use crate::surfaces::{Surface, Surfaces, Triangle};
