#[cfg(feature = "cli")]
use clap::{Parser, Subcommand};
use jp_landxml::{LandXMLParser, SurfaceType};
use jp_landxml::dem::{TriangulationSource, GeoTiffWriter, GridBounds};
use std::path::PathBuf;
use std::process;

#[cfg(feature = "cli")]
#[derive(Parser)]
#[command(name = "jp-landxml")]
#[command(about = "A CLI tool for processing J-LandXML files")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[cfg(feature = "cli")]
#[derive(Subcommand)]
enum Commands {
    /// Parse LandXML file and output information
    Parse {
        /// Input LandXML file
        input: PathBuf,
        /// Output JSON file (optional)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Export Surface as DEM (Digital Elevation Model)
    ExportDem {
        /// Input LandXML file
        input: PathBuf,
        
        /// Output directory for DEM files
        #[arg(short, long)]
        output: PathBuf,
        
        /// DEM resolution in meters
        #[arg(long, default_value = "1.0")]
        resolution: f64,
        
        /// Surface description filter (e.g., "ExistingGround", "FinishedGrade")
        #[arg(long)]
        surface_filter: Option<String>,
        
        /// Output format
        #[arg(long, value_enum, default_value = "geotiff")]
        format: OutputFormat,
        
        /// Compression type for GeoTIFF
        #[arg(long, value_enum, default_value = "deflate")]
        compression: CompressionOption,
        
        /// Number of parallel threads
        #[arg(long)]
        threads: Option<usize>,
        
        /// Export all surfaces individually
        #[arg(long)]
        all_surfaces: bool,
    },
}

#[cfg(feature = "cli")]
#[derive(clap::ValueEnum, Clone)]
enum OutputFormat {
    Geotiff,
    AsciiGrid,
    Xyz,
}

#[cfg(feature = "cli")]
#[derive(clap::ValueEnum, Clone)]
enum CompressionOption {
    None,
    Deflate,
    Lzw,
    Jpeg,
}

#[cfg(feature = "cli")]
impl From<CompressionOption> for jp_landxml::CompressionType {
    fn from(opt: CompressionOption) -> Self {
        match opt {
            CompressionOption::None => jp_landxml::CompressionType::None,
            CompressionOption::Deflate => jp_landxml::CompressionType::Deflate,
            CompressionOption::Lzw => jp_landxml::CompressionType::Lzw,
            CompressionOption::Jpeg => jp_landxml::CompressionType::Jpeg,
        }
    }
}

#[cfg(feature = "cli")]
fn main() {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Parse { input, output } => {
            if let Err(e) = handle_parse_command(input, output) {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        }
        Commands::ExportDem { 
            input, 
            output, 
            resolution, 
            surface_filter, 
            format, 
            compression, 
            threads,
            all_surfaces,
        } => {
            if let Err(e) = handle_export_dem_command(
                input, 
                output, 
                resolution, 
                surface_filter, 
                format, 
                compression, 
                threads,
                all_surfaces,
            ) {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        }
    }
}

#[cfg(not(feature = "cli"))]
fn main() {
    eprintln!("CLI feature is not enabled. Build with --features cli to use the command line interface.");
    process::exit(1);
}

#[cfg(feature = "cli")]
fn handle_parse_command(input: PathBuf, output: Option<PathBuf>) -> Result<(), Box<dyn std::error::Error>> {
    println!("Parsing LandXML file: {}", input.display());
    
    let parser = LandXMLParser::from_file(&input)?;
    let landxml = parser.parse()?;
    
    println!("Successfully parsed LandXML file!");
    println!("Version: {}", landxml.version);
    
    println!("Surfaces: {}", landxml.surfaces.len());
    for (i, surface) in landxml.surfaces.iter().enumerate() {
        let surface_type_str = match &surface.surface_type {
            SurfaceType::ExistingGround => "ExistingGround",
            SurfaceType::DesignGround => "DesignGround",
            SurfaceType::Other(s) => s,
        };
        println!("  {}: {} ({})", i + 1, surface.name, surface_type_str);
        println!("    Points: {}, Triangles: {}", surface.definition.points.len(), surface.definition.faces.len());
    }
    
    println!("Alignments: {}", landxml.alignments.len());
    for (i, alignment) in landxml.alignments.iter().enumerate() {
        println!("  {}: {}", i + 1, alignment.name);
    }
    
    // JSON出力
    if let Some(output_path) = output {
        let json = serde_json::to_string_pretty(&landxml)?;
        std::fs::write(&output_path, json)?;
        println!("JSON output written to: {}", output_path.display());
    }
    
    Ok(())
}

#[cfg(feature = "cli")]
fn handle_export_dem_command(
    input: PathBuf,
    output: PathBuf,
    resolution: f64,
    surface_filter: Option<String>,
    format: OutputFormat,
    compression: CompressionOption,
    threads: Option<usize>,
    all_surfaces: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    use rayon::ThreadPoolBuilder;
    use std::fs;
    use jp_landxml::{TriangulationSource, GeoTiffWriter};
    use jp_landxml::dem::DemGridGenerator;
    
    // スレッドプール設定
    if let Some(num_threads) = threads {
        ThreadPoolBuilder::new()
            .num_threads(num_threads)
            .build_global()?;
        println!("Using {} threads", num_threads);
    }
    
    // 出力ディレクトリを作成
    fs::create_dir_all(&output)?;
    
    println!("Parsing LandXML file: {}", input.display());
    
    // J-LandXMLパーサーを試行、失敗したら標準パーサーを使用
    let (landxml, detected_epsg) = match jp_landxml::JLandXmlParser::from_file(&input) {
        Ok(jlandxml_parser) => {
            match jlandxml_parser.parse() {
                Ok(jlandxml_doc) => {
                    let detected_epsg = jlandxml_doc.get_epsg_code();
                    if let Some(epsg) = detected_epsg {
                        println!("  Detected J-LandXML with coordinate system: EPSG:{}", epsg);
                        if let Some(zone) = jlandxml_doc.get_plane_coordinate_zone() {
                            println!("  {}", zone);
                        }
                        
                        // 座標系詳細情報を表示
                        if let Some(coord_sys) = &jlandxml_doc.coordinate_system {
                            println!("  Horizontal datum: {:?}", coord_sys.horizontal_datum);
                            println!("  Vertical datum: {:?}", coord_sys.vertical_datum);
                            println!("  Coordinate system name: {}", coord_sys.horizontal_coordinate_system_name);
                            if let Some(differ_tp) = coord_sys.differ_tp {
                                println!("  DifferTP: {:.4}m", differ_tp);
                            }
                        }
                    }
                    (jlandxml_doc.base, detected_epsg)
                }
                Err(_) => {
                    println!("  J-LandXML parsing failed, falling back to standard LandXML");
                    let parser = LandXMLParser::from_file(&input)?;
                    (parser.parse()?, None)
                }
            }
        }
        Err(_) => {
            let parser = LandXMLParser::from_file(&input)?;
            (parser.parse()?, None)
        }
    };
    
    if landxml.surfaces.is_empty() {
        return Err("No surfaces found in LandXML file".into());
    }
    let surfaces = &landxml.surfaces;
    
    // Surfaceフィルタリング
    let target_surfaces: Vec<_> = if all_surfaces {
        surfaces.iter().collect()
    } else if let Some(ref filter) = surface_filter {
        surfaces.iter()
            .filter(|surface| {
                let surface_type_str = match &surface.surface_type {
                    SurfaceType::ExistingGround => "ExistingGround",
                    SurfaceType::DesignGround => "DesignGround",
                    SurfaceType::Other(s) => s,
                };
                surface_type_str.contains(filter) || surface.name.contains(filter)
            })
            .collect()
    } else {
        // デフォルトでは最初のサーフェスのみ
        vec![&surfaces[0]]
    };
    
    if target_surfaces.is_empty() {
        return Err("No surfaces match the specified filter".into());
    }
    
    println!("Found {} surface(s) to export", target_surfaces.len());
    
    let writer = GeoTiffWriter::new();
    
    for surface in target_surfaces {
        let surface_type_str = match &surface.surface_type {
            SurfaceType::ExistingGround => "ExistingGround",
            SurfaceType::DesignGround => "DesignGround",
            SurfaceType::Other(s) => s,
        };
        println!("Processing surface: {} ({})", surface.name, surface_type_str);
        
        // TriangulationSourceを作成
        let tri_source = TriangulationSource::from_surface(surface)?;
        
        // 品質評価
        let quality = tri_source.quality_assessment();
        println!("  Points: {}, Triangles: {}", quality.point_count, quality.triangle_count);
        println!("  Point density: {:.2} points/m²", quality.point_density);
        println!("  Bounds: ({:.6}, {:.6}) to ({:.6}, {:.6})", 
                 quality.bounds.min_x, quality.bounds.min_y,
                 quality.bounds.max_x, quality.bounds.max_y);
        
        // 座標値の範囲を分析（座標系が適切かチェック）
        let x_range = quality.bounds.max_x - quality.bounds.min_x;
        let y_range = quality.bounds.max_y - quality.bounds.min_y;
        println!("  Coordinate range: X={:.2}m, Y={:.2}m", x_range, y_range);
        
        // 座標値から座標系タイプを推定（デバッグ用）
        if quality.bounds.min_x > -180.0 && quality.bounds.max_x < 180.0 &&
           quality.bounds.min_y > -90.0 && quality.bounds.max_y < 90.0 {
            println!("  WARNING: Coordinates appear to be in geographic (lat/lon) format!");
            println!("           Expected plane rectangular coordinates for specified coordinate system");
        } else if quality.bounds.min_x > 100000.0 {
            println!("  Coordinates appear to be in plane rectangular coordinate system");
        }
        
        // DEMグリッド生成
        println!("  Generating DEM grid with resolution: {}m", resolution);
        let mut dem_grid = tri_source.generate_grid(resolution, None)?;
        
        // 座標系を設定（J-LandXMLで検出されたEPSGコードを優先、なければ座標値から推定）
        dem_grid.epsg_code = detected_epsg.or_else(|| {
            // 座標値から平面直角座標系を推定
            estimate_coordinate_system_from_bounds(&quality.bounds)
        });
        
        if let Some(epsg) = dem_grid.epsg_code {
            println!("  Using coordinate system: EPSG:{}", epsg);
        }
        
        // 座標系詳細情報を表示（デバッグ用）
        println!("  Grid origin: ({:.6}, {:.6})", dem_grid.origin_x, dem_grid.origin_y);
        println!("  Grid resolution: {:.3}m x {:.3}m", dem_grid.x_resolution, dem_grid.y_resolution);
        println!("  Grid extent: ({:.2}, {:.2}) to ({:.2}, {:.2})", 
                 dem_grid.origin_x, 
                 dem_grid.origin_y - dem_grid.rows as f64 * dem_grid.y_resolution,
                 dem_grid.origin_x + dem_grid.cols as f64 * dem_grid.x_resolution,
                 dem_grid.origin_y);
        
        let stats = dem_grid.statistics();
        println!("  Grid size: {}x{} ({} cells)", dem_grid.rows, dem_grid.cols, stats.total_count);
        println!("  Valid data: {:.1}% ({} cells)", stats.valid_ratio() * 100.0, stats.valid_count);
        if stats.valid_count > 0 {
            println!("  Elevation range: {:.2}m to {:.2}m", stats.min_value, stats.max_value);
        }
        
        // ファイル名を生成
        let safe_name = surface.name.replace(" ", "_").replace("/", "_");
        
        // 出力ファイルを書き込み
        match format {
            OutputFormat::Geotiff => {
                let filename = format!("{}.tif", safe_name);
                let output_path = output.join(filename);
                
                let options = jp_landxml::dem::geotiff_writer::GeoTiffOptions {
                    compression: compression.clone().into(),
                    ..Default::default()
                };
                
                writer.write_geotiff(&dem_grid, &output_path, Some(options))?;
                println!("  Written: {}", output_path.display());
            }
            OutputFormat::AsciiGrid => {
                let filename = format!("{}.asc", safe_name);
                let output_path = output.join(filename);
                
                writer.write_ascii_grid(&dem_grid, &output_path)?;
                println!("  Written: {}", output_path.display());
            }
            OutputFormat::Xyz => {
                let filename = format!("{}.xyz", safe_name);
                let output_path = output.join(filename);
                
                writer.write_xyz(&dem_grid, &output_path)?;
                println!("  Written: {}", output_path.display());
            }
        }
    }
    
    println!("DEM export completed successfully!");
    Ok(())
}

/// 座標値の範囲から適切な平面直角座標系（EPSG）を推定
fn estimate_coordinate_system_from_bounds(bounds: &GridBounds) -> Option<u32> {
    // 日本の平面直角座標系の典型的な座標範囲で判定
    let center_x = (bounds.min_x + bounds.max_x) / 2.0;
    let center_y = (bounds.min_y + bounds.max_y) / 2.0;
    
    // 座標値が小さすぎる場合はテストデータと判定してデフォルトを返す
    if bounds.max_x < 1000.0 && bounds.max_y < 1000.0 && bounds.min_x >= 0.0 && bounds.min_y >= 0.0 {
        println!("  Note: Small positive coordinate values detected (test data?), using default EPSG:6677 (Zone 9)");
        return Some(6677); // 東京都（9系）をデフォルト
    }
    
    // 平面直角座標系の原点からの距離で判定
    // 各系の原点周辺での典型的な座標範囲を基に推定
    match (center_x, center_y) {
        // 1系: 北海道（西部）
        (x, y) if x > 20000.0 && x < 100000.0 && y > -100000.0 && y < 0.0 => {
            println!("  Estimated coordinate system: Zone 1 (Hokkaido West)");
            Some(6669)
        },
        // 2系: 北海道（中央・東部）
        (x, y) if x > 0.0 && x < 80000.0 && y > -100000.0 && y < 50000.0 => {
            println!("  Estimated coordinate system: Zone 2 (Hokkaido Central/East)");
            Some(6670)
        },
        // 9系: 東京都、福島県、栃木県、群馬県、埼玉県
        (x, y) if x > -100000.0 && x < 50000.0 && y > -100000.0 && y < 50000.0 => {
            println!("  Estimated coordinate system: Zone 9 (Tokyo, Fukushima, Tochigi, Gunma, Saitama)");
            Some(6677)
        },
        // 6系: 山形県、福島県、新潟県
        (x, y) if x > -50000.0 && x < 100000.0 && y > -50000.0 && y < 100000.0 => {
            println!("  Estimated coordinate system: Zone 6 (Yamagata, Fukushima, Niigata)");
            Some(6674)
        },
        // 7系: 茨城県、栃木県、群馬県、埼玉県、千葉県
        (x, y) if x > -80000.0 && x < 80000.0 && y > -80000.0 && y < 80000.0 => {
            println!("  Estimated coordinate system: Zone 7 (Ibaraki, Tochigi, Gunma, Saitama, Chiba)");
            Some(6675)
        },
        // 8系: 新潟県、群馬県、長野県
        (x, y) if x > -100000.0 && x < 100000.0 && y > -50000.0 && y < 100000.0 => {
            println!("  Estimated coordinate system: Zone 8 (Niigata, Gunma, Nagano)");
            Some(6676)
        },
        // 10系: 茨城県、埼玉県、千葉県、東京都、神奈川県、山梨県
        (x, y) if x > -100000.0 && x < 50000.0 && y > -100000.0 && y < 50000.0 => {
            println!("  Estimated coordinate system: Zone 10 (Ibaraki, Saitama, Chiba, Tokyo, Kanagawa, Yamanashi)");
            Some(6678)
        },
        // その他の場合はデフォルト（9系: 東京）
        _ => {
            println!("  Could not estimate coordinate system from bounds, using default EPSG:6677 (Zone 9)");
            println!("  Center coordinates: ({:.1}, {:.1})", center_x, center_y);
            Some(6677)
        }
    }
}