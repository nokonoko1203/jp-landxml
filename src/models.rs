use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// LandXMLドキュメントのルート構造体
/// 今後Alignment中心に拡張予定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LandXML {
    pub version: String,
    pub coordinate_system: Option<CoordinateSystem>,
    pub alignments: Vec<Alignment>,
    pub features: Vec<Feature>,
}

/// 座標系情報（標準LandXML）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoordinateSystem {
    pub name: String,
    pub epsg_code: Option<String>,
    pub proj4_string: Option<String>,
}

/// 2次元座標点
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

/// 3次元座標点
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// 線形（Alignment）
/// CoordGeom, Profile, CrossSectionsの詳細はalignmentモジュールで定義
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alignment {
    pub name: String,
    pub desc: Option<String>,
    pub sta_start: Option<f64>,
    // coord_geom, profile, cross_sectionsは今後alignmentモジュールの型を使用
}

/// 汎用Feature要素（プレースホルダー）
/// 詳細はalignment/feature.rsで定義予定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feature {
    pub code: String,
    pub properties: HashMap<String, String>,
}
