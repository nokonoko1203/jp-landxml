use crate::error::LandXMLError;
/// J-LandXML Ver.1.6 完全対応 座標系定義・変換モジュール
///
/// 日本の測量法に基づく座標系の完全実装：
/// - 水平測地原子（JGD2000/JGD2011/TD）
/// - 鉛直原子（T.P/K.P/S.P/Y.P/A.P/O.P/T.P.W/B.S.L）
/// - 平面直角座標系（1系～19系）
/// - T.P基準への高さ系変換（differTP）
use serde::{Deserialize, Serialize};
use std::fmt;

/// 日本の平面直角座標系（1系～19系）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum JapanPlaneCoordinateSystem {
    /// 1系 - 長崎県、鹿児島県の一部（東経129°30'、北緯33°）
    Zone1,
    /// 2系 - 福岡、佐賀、熊本、大分、宮崎県（東経131°、北緯33°）
    Zone2,
    /// 3系 - 山口、島根、広島県（東経132°10'、北緯34°20'）
    Zone3,
    /// 4系 - 香川、愛媛、徳島、高知県（東経133°30'、北緯33°）
    Zone4,
    /// 5系 - 兵庫、鳥取、岡山県（東経134°20'、北緯34°40'）
    Zone5,
    /// 6系 - 京都、大阪、福井、滋賀、三重、奈良、和歌山県（東経136°、北緯36°）
    Zone6,
    /// 7系 - 石川、富山、岐阜、愛知県（東経137°10'、北緯36°）
    Zone7,
    /// 8系 - 新潟、長野、山梨、静岡県（東経138°30'、北緯36°）
    Zone8,
    /// 9系 - 東京都、福島、栃木、茨城、埼玉、千葉、群馬、神奈川県（東経139°50'、北緯36°）
    Zone9,
    /// 10系 - 青森、秋田、山形、岩手、宮城県（東経140°50'、北緯40°）
    Zone10,
    /// 11系 - 小笠原諸島（東経142°15'、北緯26°）
    Zone11,
    /// 12系 - 北海道西部（東経142°15'、北緯44°）
    Zone12,
    /// 13系 - 北海道中央部（東経144°15'、北緯44°）
    Zone13,
    /// 14系 - 北海道東部（東経142°、北緯26°）
    Zone14,
    /// 15系 - 沖縄県（東経127°30'、北緯26°）
    Zone15,
    /// 16系 - 沖縄県（東経124°、北緯26°）
    Zone16,
    /// 17系 - 沖縄県（東経131°、北緯26°）
    Zone17,
    /// 18系 - 沖縄県（東経136°、北緯20°）
    Zone18,
    /// 19系 - 南鳥島、沖ノ鳥島（東経154°、北緯26°）
    Zone19,
}

impl JapanPlaneCoordinateSystem {
    /// 対応するEPSGコードを取得（JGD2011基準）
    pub fn epsg_code(&self) -> u32 {
        match self {
            Self::Zone1 => 6669,
            Self::Zone2 => 6670,
            Self::Zone3 => 6671,
            Self::Zone4 => 6672,
            Self::Zone5 => 6673,
            Self::Zone6 => 6674,
            Self::Zone7 => 6675,
            Self::Zone8 => 6676,
            Self::Zone9 => 6677,
            Self::Zone10 => 6678,
            Self::Zone11 => 6679,
            Self::Zone12 => 6680,
            Self::Zone13 => 6681,
            Self::Zone14 => 6682,
            Self::Zone15 => 6683,
            Self::Zone16 => 6684,
            Self::Zone17 => 6685,
            Self::Zone18 => 6686,
            Self::Zone19 => 6687,
        }
    }

    /// 系の番号を取得（1～19）
    pub fn zone_number(&self) -> u8 {
        match self {
            Self::Zone1 => 1,
            Self::Zone2 => 2,
            Self::Zone3 => 3,
            Self::Zone4 => 4,
            Self::Zone5 => 5,
            Self::Zone6 => 6,
            Self::Zone7 => 7,
            Self::Zone8 => 8,
            Self::Zone9 => 9,
            Self::Zone10 => 10,
            Self::Zone11 => 11,
            Self::Zone12 => 12,
            Self::Zone13 => 13,
            Self::Zone14 => 14,
            Self::Zone15 => 15,
            Self::Zone16 => 16,
            Self::Zone17 => 17,
            Self::Zone18 => 18,
            Self::Zone19 => 19,
        }
    }

