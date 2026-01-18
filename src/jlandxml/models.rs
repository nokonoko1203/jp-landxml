use super::coordinate_systems::JapanPlaneCoordinateSystem;
use crate::models::{CoordinateSystem, LandXML};
/// J-LandXML Ver.1.6 拡張データモデル
///
/// 標準LandXMLのデータ構造を拡張して、J-LandXML特有の機能をサポートします。
use serde::{Deserialize, Serialize};

/// J-LandXML Ver.1.6 完全対応 座標系情報
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JLandXmlCoordinateSystem {
    /// 基本属性
    pub name: String,
    pub desc: Option<String>,

    /// 測地・座標系定義（J-LandXML完全仕様）
    pub horizontal_datum: super::coordinate_systems::HorizontalDatum,
    pub vertical_datum: super::coordinate_systems::VerticalDatum,
    pub horizontal_coordinate_system_name: String,

    /// 高さ系補正（T.P基準との差分、メートル）
    pub differ_tp: Option<f64>,

    /// 解析済み情報
    pub plane_coordinate_zone: Option<JapanPlaneCoordinateSystem>,

    /// 標準LandXML互換性
    pub base: Option<CoordinateSystem>,
    pub epsg_code: Option<String>,
    pub proj4_string: Option<String>,

    /// 追加属性（J-LandXML拡張）
    pub geoid_name: Option<String>,
}

impl JLandXmlCoordinateSystem {
    /// 完全仕様に基づいた新規作成
    pub fn new(
        name: String,
        horizontal_datum: super::coordinate_systems::HorizontalDatum,
        vertical_datum: super::coordinate_systems::VerticalDatum,
        horizontal_coordinate_system_name: String,
    ) -> Result<Self, crate::error::LandXMLError> {
        // 平面直角座標系を自動解析
        let plane_coordinate_zone = super::coordinate_systems::CoordinateSystemMapper::parse_horizontal_coordinate_system_name(&horizontal_coordinate_system_name)?;

        // differTPを自動設定
        let differ_tp = if matches!(vertical_datum, super::coordinate_systems::VerticalDatum::TP) {
            None // T.P基準は補正不要
        } else {
            Some(vertical_datum.tp_offset())
        };

        Ok(Self {
            name,
            desc: None,
            horizontal_datum,
            vertical_datum,
            horizontal_coordinate_system_name,
            differ_tp,
            plane_coordinate_zone,
            base: None,
            epsg_code: plane_coordinate_zone.map(|zone| zone.epsg_code().to_string()),
            proj4_string: None,
            geoid_name: None,
        })
    }

    /// 標準LandXMLの座標系から作成（後方互換性）
    pub fn from_base(base: CoordinateSystem) -> Self {
        Self {
            name: base.name.clone(),
            desc: None,
            horizontal_datum: super::coordinate_systems::HorizontalDatum::JGD2011, // デフォルト
            vertical_datum: super::coordinate_systems::VerticalDatum::TP,          // デフォルト
            horizontal_coordinate_system_name: "9(X,Y)".to_string(), // デフォルト（東京都）
            differ_tp: None,
            plane_coordinate_zone: Some(JapanPlaneCoordinateSystem::Zone9),
            base: Some(base.clone()),
            epsg_code: base.epsg_code,
            proj4_string: base.proj4_string,
            geoid_name: None,
        }
    }

    /// J-LandXML拡張属性を設定
    pub fn with_horizontal_coordinate_system_name(mut self, name: String) -> Self {
        // 座標系名から平面直角座標系を自動解析
        if let Ok(Some(zone)) = super::coordinate_systems::CoordinateSystemMapper::parse_horizontal_coordinate_system_name(&name) {
            self.plane_coordinate_zone = Some(zone);
            self.epsg_code = Some(zone.epsg_code().to_string());
        }
        self.horizontal_coordinate_system_name = name;
        self
    }

    /// 鉛直原子を設定（differTPも自動更新）
    pub fn with_vertical_datum(
        mut self,
        vertical_datum: super::coordinate_systems::VerticalDatum,
    ) -> Self {
        self.vertical_datum = vertical_datum;
        self.differ_tp = if matches!(vertical_datum, super::coordinate_systems::VerticalDatum::TP) {
            None
        } else {
            Some(vertical_datum.tp_offset())
        };
        self
    }

