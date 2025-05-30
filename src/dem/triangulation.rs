use crate::models::{Point3D, Triangle, Surface};
use crate::dem::{DemGrid, GridBounds, GeometrySource, DemGridGenerator};
use crate::error::LandXMLError;
use std::collections::HashSet;
use rayon::prelude::*;

/// TINデータからDEMや点群を生成するための構造体
pub struct TriangulationSource {
    points: Vec<Point3D>,
    triangles: Vec<Triangle>,
    bounds: GridBounds,
}

impl TriangulationSource {
    /// Surfaceからтриangulationソースを作成
    pub fn from_surface(surface: &Surface) -> Result<Self, LandXMLError> {
        let definition = &surface.definition;
        
        let points = definition.points.clone();
        let triangles = definition.faces.clone();
        
        if points.is_empty() {
            return Err(LandXMLError::EmptyPointCloud);
        }
        
        let bounds = Self::calculate_bounds(&points);
        
        Ok(Self {
            points,
            triangles,
            bounds,
        })
    }

    /// 点群から直接作成（三角形なし）
    pub fn from_points(points: Vec<Point3D>) -> Result<Self, LandXMLError> {
        if points.is_empty() {
            return Err(LandXMLError::EmptyPointCloud);
        }
        
        let bounds = Self::calculate_bounds(&points);
        
        Ok(Self {
            points,
            triangles: Vec::new(),
            bounds,
        })
    }

    /// 境界範囲を計算
    fn calculate_bounds(points: &[Point3D]) -> GridBounds {
        let mut min_x = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_y = f64::NEG_INFINITY;
        let mut min_z = f32::INFINITY;
        let mut max_z = f32::NEG_INFINITY;

        for point in points {
            min_x = min_x.min(point.x);
            max_x = max_x.max(point.x);
            min_y = min_y.min(point.y);
            max_y = max_y.max(point.y);
            min_z = min_z.min(point.z);
            max_z = max_z.max(point.z);
        }

        GridBounds::new(min_x, max_x, min_y, max_y, min_z, max_z)
    }

    /// 点群データを取得（重複除去済み）
    pub fn extract_unique_points(&self) -> Vec<Point3D> {
        let mut unique_points = Vec::new();
        let mut seen_points = HashSet::new();

        for point in &self.points {
            // 座標の精度を考慮した重複判定（1mm精度）
            let key = (
                (point.x * 1000.0).round() as i64,
                (point.y * 1000.0).round() as i64,
                (point.z * 1000.0).round() as i32,
            );

            if seen_points.insert(key) {
                unique_points.push(*point);
            }
        }

        unique_points
    }

    /// 指定された領域内の点群を抽出
    pub fn extract_points_in_bounds(&self, bounds: GridBounds) -> Vec<Point3D> {
        self.points
            .iter()
            .filter(|point| {
                point.x >= bounds.min_x
                    && point.x <= bounds.max_x
                    && point.y >= bounds.min_y
                    && point.y <= bounds.max_y
            })
            .cloned()
            .collect()
    }

    /// 三角形の頂点を展開して点群として取得
    pub fn extract_triangle_vertices(&self) -> Vec<Point3D> {
        let mut vertices = Vec::new();
        
        for triangle in &self.triangles {
            // 三角形の各頂点インデックスから座標を取得
            if let (Some(p1), Some(p2), Some(p3)) = (
                self.points.get(triangle.vertex1()),
                self.points.get(triangle.vertex2()),
                self.points.get(triangle.vertex3()),
            ) {
                vertices.push(*p1);
                vertices.push(*p2);
                vertices.push(*p3);
            }
        }
        
        vertices
    }