    /// 適用地域の説明を取得
    pub fn description(&self) -> &'static str {
        match self {
            Self::Zone1 => "長崎県、鹿児島県の一部",
            Self::Zone2 => "福岡、佐賀、熊本、大分、宮崎県",
            Self::Zone3 => "山口、島根、広島県",
            Self::Zone4 => "香川、愛媛、徳島、高知県",
            Self::Zone5 => "兵庫、鳥取、岡山県",
            Self::Zone6 => "京都、大阪、福井、滋賀、三重、奈良、和歌山県",
            Self::Zone7 => "石川、富山、岐阜、愛知県",
            Self::Zone8 => "新潟、長野、山梨、静岡県",
            Self::Zone9 => "東京都、福島、栃木、茨城、埼玉、千葉、群馬、神奈川県",
            Self::Zone10 => "青森、秋田、山形、岩手、宮城県",
            Self::Zone11 => "小笠原諸島",
            Self::Zone12 => "北海道西部",
            Self::Zone13 => "北海道中央部",
            Self::Zone14 => "北海道東部",
            Self::Zone15 => "沖縄県（本島周辺）",
            Self::Zone16 => "沖縄県（宮古島周辺）",
            Self::Zone17 => "沖縄県（石垣島周辺）",
            Self::Zone18 => "沖縄県（与那国島周辺）",
            Self::Zone19 => "南鳥島、沖ノ鳥島",
        }
    }

    /// 系番号から平面直角座標系を取得
    pub fn from_zone_number(zone: u8) -> Result<Self, LandXMLError> {
        match zone {
            1 => Ok(Self::Zone1),
            2 => Ok(Self::Zone2),
            3 => Ok(Self::Zone3),
            4 => Ok(Self::Zone4),
            5 => Ok(Self::Zone5),
            6 => Ok(Self::Zone6),
            7 => Ok(Self::Zone7),
            8 => Ok(Self::Zone8),
            9 => Ok(Self::Zone9),
            10 => Ok(Self::Zone10),
            11 => Ok(Self::Zone11),
            12 => Ok(Self::Zone12),
            13 => Ok(Self::Zone13),
            14 => Ok(Self::Zone14),
            15 => Ok(Self::Zone15),
            16 => Ok(Self::Zone16),
            17 => Ok(Self::Zone17),
            18 => Ok(Self::Zone18),
            19 => Ok(Self::Zone19),
            _ => Err(LandXMLError::InvalidCoordinateSystem(format!(
                "Unsupported plane coordinate zone: {}",
                zone
            ))),
        }
    }

    /// すべての平面直角座標系を列挙
    pub fn all_zones() -> Vec<Self> {
        vec![
            Self::Zone1,
            Self::Zone2,
            Self::Zone3,
            Self::Zone4,
            Self::Zone5,
            Self::Zone6,
            Self::Zone7,
            Self::Zone8,
            Self::Zone9,
            Self::Zone10,
            Self::Zone11,
            Self::Zone12,
            Self::Zone13,
            Self::Zone14,
            Self::Zone15,
            Self::Zone16,
            Self::Zone17,
            Self::Zone18,
            Self::Zone19,
        ]
    }
}

impl fmt::Display for JapanPlaneCoordinateSystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "平面直角座標系{}系 (EPSG:{})",
            self.zone_number(),
            self.epsg_code()
        )
    }
}

/// 水平測地原子（測地基準系）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HorizontalDatum {
    /// 日本測地系2000（GRS80楕円体）
    JGD2000,
    /// 日本測地系2011（東日本大震災後の再測量対応）
    JGD2011,
    /// 旧日本測地系（Tokyo Datum、Bessel楕円体）
    TD,
}