    /// differTPを手動設定（カスタム値用）
    pub fn with_differ_tp(mut self, differ_tp: f64) -> Self {
        self.differ_tp = Some(differ_tp);
        self
    }

    /// ジオイドモデル名を設定
    pub fn with_geoid_name(mut self, name: String) -> Self {
        self.geoid_name = Some(name);
        self
    }

    /// 説明を設定
    pub fn with_description(mut self, desc: String) -> Self {
        self.desc = Some(desc);
        self
    }

    /// 平面直角座標系のEPSGコードを取得
    pub fn get_plane_coordinate_epsg(&self) -> Option<u32> {
        self.plane_coordinate_zone.map(|zone| zone.epsg_code())
    }

    /// T.P基準への標高変換
    pub fn to_tp_elevation(&self, raw_elevation: f64) -> f64 {
        match self.differ_tp {
            Some(diff) => raw_elevation + diff,
            None => raw_elevation, // T.P基準または未設定
        }
    }

    /// 座標系の妥当性をバリデーション
    pub fn validate(
        &self,
    ) -> Result<Vec<super::coordinate_systems::ValidationWarning>, crate::error::LandXMLError> {
        if let Some(zone) = self.plane_coordinate_zone {
            super::coordinate_systems::CoordinateSystemValidator::validate_complete_system(
                self.horizontal_datum,
                self.vertical_datum,
                zone,
                self.differ_tp,
            )
        } else {
            Err(
                crate::error::LandXMLError::InvalidCoordinateSystemNameFormat {
                    name: self.horizontal_coordinate_system_name.clone(),
                },
            )
        }
    }

    /// 座標系の詳細情報を取得
    pub fn get_coordinate_system_info(&self) -> CoordinateSystemInfo {
        CoordinateSystemInfo {
            name: self.name.clone(),
            desc: self.desc.clone(),
            horizontal_datum: self.horizontal_datum,
            vertical_datum: self.vertical_datum,
            horizontal_coordinate_system_name: self.horizontal_coordinate_system_name.clone(),
            differ_tp: self.differ_tp,
            plane_coordinate_zone: self.plane_coordinate_zone,
            epsg_code: self.get_plane_coordinate_epsg(),
            geoid_name: self.geoid_name.clone(),
        }
    }
}

/// 座標系情報の統合ビュー（完全仕様対応）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoordinateSystemInfo {
    /// 座標系名
    pub name: String,
    /// 説明
    pub desc: Option<String>,
    /// 水平測地原子
    pub horizontal_datum: super::coordinate_systems::HorizontalDatum,
    /// 鉛直原子
    pub vertical_datum: super::coordinate_systems::VerticalDatum,
    /// 水平座標系名（J-LandXML形式）
    pub horizontal_coordinate_system_name: String,
    /// T.P基準との差分（メートル）
    pub differ_tp: Option<f64>,
    /// 平面直角座標系（解析済み）
    pub plane_coordinate_zone: Option<JapanPlaneCoordinateSystem>,
    /// EPSGコード（数値、解析済み）
    pub epsg_code: Option<u32>,
    /// ジオイドモデル名（J-LandXML拡張）
    pub geoid_name: Option<String>,
}

/// J-LandXML拡張文書
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JLandXmlDocument {
    /// 標準LandXMLデータ
    pub base: LandXML,

    /// J-LandXML拡張座標系
    pub coordinate_system: Option<JLandXmlCoordinateSystem>,

    /// J-LandXML Ver.1.6識別情報
    pub j_landxml_version: Option<String>,

    /// アプリケーション基準（applicationCriterion）
    pub application_criterion: Option<String>,
}

impl JLandXmlDocument {
    /// 標準LandXMLから作成
    pub fn from_base(base: LandXML) -> Self {
        // 標準座標系をJ-LandXML拡張座標系に変換
        let coordinate_system = base
            .coordinate_system
            .as_ref()
            .map(|cs| JLandXmlCoordinateSystem::from_base(cs.clone()));

        Self {
            base,
            coordinate_system,
            j_landxml_version: None,
            application_criterion: None,
        }
    }

