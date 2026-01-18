/// J-LandXML Ver.1.6 拡張仕様対応モジュール
///
/// 標準LandXMLに加えて、日本独自の拡張仕様をサポートします：
/// - 平面直角座標系1系～19系対応
/// - horizontalCoordinateSystemName属性処理
/// - 日本測地系・世界測地系対応
pub mod coordinate_systems;
pub mod models;
pub mod parser;

pub use coordinate_systems::{
    CoordinateSystemMapper, CoordinateSystemValidator, HorizontalDatum, JapanPlaneCoordinateSystem,
    ValidationWarning, VerticalDatum,
};
pub use models::{
    CoordinateSystemInfo, JLandXmlCoordinateSystem, JLandXmlDocument, JLandXmlProperty,
};
pub use parser::{CoordinateSystemNameParser, JLandXmlParser};

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
