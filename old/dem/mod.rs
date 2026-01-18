pub mod grid;
pub mod triangulation;
pub mod geotiff_writer;

pub use grid::{DemGrid, GridBounds};
pub use triangulation::TriangulationSource;
pub use geotiff_writer::{GeoTiffWriter, CompressionType};

use crate::error::LandXMLError;

/// DEMグリッド生成のための統一インターフェース
pub trait DemGridGenerator {
    /// 指定された解像度でDEMグリッドを生成
    fn generate_grid(&self, resolution: f64, bounds: Option<GridBounds>) -> Result<DemGrid, LandXMLError>;
}

/// 3次元ポイントクラウド/TINデータの抽象化
pub trait GeometrySource {
    /// 境界ボックスを取得
    fn bounds(&self) -> GridBounds;
    
    /// 指定された座標での標高値を補間取得
    fn interpolate_elevation(&self, x: f64, y: f64) -> Option<f32>;
    
    /// 全ての3Dポイントを取得（効率的な処理のため）
    fn points(&self) -> &[crate::models::Point3D];
    
    /// 三角形面を取得（TINの場合）
    fn triangles(&self) -> Option<&[crate::models::Triangle]>;
}