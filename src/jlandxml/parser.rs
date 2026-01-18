use super::coordinate_systems::{CoordinateSystemMapper, JapanPlaneCoordinateSystem};
use super::models::{JLandXmlCoordinateSystem, JLandXmlDocument};
use crate::error::LandXMLError;
use crate::parser::LandXMLParser;
/// J-LandXML Ver.1.6 専用パーサー
///
/// 標準LandXMLパーサーを拡張して、J-LandXML特有の属性と要素をサポートします。
use quick_xml::events::Event;
use quick_xml::Reader;

/// 座標系名パーサー
pub struct CoordinateSystemNameParser;

impl CoordinateSystemNameParser {
    /// horizontalCoordinateSystemName属性をパース
    pub fn parse(name: &str) -> Result<Option<JapanPlaneCoordinateSystem>, LandXMLError> {
        CoordinateSystemMapper::parse_horizontal_coordinate_system_name(name)
    }

    /// 座標系名の妥当性をチェック
    pub fn validate(name: &str) -> bool {
        Self::parse(name).map(|opt| opt.is_some()).unwrap_or(false)
    }

    /// 座標系名から詳細情報を取得
    pub fn get_info(name: &str) -> Result<Option<String>, LandXMLError> {
        match Self::parse(name)? {
            Some(zone) => Ok(Some(format!("{} - {}", zone, zone.description()))),
            None => Ok(None),
        }
    }
}

/// J-LandXML専用パーサー
pub struct JLandXmlParser {
    /// ファイルパス（再パース用）
    file_path: std::path::PathBuf,
}