impl HorizontalDatum {
    /// 測地原子の文字列表現を取得
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::JGD2000 => "JGD2000",
            Self::JGD2011 => "JGD2011",
            Self::TD => "TD",
        }
    }

    /// 文字列から測地原子を解析
    pub fn from_str(s: &str) -> Result<Self, LandXMLError> {
        match s.trim() {
            "JGD2000" => Ok(Self::JGD2000),
            "JGD2011" => Ok(Self::JGD2011),
            "TD" => Ok(Self::TD),
            _ => Err(LandXMLError::InvalidCoordinateSystem(format!(
                "Unsupported horizontal datum: {}",
                s
            ))),
        }
    }

    /// 説明を取得
    pub fn description(&self) -> &'static str {
        match self {
            Self::JGD2000 => "日本測地系2000（GRS80楕円体）",
            Self::JGD2011 => "日本測地系2011（東日本大震災後対応）",
            Self::TD => "旧日本測地系（Tokyo Datum、Bessel楕円体）",
        }
    }
}

/// 鉛直原子（高さ基準系）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VerticalDatum {
    /// 東京湾平均海面（基準）
    TP,
    /// 北上川基準点
    KP,
    /// 鳴瀬川基準点  
    SP,
    /// 利根川基準点
    YP,
    /// 荒川基準点（関東）/ 吉野川基準点（四国）
    AP,
    /// 淀川基準点（大阪）
    OP,
    /// 渡川基準点
    TPW,
    /// 琵琶湖水準面
    BSL,
}

impl VerticalDatum {
    /// 鉛直原子の文字列表現を取得
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::TP => "T.P",
            Self::KP => "K.P",
            Self::SP => "S.P",
            Self::YP => "Y.P",
            Self::AP => "A.P",
            Self::OP => "O.P",
            Self::TPW => "T.P.W",
            Self::BSL => "B.S.L",
        }
    }

    /// 文字列から鉛直原子を解析
    pub fn from_str(s: &str) -> Result<Self, LandXMLError> {
        match s.trim() {
            "T.P" => Ok(Self::TP),
            "K.P" => Ok(Self::KP),
            "S.P" => Ok(Self::SP),
            "Y.P" => Ok(Self::YP),
            "A.P" => Ok(Self::AP),
            "O.P" => Ok(Self::OP),
            "T.P.W" => Ok(Self::TPW),
            "B.S.L" => Ok(Self::BSL),
            _ => Err(LandXMLError::InvalidCoordinateSystem(format!(
                "Unsupported vertical datum: {}",
                s
            ))),
        }
    }

    /// T.P基準からの差分（メートル）を取得
    pub fn tp_offset(&self) -> f64 {
        match self {
            Self::TP => 0.0,
            Self::KP => -0.8745,
            Self::SP => -0.0873,
            Self::YP => -0.8402,
            Self::AP => -1.1344, // 関東のA.P（吉野川A.Pは-0.8333だが実装では関東を標準とする）
            Self::OP => -1.3000,
            Self::TPW => 0.113,
            Self::BSL => 84.371,
        }
    }

    /// 対象河川・水域の説明を取得
    pub fn description(&self) -> &'static str {
        match self {
            Self::TP => "東京湾平均海面（Tokyo Peil）",
            Self::KP => "北上川（Kitakami Peil）",
            Self::SP => "鳴瀬川（Same Peil）",
            Self::YP => "利根川（Tone Peil）",
            Self::AP => "荒川・中川・多摩川（Arakawa Peil）",
            Self::OP => "淀川（Osaka Peil）",
            Self::TPW => "渡川（Tosa Peil Watarigawa）",
            Self::BSL => "琵琶湖（Biwa Surface Level）",
        }
    }

    /// T.P基準への標高変換
    pub fn to_tp_elevation(&self, raw_elevation: f64) -> f64 {
        raw_elevation + self.tp_offset()
    }
}

impl fmt::Display for HorizontalDatum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.as_str(), self.description())
    }
}

impl fmt::Display for VerticalDatum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {}", self.as_str(), self.description())
    }
}

/// 座標系マッピング・バリデーション機能
pub struct CoordinateSystemMapper;

impl CoordinateSystemMapper {
    /// J-LandXMLの座標系名から適切なEPSGコードを取得
    pub fn get_epsg_from_jlandxml_name(name: &str) -> Option<u32> {
        // "1(X,Y)" 形式から系番号を抽出
        if let Ok(Some(zone)) = Self::parse_horizontal_coordinate_system_name(name) {
            Some(zone.epsg_code())
        } else {
            None
        }
    }

