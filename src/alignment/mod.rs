//! Alignment module for J-LandXML parser
//!
//! This module contains data structures for road/river alignment elements:
//! - CoordGeom: Horizontal alignment (Line, Curve, Spiral)
//! - Profile: Vertical alignment (PVI, ParaCurve, CircCurve)
//! - CrossSection: Cross-sectional data

pub mod coord_geom;

pub use coord_geom::*;