    /// 三角形の重心点を計算
    pub fn triangle_centroids(&self) -> Vec<Point3D> {
        let mut centroids = Vec::new();
        
        for triangle in &self.triangles {
            if let (Some(p1), Some(p2), Some(p3)) = (
                self.points.get(triangle.vertex1()),
                self.points.get(triangle.vertex2()),
                self.points.get(triangle.vertex3()),
            ) {
                let centroid = Point3D {
                    x: (p1.x + p2.x + p3.x) / 3.0,
                    y: (p1.y + p2.y + p3.y) / 3.0,
                    z: (p1.z + p2.z + p3.z) / 3.0,
                    id: None,
                };
                centroids.push(centroid);
            }
        }
        
        centroids
    }

    /// 三角形内部の点の標高を重心座標補間で計算
    fn interpolate_in_triangle(&self, x: f64, y: f64, triangle: &Triangle) -> Option<f32> {
        let p1 = self.points.get(triangle.vertex1())?;
        let p2 = self.points.get(triangle.vertex2())?;
        let p3 = self.points.get(triangle.vertex3())?;

        // 重心座標を計算
        let denom = (p2.y - p3.y) * (p1.x - p3.x) + (p3.x - p2.x) * (p1.y - p3.y);
        if denom.abs() < 1e-10 {
            return None; // 縮退した三角形
        }

        let lambda1 = ((p2.y - p3.y) * (x - p3.x) + (p3.x - p2.x) * (y - p3.y)) / denom;
        let lambda2 = ((p3.y - p1.y) * (x - p3.x) + (p1.x - p3.x) * (y - p3.y)) / denom;
        let lambda3 = 1.0 - lambda1 - lambda2;

        // 点が三角形内部にあるかチェック（許容誤差付き）
        const EPSILON: f64 = -1e-10; // わずかに外側の点も許容
        if lambda1 >= EPSILON && lambda2 >= EPSILON && lambda3 >= EPSILON {
            let z = lambda1 * p1.z as f64 + lambda2 * p2.z as f64 + lambda3 * p3.z as f64;
            Some(z as f32)
        } else {
            None
        }
    }

    /// 三角形のバウンディングボックスを計算
    fn triangle_bbox(&self, triangle: &Triangle) -> Option<(f64, f64, f64, f64)> {
        let p1 = self.points.get(triangle.vertex1())?;
        let p2 = self.points.get(triangle.vertex2())?;
        let p3 = self.points.get(triangle.vertex3())?;
        
        let min_x = p1.x.min(p2.x).min(p3.x);
        let max_x = p1.x.max(p2.x).max(p3.x);
        let min_y = p1.y.min(p2.y).min(p3.y);
        let max_y = p1.y.max(p2.y).max(p3.y);
        
        Some((min_x, max_x, min_y, max_y))
    }

    /// 点が三角形のバウンディングボックス内にあるかチェック
    fn point_in_bbox(&self, x: f64, y: f64, bbox: (f64, f64, f64, f64)) -> bool {
        let (min_x, max_x, min_y, max_y) = bbox;
        x >= min_x && x <= max_x && y >= min_y && y <= max_y
    }

    /// 指定した座標で最も近い三角形を見つけて補間（最適化版）
    fn find_nearest_triangle_interpolation(&self, x: f64, y: f64) -> Option<f32> {
        // まず、バウンディングボックス内の三角形でのみ内部補間を試行
        for triangle in &self.triangles {
            if let Some(bbox) = self.triangle_bbox(triangle) {
                if self.point_in_bbox(x, y, bbox) {
                    if let Some(z) = self.interpolate_in_triangle(x, y, triangle) {
                        return Some(z);
                    }
                }
            }
        }

        // 内部補間が失敗した場合、最近傍重心との距離による補間
        let mut best_distance = f64::INFINITY;
        let mut best_elevation = None;

        for triangle in &self.triangles {
            if let (Some(p1), Some(p2), Some(p3)) = (
                self.points.get(triangle.vertex1()),
                self.points.get(triangle.vertex2()),
                self.points.get(triangle.vertex3()),
            ) {
                let centroid_x = (p1.x + p2.x + p3.x) / 3.0;
                let centroid_y = (p1.y + p2.y + p3.y) / 3.0;
                let centroid_z = (p1.z + p2.z + p3.z) / 3.0;

                let distance = ((x - centroid_x).powi(2) + (y - centroid_y).powi(2)).sqrt();
                if distance < best_distance {
                    best_distance = distance;
                    best_elevation = Some(centroid_z as f32);
                }
            }
        }

        best_elevation
    }

