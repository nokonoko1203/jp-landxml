use crate::dem::DemGrid;
use crate::error::LandXMLError;
use gdal::{Dataset, DriverManager, Metadata};
use gdal::cpl::CslStringList;
use std::path::Path;

/// GeoTIFF圧縮タイプ
#[derive(Debug, Clone, Copy)]
pub enum CompressionType {
    None,
    Deflate,
    Lzw,
    Jpeg,
}

impl CompressionType {
    fn to_gdal_option(&self) -> (&'static str, &'static str) {
        match self {
            CompressionType::None => ("COMPRESS", "NONE"),
            CompressionType::Deflate => ("COMPRESS", "DEFLATE"),
            CompressionType::Lzw => ("COMPRESS", "LZW"),
            CompressionType::Jpeg => ("COMPRESS", "JPEG"),
        }
    }
}

/// GeoTIFF作成オプション
#[derive(Debug, Clone)]
pub struct GeoTiffOptions {
    pub compression: CompressionType,
    pub tiled: bool,
    pub tile_size: Option<usize>,
    pub predictor: Option<u8>, // 1=none, 2=horizontal, 3=floating point
    pub big_tiff: bool,
}

impl Default for GeoTiffOptions {
    fn default() -> Self {
        Self {
            compression: CompressionType::Deflate,
            tiled: true,
            tile_size: Some(256),
            predictor: Some(3), // floating point predictor
            big_tiff: false,
        }
    }
}

/// GeoTIFF書き込み機能を提供する構造体
pub struct GeoTiffWriter;

impl GeoTiffWriter {
    /// 新しいGeoTiffWriterを作成
    pub fn new() -> Self {
        Self
    }

