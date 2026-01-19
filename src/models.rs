//! Common models for LandXML parser
//!
//! This module contains shared data structures used across the library.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::coordinate_system::CoordinateSystem;

/// LandXMLドキュメントのルート構造体
/// 今後Alignment中心に拡張予定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LandXML {
    pub version: String,
    pub coordinate_system: Option<CoordinateSystem>,
    pub alignments: Vec<Alignment>,
    pub features: Vec<Feature>,
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
/// CoordGeom, Profile, CrossSectionsの詳細はalignmentsモジュールで定義
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alignment {
    pub name: String,
    pub desc: Option<String>,
    pub sta_start: Option<f64>,
}

/// 汎用Feature要素（プレースホルダー）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feature {
    pub code: String,
    pub properties: HashMap<String, String>,
}
