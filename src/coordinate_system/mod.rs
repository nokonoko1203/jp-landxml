//! CoordinateSystem module for J-LandXML parser
//!
//! This module handles coordinate system definitions including:
//! - Standard LandXML coordinate systems
//! - J-LandXML extensions (Japan Plane Coordinate System 1-19)
//! - Horizontal/Vertical datum definitions
//! - Coordinate system validation

pub mod jlandxml;
pub mod parser;
pub mod standard;

// 標準LandXML座標系
pub use standard::CoordinateSystem;

// J-LandXML拡張 - 座標系関連
pub use jlandxml::{
    CoordinateSystemInfo, CoordinateSystemMapper, CoordinateSystemValidator, HorizontalDatum,
    JLandXmlCoordinateSystem, JLandXmlDocument, JLandXmlProperty, JapanPlaneCoordinateSystem,
    ValidationWarning, VerticalDatum,
};

// パーサー
pub use parser::{CoordinateSystemNameParser, JLandXmlParser, ParsingStats};

use crate::error::LandXMLError;

/// J-LandXML拡張機能の統一インターフェース
pub struct JLandXml;

impl JLandXml {
    /// 座標系名から平面直角座標系を解析
    pub fn parse_coordinate_system_name(
        name: &str,
    ) -> Result<Option<JapanPlaneCoordinateSystem>, LandXMLError> {
        CoordinateSystemNameParser::parse(name)
    }

    /// 平面直角座標系からEPSGコードを取得
    pub fn get_epsg_code(zone: JapanPlaneCoordinateSystem) -> u32 {
        zone.epsg_code()
    }

    /// 平面直角座標系の説明を取得
    pub fn get_zone_description(zone: JapanPlaneCoordinateSystem) -> &'static str {
        zone.description()
    }
}
