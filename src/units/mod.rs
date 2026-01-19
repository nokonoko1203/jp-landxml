//! Units module for J-LandXML parser
//!
//! This module handles unit definitions including:
//! - Linear units (meter, foot, etc.)
//! - Angular units (degree, radian, etc.)
//! - Area/Volume units

use serde::{Deserialize, Serialize};

/// Linear unit types
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum LinearUnit {
    #[default]
    Meter,
    Foot,
    UsSurveyFoot,
}

/// Angular unit types
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum AngularUnit {
    #[default]
    DecimalDegrees,
    Gradian,
    Radian,
}

/// Area unit types
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum AreaUnit {
    #[default]
    SquareMeter,
    SquareFoot,
    Hectare,
    Acre,
}

/// Volume unit types
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum VolumeUnit {
    #[default]
    CubicMeter,
    CubicFoot,
    CubicYard,
}

/// Units definition for LandXML document
///
/// Reference: LandXML 1.2 Units element
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Units {
    pub linear_unit: LinearUnit,
    pub angular_unit: AngularUnit,
    pub area_unit: AreaUnit,
    pub volume_unit: VolumeUnit,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_units() {
        let units = Units::default();
        assert_eq!(units.linear_unit, LinearUnit::Meter);
        assert_eq!(units.angular_unit, AngularUnit::DecimalDegrees);
        assert_eq!(units.area_unit, AreaUnit::SquareMeter);
        assert_eq!(units.volume_unit, VolumeUnit::CubicMeter);
    }
}
