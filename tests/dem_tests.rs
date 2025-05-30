use jp_landxml::{DemGrid, GridBounds, TriangulationSource, GeoTiffWriter};
use jp_landxml::dem::{DemGridGenerator, GeometrySource};
use jp_landxml::models::{Point3D, Surface, SurfaceDefinition};
use tempfile::TempDir;

/// テスト用の簡単なTINデータを作成
fn create_test_surface() -> Surface {
    use jp_landxml::models::{SurfaceType, Face};
    
    // 2x2の正方形のTINメッシュ（4点、2三角形）
    let points = vec![
        Point3D { x: 0.0, y: 0.0, z: 10.0, id: Some(1) },  // 左下
        Point3D { x: 10.0, y: 0.0, z: 12.0, id: Some(2) }, // 右下
        Point3D { x: 0.0, y: 10.0, z: 11.0, id: Some(3) }, // 左上
        Point3D { x: 10.0, y: 10.0, z: 13.0, id: Some(4) }, // 右上
    ];
    
    let triangles = vec![
        Face { p1: 0, p2: 1, p3: 2 }, // 左下三角形
        Face { p1: 1, p2: 3, p3: 2 }, // 右上三角形
    ];
    
    Surface {
        name: "TestSurface".to_string(),
        surface_type: SurfaceType::ExistingGround,
        definition: SurfaceDefinition {
            points,
            faces: triangles,
        },
    }
}

#[test]
fn test_triangulation_source_creation() {
    let surface = create_test_surface();
    let tri_source = TriangulationSource::from_surface(&surface).unwrap();
    
    assert_eq!(tri_source.points().len(), 4);
    assert_eq!(tri_source.triangles().unwrap().len(), 2);
    
    let bounds = tri_source.bounds();
    assert_eq!(bounds.min_x, 0.0);
    assert_eq!(bounds.max_x, 10.0);
    assert_eq!(bounds.min_y, 0.0);
    assert_eq!(bounds.max_y, 10.0);
    assert_eq!(bounds.min_z, 10.0);
    assert_eq!(bounds.max_z, 13.0);
}

#[test]
fn test_dem_grid_generation() {
    let surface = create_test_surface();
    let tri_source = TriangulationSource::from_surface(&surface).unwrap();
    
    // 1mの解像度でDEMグリッドを生成
    let dem_grid = tri_source.generate_grid(1.0, None).unwrap();
    
    assert_eq!(dem_grid.x_resolution, 1.0);
    assert_eq!(dem_grid.y_resolution, 1.0);
    assert!(dem_grid.rows > 0);
    assert!(dem_grid.cols > 0);
    
    // グリッドサイズの妥当性をチェック
    let bounds = tri_source.bounds();
    let expected_cols = (bounds.width() / 1.0).ceil() as usize;
    let expected_rows = (bounds.height() / 1.0).ceil() as usize;
    assert_eq!(dem_grid.cols, expected_cols);
    assert_eq!(dem_grid.rows, expected_rows);
    
    // データ検証
    assert!(dem_grid.validate().is_ok());
}

#[test]
fn test_grid_bounds_calculation() {
    let bounds = GridBounds::new(0.0, 10.0, 0.0, 10.0, 10.0, 13.0);
    
    assert_eq!(bounds.width(), 10.0);
    assert_eq!(bounds.height(), 10.0);
    assert_eq!(bounds.elevation_range(), 3.0);
    
    let (rows, cols) = bounds.grid_size(2.0);
    assert_eq!(rows, 5); // 10/2 = 5
    assert_eq!(cols, 5); // 10/2 = 5
}

#[test]
fn test_dem_grid_coordinate_conversion() {
    let bounds = GridBounds::new(0.0, 10.0, 0.0, 10.0, 10.0, 13.0);
    let dem_grid = DemGrid::new(11, 11, 0.0, 10.0, 1.0, 1.0, bounds);
    
    // グリッド座標から地理座標への変換
    let (x, y) = dem_grid.grid_to_world(0, 0).unwrap();
    assert!((x - 0.5).abs() < 1e-10); // グリッド中心
    assert!((y - 9.5).abs() < 1e-10); // 上端から0.5m下
    
    // 地理座標からグリッド座標への変換
    let (row, col) = dem_grid.world_to_grid(5.5, 5.5).unwrap();
    assert_eq!(row, 4); // (10.0 - 5.5) / 1.0 = 4.5 -> 4
    assert_eq!(col, 5); // (5.5 - 0.0) / 1.0 = 5.5 -> 5
}