    /// DEMグリッドをGeoTIFFファイルとして出力
    pub fn write_geotiff<P: AsRef<Path>>(
        &self,
        grid: &DemGrid,
        output_path: P,
        options: Option<GeoTiffOptions>,
    ) -> Result<(), LandXMLError> {
        let options = options.unwrap_or_default();
        
        // GDALドライバを取得
        let driver = DriverManager::get_driver_by_name("GTiff")
            .map_err(|e| LandXMLError::GdalError(format!("Failed to get GTiff driver: {}", e)))?;

        // 作成オプションを構築
        let mut create_options = Vec::new();
        
        // 圧縮設定
        let (compress_key, compress_value) = options.compression.to_gdal_option();
        create_options.push(format!("{}={}", compress_key, compress_value));
        
        // Predictor設定（圧縮効率向上）
        if let Some(predictor) = options.predictor {
            create_options.push(format!("PREDICTOR={}", predictor));
        }
        
        // タイル化設定
        if options.tiled {
            create_options.push("TILED=YES".to_string());
            if let Some(tile_size) = options.tile_size {
                create_options.push(format!("BLOCKXSIZE={}", tile_size));
                create_options.push(format!("BLOCKYSIZE={}", tile_size));
            }
        }
        
        // BigTIFF設定（4GB超のファイル用）
        if options.big_tiff {
            create_options.push("BIGTIFF=YES".to_string());
        }
        
        // CslStringListに変換
        let gdal_options = CslStringList::from_iter(create_options.iter().map(|s| s.as_str()));
        
        // データセットを作成（最適化オプション付き）
        let mut dataset = driver
            .create_with_band_type_with_options::<f32, _>(
                output_path.as_ref(),
                grid.cols,
                grid.rows,
                1,
                &gdal_options,
            )
            .map_err(|e| LandXMLError::GdalError(format!("Failed to create dataset: {}", e)))?;

        // GeoTransformを設定
        // 注意: grid.geo_transform()はピクセルの左上角を基準とした座標を返す
        // これはGeoTIFFの標準仕様に準拠している
        let geo_transform = grid.geo_transform();
        dataset
            .set_geo_transform(&geo_transform)
            .map_err(|e| LandXMLError::GdalError(format!("Failed to set geo transform: {}", e)))?;

        // 座標系を設定
        if let Some(epsg_code) = grid.epsg_code {
            let spatial_ref = gdal::spatial_ref::SpatialRef::from_epsg(epsg_code)
                .map_err(|e| LandXMLError::GdalError(format!("Failed to create spatial reference for EPSG:{}: {}", epsg_code, e)))?;
            
            dataset
                .set_projection(&spatial_ref.to_wkt().unwrap_or_default())
                .map_err(|e| LandXMLError::GdalError(format!("Failed to set projection: {}", e)))?;
        }

        // バンドを取得してデータを書き込み
        let mut rasterband = dataset
            .rasterband(1)
            .map_err(|e| LandXMLError::GdalError(format!("Failed to get raster band: {}", e)))?;

        // NoData値を設定
        rasterband
            .set_no_data_value(Some(-9999.0))
            .map_err(|e| LandXMLError::GdalError(format!("Failed to set no data value: {}", e)))?;

        // データを書き込み
        // GDALのBufferは(width, height)を期待し、データは行優先で格納される
        // grid.valuesは行優先（row-major）で格納されているので、そのまま使用できる
        use gdal::raster::Buffer;
        let mut buffer = Buffer::new((grid.cols, grid.rows), grid.values.clone());
        
        // ラスタデータを書き込み
        // write()の引数: (x_offset, y_offset), (width, height)
        rasterband
            .write((0, 0), (grid.cols, grid.rows), &mut buffer)
            .map_err(|e| LandXMLError::GdalError(format!("Failed to write raster data: {}", e)))?;

        // メタデータを追加
        self.write_metadata(&mut dataset, grid)?;
        
        // AREA_OR_POINT設定を明示的にPointに設定
        // J-LandXMLのDEMデータはポイントデータとして扱うべき
        dataset
            .set_metadata_item("AREA_OR_POINT", "Point", "")
            .map_err(|e| LandXMLError::GdalError(format!("Failed to set AREA_OR_POINT metadata: {}", e)))?;

        // 地理参照情報の検証（デバッグ用）
        if let Ok(geo_transform) = dataset.geo_transform() {
            println!("    GeoTIFF GeoTransform: [{:.6}, {:.6}, {:.6}, {:.6}, {:.6}, {:.6}]",
                geo_transform[0], geo_transform[1], geo_transform[2],
                geo_transform[3], geo_transform[4], geo_transform[5]);
            
            // 四隅の座標を計算（GeoTIFFではピクセルの左上角が基準）
            let top_left = (geo_transform[0], geo_transform[3]);
            let top_right = (geo_transform[0] + geo_transform[1] * grid.cols as f64,
                           geo_transform[3] + geo_transform[4] * grid.cols as f64);
            let bottom_left = (geo_transform[0] + geo_transform[2] * grid.rows as f64,
                             geo_transform[3] + geo_transform[5] * grid.rows as f64);
            let bottom_right = (geo_transform[0] + geo_transform[1] * grid.cols as f64 + geo_transform[2] * grid.rows as f64,
                              geo_transform[3] + geo_transform[4] * grid.cols as f64 + geo_transform[5] * grid.rows as f64);
            
            println!("    Corner coordinates (pixel edges):");
            println!("      Top-left: ({:.6}, {:.6})", top_left.0, top_left.1);
            println!("      Top-right: ({:.6}, {:.6})", top_right.0, top_right.1);
            println!("      Bottom-left: ({:.6}, {:.6})", bottom_left.0, bottom_left.1);
            println!("      Bottom-right: ({:.6}, {:.6})", bottom_right.0, bottom_right.1);
            
            // グリッドの期待される範囲と比較
            println!("    Expected bounds from grid:");
            println!("      Min X: {:.6}, Max X: {:.6}", grid.bounds.min_x, grid.bounds.max_x);
            println!("      Min Y: {:.6}, Max Y: {:.6}", grid.bounds.min_y, grid.bounds.max_y);
            
            // ピクセル中心の計算例（最初と最後のピクセル）
            let first_pixel_center_x = geo_transform[0] + 0.5 * geo_transform[1];
            let first_pixel_center_y = geo_transform[3] + 0.5 * geo_transform[5];
            let last_pixel_center_x = geo_transform[0] + (grid.cols as f64 - 0.5) * geo_transform[1];
            let last_pixel_center_y = geo_transform[3] + (grid.rows as f64 - 0.5) * geo_transform[5];
            
            println!("    Pixel center coordinates:");
            println!("      First pixel (0,0): ({:.6}, {:.6})", first_pixel_center_x, first_pixel_center_y);
            println!("      Last pixel ({},{}): ({:.6}, {:.6})", grid.rows-1, grid.cols-1, last_pixel_center_x, last_pixel_center_y);
        }

        Ok(())
    }