    /// horizontalCoordinateSystemName をパースして平面直角座標系を取得
    pub fn parse_horizontal_coordinate_system_name(
        name: &str,
    ) -> Result<Option<JapanPlaneCoordinateSystem>, LandXMLError> {
        use regex::Regex;

        // "N(X,Y)" 形式の正規表現（Nは1～19の数字）
        let re = Regex::new(r"^(\d{1,2})\(X,Y\)$")
            .map_err(|e| LandXMLError::ParseError(format!("Regex compilation failed: {}", e)))?;

        if let Some(captures) = re.captures(name.trim()) {
            if let Some(zone_str) = captures.get(1) {
                let zone_num: u8 = zone_str
                    .as_str()
                    .parse()
                    .map_err(|e| LandXMLError::ParseError(format!("Invalid zone number: {}", e)))?;

                // from_zone_numberでエラーが発生した場合（範囲外の系番号）はNoneを返す
                match JapanPlaneCoordinateSystem::from_zone_number(zone_num) {
                    Ok(zone) => return Ok(Some(zone)),
                    Err(_) => return Ok(None), // 無効な系番号（1-19範囲外）
                }
            }
        }

        // パターンにマッチしない場合はNone（エラーではない）
        Ok(None)
    }

    /// 測地原子と平面直角座標系の整合性をチェック
    pub fn validate_datum_compatibility(
        horizontal_datum: HorizontalDatum,
        _zone: JapanPlaneCoordinateSystem,
    ) -> Result<(), LandXMLError> {
        match horizontal_datum {
            HorizontalDatum::JGD2000 | HorizontalDatum::JGD2011 => {
                // JGD2000/JGD2011は全系で使用可能
                Ok(())
            }
            HorizontalDatum::TD => {
                // 旧日本測地系は互換性のために許可するが、警告レベル
                // 実際の運用では非推奨
                Ok(())
            }
        }
    }

    /// 高さ系補正が必要かどうかを判定
    pub fn needs_tp_correction(vertical_datum: VerticalDatum) -> bool {
        !matches!(vertical_datum, VerticalDatum::TP)
    }
}

/// 座標系の統合バリデーター
pub struct CoordinateSystemValidator;

impl CoordinateSystemValidator {
    /// 完全な座標系設定の妥当性をチェック
    pub fn validate_complete_system(
        horizontal_datum: HorizontalDatum,
        vertical_datum: VerticalDatum,
        zone: JapanPlaneCoordinateSystem,
        differ_tp: Option<f64>,
    ) -> Result<Vec<ValidationWarning>, LandXMLError> {
        let mut warnings = Vec::new();

        // 測地原子と座標系の整合性チェック
        CoordinateSystemMapper::validate_datum_compatibility(horizontal_datum, zone)?;

        // differTP の整合性チェック
        if CoordinateSystemMapper::needs_tp_correction(vertical_datum) {
            match differ_tp {
                Some(provided_diff) => {
                    let expected_diff = vertical_datum.tp_offset();
                    let tolerance = 0.001; // 1mm許容差

                    if (provided_diff - expected_diff).abs() > tolerance {
                        warnings.push(ValidationWarning::DifferTpMismatch {
                            vertical_datum,
                            provided: provided_diff,
                            expected: expected_diff,
                        });
                    }
                }
                None => {
                    warnings.push(ValidationWarning::MissingDifferTp {
                        vertical_datum,
                        expected: vertical_datum.tp_offset(),
                    });
                }
            }
        } else {
            // T.P基準の場合はdifferTPがあっても無視される
            if differ_tp.is_some() {
                warnings.push(ValidationWarning::UnnecessaryDifferTp);
            }
        }

        // 旧測地系の使用警告
        if matches!(horizontal_datum, HorizontalDatum::TD) {
            warnings.push(ValidationWarning::LegacyDatumUsage {
                datum: horizontal_datum,
            });
        }

        Ok(warnings)
    }
}