impl JLandXmlParser {
    /// ファイルから作成
    pub fn from_file<P: AsRef<std::path::Path>>(path: P) -> Result<Self, LandXMLError> {
        let file_path = path.as_ref().to_path_buf();
        // ファイルの存在を確認
        if !file_path.exists() {
            return Err(LandXMLError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("File not found: {}", file_path.display()),
            )));
        }
        Ok(Self { file_path })
    }

    /// J-LandXML文書をパース
    pub fn parse(self) -> Result<JLandXmlDocument, LandXMLError> {
        // まず標準LandXMLとしてパース
        let base_parser = LandXMLParser::from_file(&self.file_path)?;
        let base_landxml = base_parser.parse()?;
        let mut jlandxml_doc = JLandXmlDocument::from_base(base_landxml);

        // J-LandXML拡張属性をパース
        self.parse_jlandxml_extensions(&mut jlandxml_doc)?;

        Ok(jlandxml_doc)
    }

    /// J-LandXML拡張属性をパース
    fn parse_jlandxml_extensions(&self, doc: &mut JLandXmlDocument) -> Result<(), LandXMLError> {
        use std::fs;
        let content = fs::read_to_string(&self.file_path)?;
        let mut reader = Reader::from_str(&content);
        reader.trim_text(true);

        let mut buf = Vec::new();
        let mut in_coordinate_system = false;
        let mut current_coordinate_system: Option<JLandXmlCoordinateSystem> = None;

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    match e.name().as_ref() {
                        b"CoordinateSystem" => {
                            in_coordinate_system = true;
                            // 既存の座標系情報から開始
                            if let Some(base_cs) = &doc.base.coordinate_system {
                                current_coordinate_system =
                                    Some(JLandXmlCoordinateSystem::from_base(base_cs.clone()));
                            }

                            // J-LandXML拡張属性をパース
                            self.parse_coordinate_system_attributes(
                                e,
                                &mut current_coordinate_system,
                            )?;
                        }
                        b"Project" => {
                            // Project要素からJ-LandXML識別情報を抽出
                            self.parse_project_attributes(e, doc)?;
                        }
                        _ => {}
                    }
                }
                Ok(Event::End(ref e)) => match e.name().as_ref() {
                    b"CoordinateSystem" => {
                        if in_coordinate_system {
                            if let Some(cs) = current_coordinate_system.take() {
                                doc.coordinate_system = Some(cs);
                            }
                            in_coordinate_system = false;
                        }
                    }
                    _ => {}
                },
                Ok(Event::Eof) => break,
                Err(e) => {
                    return Err(LandXMLError::ParseError(format!(
                        "XML parsing error: {}",
                        e
                    )))
                }
                _ => {}
            }
            buf.clear();
        }

        Ok(())
    }

    /// CoordinateSystem要素のJ-LandXML拡張属性をパース
    fn parse_coordinate_system_attributes(
        &self,
        element: &quick_xml::events::BytesStart<'_>,
        coordinate_system: &mut Option<JLandXmlCoordinateSystem>,
    ) -> Result<(), LandXMLError> {
        if let Some(ref mut cs) = coordinate_system {
            for attr in element.attributes() {
                let attr = attr.map_err(|e| {
                    LandXMLError::ParseError(format!("Attribute parsing error: {}", e))
                })?;
                let key = std::str::from_utf8(attr.key.as_ref()).map_err(|e| {
                    LandXMLError::ParseError(format!("UTF-8 conversion error: {}", e))
                })?;
                let value = std::str::from_utf8(&attr.value).map_err(|e| {
                    LandXMLError::ParseError(format!("UTF-8 conversion error: {}", e))
                })?;

                match key {
                    "horizontalCoordinateSystemName" => {
                        *cs = cs
                            .clone()
                            .with_horizontal_coordinate_system_name(value.to_string());
                    }
                    "verticalDatum" => {
                        if let Ok(vertical_datum) =
                            super::coordinate_systems::VerticalDatum::from_str(value)
                        {
                            *cs = cs.clone().with_vertical_datum(vertical_datum);
                        }
                    }
                    "geoidName" => {
                        *cs = cs.clone().with_geoid_name(value.to_string());
                    }
                    _ => {} // 他の属性は標準LandXMLパーサーが処理
                }
            }
        }
        Ok(())
    }

    /// Project要素からJ-LandXML識別情報を抽出
    fn parse_project_attributes(
        &self,
        element: &quick_xml::events::BytesStart<'_>,
        doc: &mut JLandXmlDocument,
    ) -> Result<(), LandXMLError> {
        // Project内のFeature要素を探してJ-LandXML識別プロパティを検索
        // この処理は簡略化されており、実際にはより詳細なFeature解析が必要

        // ここでは基本的な識別のみ実装
        for attr in element.attributes() {
            let attr = attr
                .map_err(|e| LandXMLError::ParseError(format!("Attribute parsing error: {}", e)))?;
            let key = std::str::from_utf8(attr.key.as_ref())
                .map_err(|e| LandXMLError::ParseError(format!("UTF-8 conversion error: {}", e)))?;
            let value = std::str::from_utf8(&attr.value)
                .map_err(|e| LandXMLError::ParseError(format!("UTF-8 conversion error: {}", e)))?;

            match key {
                "applicationCriterion" => {
                    doc.application_criterion = Some(value.to_string());
                    // Ver.1.6の判定
                    if value.contains("Ver1.6") || value.contains("1.6") {
                        doc.j_landxml_version = Some("1.6".to_string());
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }

    /// J-LandXMLとしての妥当性をチェック
    pub fn validate_j_landxml(&self, doc: &JLandXmlDocument) -> Result<bool, LandXMLError> {
        // J-LandXMLの基本要件をチェック

        // 1. 座標系チェック
        if let Some(ref cs) = doc.coordinate_system {
            let horizontal_name = &cs.horizontal_coordinate_system_name;
            if !CoordinateSystemNameParser::validate(horizontal_name) {
                return Ok(false);
            }
        }

        // 2. Ver.1.6識別子チェック
        if doc.j_landxml_version.is_some() || doc.application_criterion.is_some() {
            return Ok(true);
        }

        // 3. J-LandXML拡張属性の存在チェック
        Ok(doc.is_j_landxml())
    }

    /// パース統計情報を取得
    pub fn get_parsing_stats(&self, doc: &JLandXmlDocument) -> ParsingStats {
        ParsingStats {
            is_j_landxml: doc.is_j_landxml(),
            j_landxml_version: doc.j_landxml_version.clone(),
            plane_coordinate_zone: doc.get_plane_coordinate_zone(),
            epsg_code: doc.get_epsg_code(),
            surface_count: doc.base.surfaces.len(),
            alignment_count: doc.base.alignments.len(),
        }
    }
}

/// パース統計情報
#[derive(Debug, Clone)]
pub struct ParsingStats {
    /// J-LandXMLかどうか
    pub is_j_landxml: bool,
    /// J-LandXMLバージョン
    pub j_landxml_version: Option<String>,
    /// 平面直角座標系
    pub plane_coordinate_zone: Option<JapanPlaneCoordinateSystem>,
    /// EPSGコード
    pub epsg_code: Option<u32>,
    /// サーフェス数
    pub surface_count: usize,
    /// アライメント数
    pub alignment_count: usize,
}

impl std::fmt::Display for ParsingStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "=== J-LandXML Parsing Statistics ===")?;
        writeln!(
            f,
            "J-LandXML: {}",
            if self.is_j_landxml { "Yes" } else { "No" }
        )?;

        if let Some(ref version) = self.j_landxml_version {
            writeln!(f, "Version: {}", version)?;
        }

        if let Some(zone) = self.plane_coordinate_zone {
            writeln!(f, "Coordinate System: {}", zone)?;
        }

        if let Some(epsg) = self.epsg_code {
            writeln!(f, "EPSG Code: {}", epsg)?;
        }

        writeln!(f, "Surfaces: {}", self.surface_count)?;
        writeln!(f, "Alignments: {}", self.alignment_count)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coordinate_system_name_parser() {
        // 正常なケース
        assert!(CoordinateSystemNameParser::validate("1(X,Y)"));
        assert!(CoordinateSystemNameParser::validate("9(X,Y)"));
        assert!(CoordinateSystemNameParser::validate("19(X,Y)"));

        // 不正なケース
        assert!(!CoordinateSystemNameParser::validate("20(X,Y)"));
        assert!(!CoordinateSystemNameParser::validate("invalid"));
        assert!(!CoordinateSystemNameParser::validate("1(X,Z)"));

        // パース結果の確認
        let result = CoordinateSystemNameParser::parse("9(X,Y)").unwrap();
        assert_eq!(result, Some(JapanPlaneCoordinateSystem::Zone9));
    }

    #[test]
    fn test_coordinate_system_info() {
        let info = CoordinateSystemNameParser::get_info("9(X,Y)").unwrap();
        assert!(info.is_some());
        assert!(info.unwrap().contains("平面直角座標系9系"));

        let invalid_info = CoordinateSystemNameParser::get_info("invalid").unwrap();
        assert!(invalid_info.is_none());
    }

    #[test]
    fn test_jlandxml_property_creation() {
        use crate::jlandxml::models::JLandXmlProperty;

        let prop = JLandXmlProperty::new("testLabel", "testValue");
        assert_eq!(prop.label, "testLabel");
        assert_eq!(prop.value, "testValue");

        let project_prop = JLandXmlProperty::project_phase("詳細設計");
        assert_eq!(project_prop.label, "projectPhase");
        assert_eq!(project_prop.value, "詳細設計");
    }
}
