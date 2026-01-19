//! Standard LandXML CoordinateSystem
//!
//! Basic coordinate system structure from LandXML 1.2 specification.

use serde::{Deserialize, Serialize};

/// 座標系情報（標準LandXML）
///
/// Reference: LandXML 1.2 CoordinateSystem element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoordinateSystem {
    /// 座標系名
    pub name: String,
    /// EPSGコード
    pub epsg_code: Option<String>,
    /// PROJ.4文字列
    pub proj4_string: Option<String>,
}