    /// J-LandXML Ver.1.6として識別
    pub fn with_j_landxml_version(mut self, version: String) -> Self {
        self.j_landxml_version = Some(version);
        self
    }

    /// アプリケーション基準を設定
    pub fn with_application_criterion(mut self, criterion: String) -> Self {
        self.application_criterion = Some(criterion);
        self
    }

    /// 座標系情報を設定
    pub fn with_coordinate_system(mut self, coordinate_system: JLandXmlCoordinateSystem) -> Self {
        self.coordinate_system = Some(coordinate_system);
        self
    }

    /// J-LandXML文書かどうかを判定
    pub fn is_j_landxml(&self) -> bool {
        self.j_landxml_version.is_some()
            || self.application_criterion.is_some()
            || self
                .coordinate_system
                .as_ref()
                .map(|cs| !cs.horizontal_coordinate_system_name.is_empty())
                .unwrap_or(false)
    }

    /// 使用されている平面直角座標系を取得
    pub fn get_plane_coordinate_zone(&self) -> Option<JapanPlaneCoordinateSystem> {
        self.coordinate_system
            .as_ref()
            .and_then(|cs| cs.plane_coordinate_zone)
    }

    /// 座標系のEPSGコードを取得
    pub fn get_epsg_code(&self) -> Option<u32> {
        self.coordinate_system
            .as_ref()
            .and_then(|cs| cs.get_plane_coordinate_epsg())
    }
}

/// J-LandXML拡張Featureプロパティのためのヘルパー
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JLandXmlProperty {
    /// プロパティラベル
    pub label: String,
    /// プロパティ値
    pub value: String,
}

impl JLandXmlProperty {
    /// 新しいプロパティを作成
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
        }
    }

    /// プロジェクトフェーズプロパティ
    pub fn project_phase(phase: impl Into<String>) -> Self {
        Self::new("projectPhase", phase)
    }

    /// アプリケーション基準プロパティ
    pub fn application_criterion(criterion: impl Into<String>) -> Self {
        Self::new("applicationCriterion", criterion)
    }

    /// 道路分類プロパティ
    pub fn road_classification(classification: impl Into<String>) -> Self {
        Self::new("classification", classification)
    }

    /// 交通量プロパティ
    pub fn traffic_volume(volume: u32) -> Self {
        Self::new("trafficVolume", volume.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::CoordinateSystem;

    #[test]
    fn test_jlandxml_coordinate_system_creation() {
        let base_cs = CoordinateSystem {
            name: "JGD2011 / Zone 9".to_string(),
            epsg_code: Some("6677".to_string()),
            proj4_string: None,
        };

        let jlandxml_cs = JLandXmlCoordinateSystem::from_base(base_cs)
            .with_horizontal_coordinate_system_name("9(X,Y)".to_string());

        assert_eq!(
            jlandxml_cs.horizontal_coordinate_system_name,
            "9(X,Y)".to_string()
        );
        assert_eq!(
            jlandxml_cs.plane_coordinate_zone,
            Some(JapanPlaneCoordinateSystem::Zone9)
        );
        assert_eq!(jlandxml_cs.get_plane_coordinate_epsg(), Some(6677));
    }

    #[test]
    fn test_jlandxml_document_creation() {
        let base_landxml = LandXML {
            version: "1.2".to_string(),
            coordinate_system: None,
            surfaces: Vec::new(),
            alignments: Vec::new(),
            features: Vec::new(),
        };

        let jlandxml_doc = JLandXmlDocument::from_base(base_landxml)
            .with_j_landxml_version("1.6".to_string())
            .with_application_criterion("MlitLandXmlVer1.6".to_string());

        assert!(jlandxml_doc.is_j_landxml());
        assert_eq!(jlandxml_doc.j_landxml_version, Some("1.6".to_string()));
    }

    #[test]
    fn test_jlandxml_properties() {
        let prop = JLandXmlProperty::project_phase("詳細設計");
        assert_eq!(prop.label, "projectPhase");
        assert_eq!(prop.value, "詳細設計");

        let traffic_prop = JLandXmlProperty::traffic_volume(12000);
        assert_eq!(traffic_prop.label, "trafficVolume");
        assert_eq!(traffic_prop.value, "12000");
    }
}