    /// 点群の品質を評価
    pub fn quality_assessment(&self) -> TriangulationQuality {
        let point_count = self.points.len();
        let triangle_count = self.triangles.len();
        
        // 理論的な三角形数（オイラーの公式より）
        let expected_triangles = if point_count >= 3 {
            2 * point_count - 5 // 凸包の場合の近似
        } else {
            0
        };

        // 点密度を計算
        let area = self.bounds.width() * self.bounds.height();
        let point_density = if area > 0.0 {
            point_count as f64 / area
        } else {
            0.0
        };

        TriangulationQuality {
            point_count,
            triangle_count,
            expected_triangles,
            point_density,
            bounds: self.bounds,
        }
    }
}

impl GeometrySource for TriangulationSource {
    fn bounds(&self) -> GridBounds {
        self.bounds
    }

    fn interpolate_elevation(&self, x: f64, y: f64) -> Option<f32> {
        if !self.triangles.is_empty() {
            self.find_nearest_triangle_interpolation(x, y)
        } else {
            // 三角形がない場合は最近傍点の標高を返す
            let mut min_distance = f64::INFINITY;
            let mut nearest_z = None;

            for point in &self.points {
                let distance = ((x - point.x).powi(2) + (y - point.y).powi(2)).sqrt();
                if distance < min_distance {
                    min_distance = distance;
                    nearest_z = Some(point.z);
                }
            }

            nearest_z
        }
    }

    fn points(&self) -> &[Point3D] {
        &self.points
    }

    fn triangles(&self) -> Option<&[Triangle]> {
        if self.triangles.is_empty() {
            None
        } else {
            Some(&self.triangles)
        }
    }
}

impl DemGridGenerator for TriangulationSource {
    fn generate_grid(&self, resolution: f64, bounds: Option<GridBounds>) -> Result<DemGrid, LandXMLError> {
        let grid_bounds = bounds.unwrap_or(self.bounds);
        let (rows, cols) = grid_bounds.grid_size(resolution);
        
        // グリッド原点を境界の左上に設定
        // GeoTIFFではピクセルの左上角が基準となる
        let origin_x = grid_bounds.min_x;
        let origin_y = grid_bounds.max_y;
        
        let mut grid = DemGrid::new(
            rows, 
            cols, 
            origin_x, 
            origin_y, 
            resolution, 
            resolution,
            grid_bounds
        );

        // 並列処理で各グリッドポイントの標高を補間
        let total_cells = rows * cols;
        let elevations: Vec<f32> = (0..total_cells)
            .into_par_iter()
            .map(|index| {
                let row = index / cols;
                let col = index % cols;
                
                let x = origin_x + (col as f64 + 0.5) * resolution;
                let y = origin_y - (row as f64 + 0.5) * resolution;
                
                self.interpolate_elevation(x, y).unwrap_or(-9999.0)
            })
            .collect();

        // 計算結果をグリッドに設定
        grid.values = elevations;

        Ok(grid)
    }
}

/// 三角形分割の品質評価結果
#[derive(Debug, Clone)]
pub struct TriangulationQuality {
    pub point_count: usize,
    pub triangle_count: usize,
    pub expected_triangles: usize,
    pub point_density: f64, // 点/平方メートル
    pub bounds: GridBounds,
}

impl TriangulationQuality {
    /// 三角分割の充実度を評価（0.0-1.0）
    pub fn completeness_ratio(&self) -> f64 {
        if self.expected_triangles == 0 {
            0.0
        } else {
            (self.triangle_count as f64 / self.expected_triangles as f64).min(1.0)
        }
    }
}