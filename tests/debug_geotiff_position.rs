#[cfg(test)]
mod tests {
    use jp_landxml::dem::{DemGrid, GeoTiffWriter, GridBounds};
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_geotiff_pixel_order() {
        // デバッグ用の3x3グリッド作成
        let rows = 3;
        let cols = 3;
        let origin_x = 0.0;
        let origin_y = 30.0; // 左上のY座標
        let resolution = 10.0;
        
        let bounds = GridBounds::new(0.0, 30.0, 0.0, 30.0, 1.0, 9.0);
        let mut grid = DemGrid::new(rows, cols, origin_x, origin_y, resolution, resolution, bounds);
        
        // 値を設定（行・列がわかりやすいように）
        // 行0（上）: 1, 2, 3
        // 行1（中）: 4, 5, 6  
        // 行2（下）: 7, 8, 9
        for row in 0..rows {
            for col in 0..cols {
                let value = (row * cols + col + 1) as f32;
                grid.set_value(row, col, value).unwrap();
            }
        }
        
        // 各ピクセルの座標を確認
        println!("=== Grid Pixel Coordinates ===");
        for row in 0..rows {
            for col in 0..cols {
                if let Some((x, y)) = grid.grid_to_world(row, col) {
                    let value = grid.get_value(row, col).unwrap();
                    println!("Row {}, Col {} -> World ({:.1}, {:.1}) = {:.0}", 
                             row, col, x, y, value);
                }
            }
        }
        
        // GeoTransformを確認
        let geo_transform = grid.geo_transform();
        println!("\n=== GeoTransform ===");
        println!("Origin X: {}", geo_transform[0]);
        println!("Pixel Width: {}", geo_transform[1]);
        println!("Rotation X: {}", geo_transform[2]);
        println!("Origin Y: {}", geo_transform[3]);
        println!("Rotation Y: {}", geo_transform[4]);
        println!("Pixel Height: {}", geo_transform[5]);
        
        // 逆変換テスト
        println!("\n=== World to Grid Conversion ===");
        let test_points = vec![
            (5.0, 25.0),   // 左上付近
            (15.0, 25.0),  // 上中央付近
            (25.0, 25.0),  // 右上付近
            (5.0, 15.0),   // 左中央付近
            (15.0, 15.0),  // 中央
            (25.0, 15.0),  // 右中央付近
            (5.0, 5.0),    // 左下付近
            (15.0, 5.0),   // 下中央付近
            (25.0, 5.0),   // 右下付近
        ];
        
        for (x, y) in test_points {
            if let Some((row, col)) = grid.world_to_grid(x, y) {
                let value = grid.get_value(row, col).unwrap();
                println!("World ({:.1}, {:.1}) -> Grid [{}, {}] = {:.0}", 
                         x, y, row, col, value);
            }
        }
        
        // GeoTIFF出力
        let output_dir = "test_output";
        fs::create_dir_all(output_dir).unwrap();
        
        let writer = GeoTiffWriter::new();
        let output_path = Path::new(output_dir).join("debug_pixel_order.tif");
        
        grid.epsg_code = Some(6677); // 東京9系
        writer.write_simple(&grid, &output_path).unwrap();
        
        println!("\n=== GeoTIFF Created ===");
        println!("Output: {}", output_path.display());
        
        // 生成されたファイルのメタデータを確認（gdalinfoで確認することを推奨）
        println!("\nTo verify the result, run:");
        println!("gdalinfo {}", output_path.display());
        println!("gdal_translate -of XYZ {} {}.xyz", 
                 output_path.display(), 
                 output_path.with_extension("").display());
    }
}