/// バリデーション警告
#[derive(Debug, Clone)]
pub enum ValidationWarning {
    /// differTPの値が期待値と一致しない
    DifferTpMismatch {
        vertical_datum: VerticalDatum,
        provided: f64,
        expected: f64,
    },
    /// T.P以外の鉛直原子でdifferTPが未設定
    MissingDifferTp {
        vertical_datum: VerticalDatum,
        expected: f64,
    },
    /// T.P基準でdifferTPが設定されている
    UnnecessaryDifferTp,
    /// 旧測地系の使用
    LegacyDatumUsage { datum: HorizontalDatum },
}

impl fmt::Display for ValidationWarning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationWarning::DifferTpMismatch {
                vertical_datum,
                provided,
                expected,
            } => {
                write!(
                    f,
                    "differTP mismatch for {}: provided {:.4}m, expected {:.4}m",
                    vertical_datum.as_str(),
                    provided,
                    expected
                )
            }
            ValidationWarning::MissingDifferTp {
                vertical_datum,
                expected,
            } => {
                write!(
                    f,
                    "Missing differTP for {}: should be {:.4}m",
                    vertical_datum.as_str(),
                    expected
                )
            }
            ValidationWarning::UnnecessaryDifferTp => {
                write!(f, "Unnecessary differTP for T.P datum")
            }
            ValidationWarning::LegacyDatumUsage { datum } => {
                write!(
                    f,
                    "Legacy datum usage: {} is deprecated, consider upgrading to JGD2011",
                    datum.as_str()
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zone_epsg_codes() {
        assert_eq!(JapanPlaneCoordinateSystem::Zone1.epsg_code(), 6669);
        assert_eq!(JapanPlaneCoordinateSystem::Zone9.epsg_code(), 6677);
        assert_eq!(JapanPlaneCoordinateSystem::Zone19.epsg_code(), 6687);
    }

    #[test]
    fn test_zone_numbers() {
        assert_eq!(JapanPlaneCoordinateSystem::Zone1.zone_number(), 1);
        assert_eq!(JapanPlaneCoordinateSystem::Zone9.zone_number(), 9);
        assert_eq!(JapanPlaneCoordinateSystem::Zone19.zone_number(), 19);
    }

    #[test]
    fn test_from_zone_number() {
        assert_eq!(
            JapanPlaneCoordinateSystem::from_zone_number(1).unwrap(),
            JapanPlaneCoordinateSystem::Zone1
        );
        assert_eq!(
            JapanPlaneCoordinateSystem::from_zone_number(9).unwrap(),
            JapanPlaneCoordinateSystem::Zone9
        );
        assert!(JapanPlaneCoordinateSystem::from_zone_number(20).is_err());
        assert!(JapanPlaneCoordinateSystem::from_zone_number(0).is_err());
    }

    #[test]
    fn test_coordinate_system_name_parsing() {
        // 正常なケース
        assert_eq!(
            CoordinateSystemMapper::parse_horizontal_coordinate_system_name("1(X,Y)").unwrap(),
            Some(JapanPlaneCoordinateSystem::Zone1)
        );
        assert_eq!(
            CoordinateSystemMapper::parse_horizontal_coordinate_system_name("9(X,Y)").unwrap(),
            Some(JapanPlaneCoordinateSystem::Zone9)
        );
        assert_eq!(
            CoordinateSystemMapper::parse_horizontal_coordinate_system_name("19(X,Y)").unwrap(),
            Some(JapanPlaneCoordinateSystem::Zone19)
        );

        // 不正なケース（エラーではなくNone）
        assert_eq!(
            CoordinateSystemMapper::parse_horizontal_coordinate_system_name("20(X,Y)").unwrap(),
            None
        );
        assert_eq!(
            CoordinateSystemMapper::parse_horizontal_coordinate_system_name("invalid").unwrap(),
            None
        );
        assert_eq!(
            CoordinateSystemMapper::parse_horizontal_coordinate_system_name("1(X,Z)").unwrap(),
            None
        );
    }

    #[test]
    fn test_epsg_from_jlandxml_name() {
        assert_eq!(
            CoordinateSystemMapper::get_epsg_from_jlandxml_name("1(X,Y)"),
            Some(6669)
        );
        assert_eq!(
            CoordinateSystemMapper::get_epsg_from_jlandxml_name("9(X,Y)"),
            Some(6677)
        );
        assert_eq!(
            CoordinateSystemMapper::get_epsg_from_jlandxml_name("invalid"),
            None
        );
    }
}