    /// 簡単なGeoTIFF出力（デフォルトオプション使用）
    pub fn write_simple<P: AsRef<Path>>(
        &self,
        grid: &DemGrid,
        output_path: P,
    ) -> Result<(), LandXMLError> {
        self.write_geotiff(grid, output_path, None)
    }

    /// メタデータを書き込み
    fn write_metadata(&self, dataset: &mut Dataset, grid: &DemGrid) -> Result<(), LandXMLError> {
        let stats = grid.statistics();
        
        // 統計情報をメタデータとして追加
        let metadata = vec![
            ("STATISTICS_MINIMUM", stats.min_value.to_string()),
            ("STATISTICS_MAXIMUM", stats.max_value.to_string()),
            ("STATISTICS_MEAN", stats.mean_value.to_string()),
            ("VALID_PERCENT", (stats.valid_ratio() * 100.0).to_string()),
            ("NODATA_COUNT", (stats.total_count - stats.valid_count).to_string()),
            ("CREATOR", "jp-landxml".to_string()),
            ("SOURCE", "LandXML Surface TIN".to_string()),
        ];

        for (key, value) in metadata {
            dataset
                .set_metadata_item(key, &value, "")
                .map_err(|e| LandXMLError::GdalError(format!("Failed to set metadata {}: {}", key, e)))?;
        }

        Ok(())
    }

    /// ASCII Grid（Arc/Info ASCII Grid）形式で出力
    pub fn write_ascii_grid<P: AsRef<Path>>(
        &self,
        grid: &DemGrid,
        output_path: P,
    ) -> Result<(), LandXMLError> {
        use std::fs::File;
        use std::io::{BufWriter, Write};

        let file = File::create(output_path.as_ref())
            .map_err(|e| LandXMLError::Io(e))?;
        let mut writer = BufWriter::new(file);

        // ヘッダー情報を書き込み
        writeln!(writer, "ncols         {}", grid.cols)?;
        writeln!(writer, "nrows         {}", grid.rows)?;
        writeln!(writer, "xllcorner     {}", grid.origin_x)?;
        writeln!(writer, "yllcorner     {}", grid.origin_y - grid.rows as f64 * grid.y_resolution)?;
        writeln!(writer, "cellsize      {}", grid.x_resolution)?;
        writeln!(writer, "NODATA_value  -9999")?;

        // データを書き込み
        for row in 0..grid.rows {
            for col in 0..grid.cols {
                if col > 0 {
                    write!(writer, " ")?;
                }
                let value = grid.values[row * grid.cols + col];
                write!(writer, "{}", value)?;
            }
            writeln!(writer)?;
        }

        writer.flush().map_err(|e| LandXMLError::Io(e))?;
        Ok(())
    }

    /// XYZ点群形式で出力（有効な値のみ）
    pub fn write_xyz<P: AsRef<Path>>(
        &self,
        grid: &DemGrid,
        output_path: P,
    ) -> Result<(), LandXMLError> {
        use std::fs::File;
        use std::io::{BufWriter, Write};

        let file = File::create(output_path.as_ref())
            .map_err(|e| LandXMLError::Io(e))?;
        let mut writer = BufWriter::new(file);

        for row in 0..grid.rows {
            for col in 0..grid.cols {
                let value = grid.values[row * grid.cols + col];
                if value != -9999.0 {
                    if let Some((x, y)) = grid.grid_to_world(row, col) {
                        writeln!(writer, "{:.6} {:.6} {:.3}", x, y, value)?;
                    }
                }
            }
        }

        writer.flush().map_err(|e| LandXMLError::Io(e))?;
        Ok(())
    }
}

impl Default for GeoTiffWriter {
    fn default() -> Self {
        Self::new()
    }
}