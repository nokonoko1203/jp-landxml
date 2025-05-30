use jp_landxml::{LandXMLParser, SurfaceType};
use std::path::Path;

#[test]
fn test_parse_real_sample_xml() {
    let sample_path = Path::new("tests/sample.xml");
    
    // ファイルの存在確認
    assert!(sample_path.exists(), "Sample XML file does not exist");
    
    let parser = LandXMLParser::from_file(sample_path).expect("Failed to create parser");
    let landxml = parser.parse().expect("Failed to parse real LandXML");
    
    // 基本的な構造の検証
    assert_eq!(landxml.version, "1.2");
    assert_eq!(landxml.surfaces.len(), 1);
    
    // 座標系の検証
    assert!(landxml.coordinate_system.is_some());
    let coord_sys = landxml.coordinate_system.unwrap();
    assert_eq!(coord_sys.name, "CRS-1");
    
    // サーフェスの検証
    let surface = &landxml.surfaces[0];
    assert_eq!(surface.name, "三角網1");
    assert!(matches!(surface.surface_type, SurfaceType::ExistingGround));
    
    // データ量の検証（大まかな範囲）
    assert!(surface.definition.points.len() > 100000, "Expected large number of points");
    assert!(surface.definition.faces.len() > 500000, "Expected large number of faces");
    
    println!("Successfully parsed {} points and {} faces", 
             surface.definition.points.len(), 
             surface.definition.faces.len());
}

#[test]
fn test_sample_xml_point_validation() {
    let sample_path = Path::new("tests/sample.xml");
    
    if !sample_path.exists() {
        println!("Skipping test - sample.xml not found");
        return;
    }
    
    let parser = LandXMLParser::from_file(sample_path).expect("Failed to create parser");
    let landxml = parser.parse().expect("Failed to parse real LandXML");
    
    let surface = &landxml.surfaces[0];
    let points = &surface.definition.points;
    
    // 最初の数点の座標値を検証
    assert!(points.len() >= 3);
    
    let p1 = &points[0];
    assert_eq!(p1.id, Some(1));
    assert!((p1.x - 78071.5).abs() < 0.001);
    assert!((p1.y - (-31992.5)).abs() < 0.001);
    assert!((p1.z - 34.1).abs() < 0.001);
    
    let p2 = &points[1];
    assert_eq!(p2.id, Some(2));
    assert!((p2.x - 78073.5).abs() < 0.001);
    assert!((p2.y - (-31992.5)).abs() < 0.001);
    assert!((p2.z - 34.0).abs() < 0.001);
    
    // 座標値の範囲チェック
    for point in points.iter().take(100) { // 最初の100点だけチェック（パフォーマンス考慮）
        assert!(point.x > 70000.0 && point.x < 100000.0, "X coordinate out of expected range");
        assert!(point.y > -40000.0 && point.y < 0.0, "Y coordinate out of expected range");  
        assert!(point.z > 30.0 && point.z < 40.0, "Z coordinate out of expected range");
    }
}

#[test]
fn test_sample_xml_face_validation() {
    let sample_path = Path::new("tests/sample.xml");
    
    if !sample_path.exists() {
        println!("Skipping test - sample.xml not found");
        return;
    }
    
    let parser = LandXMLParser::from_file(sample_path).expect("Failed to create parser");
    let landxml = parser.parse().expect("Failed to parse real LandXML");
    
    let surface = &landxml.surfaces[0];
    let points = &surface.definition.points;
    let faces = &surface.definition.faces;
    
    // 面データの妥当性チェック
    assert!(faces.len() > 0);
    
    // 最初の数面のインデックスが有効範囲内であることを確認
    for face in faces.iter().take(100) { // 最初の100面だけチェック
        assert!(face.p1 < points.len(), "Face p1 index out of bounds");
        assert!(face.p2 < points.len(), "Face p2 index out of bounds");
        assert!(face.p3 < points.len(), "Face p3 index out of bounds");
        
        // 同じ点を参照していないかチェック
        assert_ne!(face.p1, face.p2, "Face has duplicate vertices");
        assert_ne!(face.p2, face.p3, "Face has duplicate vertices");
        assert_ne!(face.p3, face.p1, "Face has duplicate vertices");
    }
}

#[test]
fn test_sample_xml_performance() {
    let sample_path = Path::new("tests/sample.xml");
    
    if !sample_path.exists() {
        println!("Skipping test - sample.xml not found");
        return;
    }
    
    let start = std::time::Instant::now();
    
    let parser = LandXMLParser::from_file(sample_path).expect("Failed to create parser");
    let landxml = parser.parse().expect("Failed to parse real LandXML");
    
    let duration = start.elapsed();
    
    println!("Parsing took: {:?}", duration);
    println!("Parsed {} points and {} faces", 
             landxml.surfaces[0].definition.points.len(),
             landxml.surfaces[0].definition.faces.len());
    
    // パフォーマンス要件: 大容量ファイルでも30秒以内に完了すること
    assert!(duration.as_secs() < 30, "Parsing took too long: {:?}", duration);
}

#[test] 
fn test_sample_xml_memory_usage() {
    let sample_path = Path::new("tests/sample.xml");
    
    if !sample_path.exists() {
        println!("Skipping test - sample.xml not found");
        return;
    }
    
    let parser = LandXMLParser::from_file(sample_path).expect("Failed to create parser");
    let landxml = parser.parse().expect("Failed to parse real LandXML");
    
    // メモリ使用量の概算（点と面のデータサイズ）
    let points_memory = landxml.surfaces[0].definition.points.len() * std::mem::size_of::<jp_landxml::Point3D>();
    let faces_memory = landxml.surfaces[0].definition.faces.len() * std::mem::size_of::<jp_landxml::Face>();
    let total_memory = points_memory + faces_memory;
    
    println!("Estimated memory usage: {} MB", total_memory / (1024 * 1024));
    
    // メモリ使用量が500MB以下であることを確認（大まかな目安）
    assert!(total_memory < 500 * 1024 * 1024, "Memory usage too high: {} MB", total_memory / (1024 * 1024));
}