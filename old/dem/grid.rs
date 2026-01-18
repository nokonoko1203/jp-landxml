use crate::error::LandXMLError;
use serde::{Serialize, Deserialize};

/// DEM格子データを表現する構造体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DemGrid {
    /// 行数（Y方向）
    pub rows: usize,
    /// 列数（X方向）
    pub cols: usize,
    /// 左上角のX座標
    pub origin_x: f64,
    /// 左上角のY座標  
    pub origin_y: f64,
    /// X方向の解像度（メートル）
    pub x_resolution: f64,
    /// Y方向の解像度（メートル）
    pub y_resolution: f64,
    /// 標高値配列（行優先、NoData = -9999.0）
    pub values: Vec<f32>,
    /// 座標系EPSG代码（オプション）
    pub epsg_code: Option<u32>,
    /// データの境界範囲
    pub bounds: GridBounds,
}

/// グリッドの境界範囲
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct GridBounds {
    pub min_x: f64,
    pub max_x: f64,
    pub min_y: f64,
    pub max_y: f64,
    pub min_z: f32,
    pub max_z: f32,
}

impl DemGrid {
    /// 新しいDEMグリッドを作成
    pub fn new(
        rows: usize, 
        cols: usize, 
        origin_x: f64, 
        origin_y: f64,
        x_resolution: f64,
        y_resolution: f64,
        bounds: GridBounds,
    ) -> Self {
        let values = vec![-9999.0; rows * cols]; // NoDataで初期化
        
        Self {
            rows,
            cols,
            origin_x,
            origin_y,
            x_resolution,
            y_resolution,
            values,
            epsg_code: None,
            bounds,
        }
    }

    /// 指定された行・列位置の標高値を取得
    pub fn get_value(&self, row: usize, col: usize) -> Option<f32> {
        if row < self.rows && col < self.cols {
            let value = self.values[row * self.cols + col];
            if value != -9999.0 {
                Some(value)
            } else {
                None
            }
        } else {
            None
        }
    }

    /// 指定された行・列位置に標高値を設定
    pub fn set_value(&mut self, row: usize, col: usize, value: f32) -> Result<(), LandXMLError> {
        if row >= self.rows || col >= self.cols {
            return Err(LandXMLError::InvalidGridIndex { row, col, max_row: self.rows, max_col: self.cols });
        }
        
        self.values[row * self.cols + col] = value;
        Ok(())
    }

    /// 地理座標(x, y)から対応するグリッド座標(row, col)を計算
    pub fn world_to_grid(&self, x: f64, y: f64) -> Option<(usize, usize)> {
        let col = ((x - self.origin_x) / self.x_resolution).floor() as i32;
        let row = ((self.origin_y - y) / self.y_resolution).floor() as i32;
        
        if row >= 0 && row < self.rows as i32 && col >= 0 && col < self.cols as i32 {
            Some((row as usize, col as usize))
        } else {
            None
        }
    }

    /// グリッド座標(row, col)から地理座標(x, y)を計算（グリッド中心点）
    pub fn grid_to_world(&self, row: usize, col: usize) -> Option<(f64, f64)> {
        if row < self.rows && col < self.cols {
            let x = self.origin_x + (col as f64 + 0.5) * self.x_resolution;
            let y = self.origin_y - (row as f64 + 0.5) * self.y_resolution;
            Some((x, y))
        } else {
            None
        }
    }

    /// GDAL用のGeoTransform配列を生成
    /// GeoTIFFではピクセルの左上角が基準となるため、適切な座標変換を行う
    /// 
    /// GeoTransform配列の意味:
    /// [0] = 左上ピクセルの左上角のX座標
    /// [1] = X方向のピクセルサイズ（東向きが正）
    /// [2] = X方向の回転/スキュー（通常0）
    /// [3] = 左上ピクセルの左上角のY座標
    /// [4] = Y方向の回転/スキュー（通常0）
    /// [5] = Y方向のピクセルサイズ（南向きが負）
    pub fn geo_transform(&self) -> [f64; 6] {
        [
            self.origin_x,                    // 左上角X座標（ピクセル左上角基準）
            self.x_resolution,                // X方向ピクセルサイズ
            0.0,                             // 回転（通常0）
            self.origin_y,                    // 左上角Y座標（ピクセル左上角基準）
            0.0,                             // 回転（通常0）
            -self.y_resolution,               // Y方向ピクセルサイズ（負の値）
        ]
    }

    /// データの有効性をチェック
    pub fn validate(&self) -> Result<(), LandXMLError> {
        if self.values.len() != self.rows * self.cols {
            return Err(LandXMLError::InvalidGridSize { 
                expected: self.rows * self.cols,
                actual: self.values.len()
            });
        }
        
        if self.x_resolution <= 0.0 || self.y_resolution <= 0.0 {
            return Err(LandXMLError::InvalidResolution { 
                x_res: self.x_resolution,
                y_res: self.y_resolution
            });
        }
        
        Ok(())
    }

    /// NoDataではない有効な値の統計を取得
    pub fn statistics(&self) -> GridStatistics {
        let valid_values: Vec<f32> = self.values.iter()
            .filter(|&&v| v != -9999.0)
            .copied()
            .collect();
        
        if valid_values.is_empty() {
            return GridStatistics::default();
        }
        
        let min = valid_values.iter().fold(f32::INFINITY, |a, &b| a.min(b));
        let max = valid_values.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
        let mean = valid_values.iter().sum::<f32>() / valid_values.len() as f32;
        
        GridStatistics {
            valid_count: valid_values.len(),
            total_count: self.values.len(),
            min_value: min,
            max_value: max,
            mean_value: mean,
        }
    }
}

impl GridBounds {
    /// 境界範囲を作成
    pub fn new(min_x: f64, max_x: f64, min_y: f64, max_y: f64, min_z: f32, max_z: f32) -> Self {
        Self { min_x, max_x, min_y, max_y, min_z, max_z }
    }

    /// 幅を取得
    pub fn width(&self) -> f64 {
        self.max_x - self.min_x
    }

    /// 高さを取得
    pub fn height(&self) -> f64 {
        self.max_y - self.min_y
    }

    /// 標高差を取得
    pub fn elevation_range(&self) -> f32 {
        self.max_z - self.min_z
    }

    /// 指定された解像度での必要グリッドサイズを計算
    pub fn grid_size(&self, resolution: f64) -> (usize, usize) {
        let cols = (self.width() / resolution).ceil() as usize;
        let rows = (self.height() / resolution).ceil() as usize;
        (rows, cols)
    }
}

/// グリッドの統計情報
#[derive(Debug, Clone, Default)]
pub struct GridStatistics {
    pub valid_count: usize,
    pub total_count: usize,
    pub min_value: f32,
    pub max_value: f32,
    pub mean_value: f32,
}

impl GridStatistics {
    /// 有効データの割合を取得
    pub fn valid_ratio(&self) -> f64 {
        if self.total_count == 0 {
            0.0
        } else {
            self.valid_count as f64 / self.total_count as f64
        }
    }
}