#[test]
fn test_interpolation() {
    let surface = create_test_surface();
    let tri_source = TriangulationSource::from_surface(&surface).unwrap();
    
    // 三角形の中心点で補間
    let elevation = tri_source.interpolate_elevation(5.0, 5.0);
    assert!(elevation.is_some());
    
    let z = elevation.unwrap();
    // 4つの頂点の平均値に近い値が期待される
    assert!(z > 10.0 && z < 13.0);
    
    // 範囲外の点（最近傍補間なので値は返るが、遠い場所）
    let elevation = tri_source.interpolate_elevation(-1.0, -1.0);
    assert!(elevation.is_some()); // 最近傍補間により値が返される
}

#[test]
fn test_point_extraction() {
    let surface = create_test_surface();
    let tri_source = TriangulationSource::from_surface(&surface).unwrap();
    
    // 一意の点を抽出
    let unique_points = tri_source.extract_unique_points();
    assert_eq!(unique_points.len(), 4); // 重複なし
    
    // 三角形頂点を抽出
    let triangle_vertices = tri_source.extract_triangle_vertices();
    assert_eq!(triangle_vertices.len(), 6); // 2三角形 × 3頂点
    
    // 重心点を計算
    let centroids = tri_source.triangle_centroids();
    assert_eq!(centroids.len(), 2); // 2三角形
}

#[test]
fn test_quality_assessment() {
    let surface = create_test_surface();
    let tri_source = TriangulationSource::from_surface(&surface).unwrap();
    
    let quality = tri_source.quality_assessment();
    assert_eq!(quality.point_count, 4);
    assert_eq!(quality.triangle_count, 2);
    assert!(quality.point_density > 0.0);
    
    let completeness = quality.completeness_ratio();
    assert!(completeness >= 0.0 && completeness <= 1.0);
}

#[test]
fn test_geotiff_writing() {
    let surface = create_test_surface();
    let tri_source = TriangulationSource::from_surface(&surface).unwrap();
    let dem_grid = tri_source.generate_grid(2.0, None).unwrap();
    
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("test_dem.tif");
    
    let writer = GeoTiffWriter::new();
    let result = writer.write_simple(&dem_grid, &output_path);
    
    assert!(result.is_ok(), "GeoTIFF writing failed: {:?}", result.err());
    assert!(output_path.exists(), "Output file was not created");
    
    // ファイルサイズをチェック
    let metadata = std::fs::metadata(&output_path).unwrap();
    assert!(metadata.len() > 0, "Output file is empty");
}

#[test]
fn test_ascii_grid_writing() {
    let surface = create_test_surface();
    let tri_source = TriangulationSource::from_surface(&surface).unwrap();
    let dem_grid = tri_source.generate_grid(2.0, None).unwrap();
    
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("test_dem.asc");
    
    let writer = GeoTiffWriter::new();
    let result = writer.write_ascii_grid(&dem_grid, &output_path);
    
    assert!(result.is_ok(), "ASCII grid writing failed: {:?}", result.err());
    assert!(output_path.exists(), "Output file was not created");
    
    // ファイル内容をチェック
    let content = std::fs::read_to_string(&output_path).unwrap();
    assert!(content.contains("ncols"));
    assert!(content.contains("nrows"));
    assert!(content.contains("xllcorner"));
    assert!(content.contains("yllcorner"));
    assert!(content.contains("cellsize"));
    assert!(content.contains("NODATA_value"));
}

#[test]
fn test_xyz_writing() {
    let surface = create_test_surface();
    let tri_source = TriangulationSource::from_surface(&surface).unwrap();
    let dem_grid = tri_source.generate_grid(2.0, None).unwrap();
    
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("test_dem.xyz");
    
    let writer = GeoTiffWriter::new();
    let result = writer.write_xyz(&dem_grid, &output_path);
    
    assert!(result.is_ok(), "XYZ writing failed: {:?}", result.err());
    assert!(output_path.exists(), "Output file was not created");
    
    // ファイル内容をチェック
    let content = std::fs::read_to_string(&output_path).unwrap();
    let lines: Vec<&str> = content.lines().collect();
    assert!(lines.len() > 0, "XYZ file should contain data lines");
    
    // 最初の行のフォーマットをチェック
    if let Some(first_line) = lines.first() {
        let parts: Vec<&str> = first_line.split_whitespace().collect();
        assert_eq!(parts.len(), 3, "Each XYZ line should have 3 values (X Y Z)");
    }
}