/// J-LandXML Ver.1.6 完全仕様対応 統合テスト

use jp_landxml::{
    JLandXmlCoordinateSystem, HorizontalDatum, VerticalDatum,
    JapanPlaneCoordinateSystem, CoordinateSystemValidator, ValidationWarning
};

#[test]
fn test_full_spec_coordinate_system_creation() {
    // 完全仕様による座標系作成（淀川・6系の例）
    let coord_system = JLandXmlCoordinateSystem::new(
        "Yodo-CRS".to_string(),
        HorizontalDatum::JGD2011,
        VerticalDatum::OP,
        "6(X,Y)".to_string(),
    ).expect("Should create coordinate system successfully");
    
    // 基本属性の確認
    assert_eq!(coord_system.name, "Yodo-CRS");
    assert_eq!(coord_system.horizontal_datum, HorizontalDatum::JGD2011);
    assert_eq!(coord_system.vertical_datum, VerticalDatum::OP);
    assert_eq!(coord_system.horizontal_coordinate_system_name, "6(X,Y)");
    
    // 自動設定の確認
    assert_eq!(coord_system.plane_coordinate_zone, Some(JapanPlaneCoordinateSystem::Zone6));
    assert_eq!(coord_system.differ_tp, Some(-1.3000)); // O.PのT.P差分
    assert_eq!(coord_system.get_plane_coordinate_epsg(), Some(6674)); // 6系のEPSG
    
    println!("✅ Full spec coordinate system creation test passed");
}

#[test]
fn test_all_horizontal_datums() {
    let datums = vec![
        ("JGD2000", HorizontalDatum::JGD2000),
        ("JGD2011", HorizontalDatum::JGD2011),
        ("TD", HorizontalDatum::TD),
    ];
    
    for (str_val, expected) in datums {
        let parsed = HorizontalDatum::from_str(str_val)
            .expect(&format!("Should parse {} successfully", str_val));
        assert_eq!(parsed, expected);
        assert_eq!(parsed.as_str(), str_val);
        
        // 説明の確認
        assert!(!parsed.description().is_empty());
    }
    
    // 無効な値のテスト
    assert!(HorizontalDatum::from_str("INVALID").is_err());
    
    println!("✅ All horizontal datums test passed");
}

#[test]
fn test_all_vertical_datums() {
    let datums_with_offsets = vec![
        ("T.P", VerticalDatum::TP, 0.0),
        ("K.P", VerticalDatum::KP, -0.8745),
        ("S.P", VerticalDatum::SP, -0.0873),
        ("Y.P", VerticalDatum::YP, -0.8402),
        ("A.P", VerticalDatum::AP, -1.1344),
        ("O.P", VerticalDatum::OP, -1.3000),
        ("T.P.W", VerticalDatum::TPW, 0.113),
        ("B.S.L", VerticalDatum::BSL, 84.371),
    ];
    
    for (str_val, expected, expected_offset) in datums_with_offsets {
        let parsed = VerticalDatum::from_str(str_val)
            .expect(&format!("Should parse {} successfully", str_val));
        assert_eq!(parsed, expected);
        assert_eq!(parsed.as_str(), str_val);
        assert_eq!(parsed.tp_offset(), expected_offset);
        
        // 説明の確認
        assert!(!parsed.description().is_empty());
        
        // T.P基準への変換テスト
        let test_elevation = 10.0;
        let tp_elevation = parsed.to_tp_elevation(test_elevation);
        assert_eq!(tp_elevation, test_elevation + expected_offset);
    }
    
    // 無効な値のテスト
    assert!(VerticalDatum::from_str("INVALID").is_err());
    
    println!("✅ All vertical datums test passed");
}

#[test]
fn test_coordinate_system_validation() {
    // 正常ケース：JGD2011 + O.P + 6系
    let warnings = CoordinateSystemValidator::validate_complete_system(
        HorizontalDatum::JGD2011,
        VerticalDatum::OP,
        JapanPlaneCoordinateSystem::Zone6,
        Some(-1.3000),
    ).expect("Should validate successfully");
    
    assert_eq!(warnings.len(), 0, "Should have no warnings for correct setup");
    
    // differTP不一致ケース
    let warnings = CoordinateSystemValidator::validate_complete_system(
        HorizontalDatum::JGD2011,
        VerticalDatum::OP,
        JapanPlaneCoordinateSystem::Zone6,
        Some(-1.0000), // 正しくは-1.3000
    ).expect("Should validate successfully");
    
    assert_eq!(warnings.len(), 1);
    assert!(matches!(warnings[0], ValidationWarning::DifferTpMismatch { .. }));
    
    // differTP未設定ケース
    let warnings = CoordinateSystemValidator::validate_complete_system(
        HorizontalDatum::JGD2011,
        VerticalDatum::OP,
        JapanPlaneCoordinateSystem::Zone6,
        None, // 本来は-1.3000が必要
    ).expect("Should validate successfully");
    
    assert_eq!(warnings.len(), 1);
    assert!(matches!(warnings[0], ValidationWarning::MissingDifferTp { .. }));
    
    // T.P基準で不要なdifferTPケース
    let warnings = CoordinateSystemValidator::validate_complete_system(
        HorizontalDatum::JGD2011,
        VerticalDatum::TP,
        JapanPlaneCoordinateSystem::Zone9,
        Some(1.0), // T.P基準なので不要
    ).expect("Should validate successfully");
    
    assert_eq!(warnings.len(), 1);
    assert!(matches!(warnings[0], ValidationWarning::UnnecessaryDifferTp));
    
    // 旧測地系使用ケース
    let warnings = CoordinateSystemValidator::validate_complete_system(
        HorizontalDatum::TD, // 旧日本測地系
        VerticalDatum::TP,
        JapanPlaneCoordinateSystem::Zone9,
        None,
    ).expect("Should validate successfully");
    
    assert_eq!(warnings.len(), 1);
    assert!(matches!(warnings[0], ValidationWarning::LegacyDatumUsage { .. }));
    
    println!("✅ Coordinate system validation test passed");
}

#[test]
fn test_coordinate_system_builder_pattern() {
    // ビルダーパターンのテスト
    let coord_system = JLandXmlCoordinateSystem::new(
        "Test-CRS".to_string(),
        HorizontalDatum::JGD2011,
        VerticalDatum::TP,
        "9(X,Y)".to_string(),
    ).expect("Should create successfully")
        .with_description("東京都心部の座標系".to_string())
        .with_geoid_name("GSIGEO2011".to_string());
    
    assert_eq!(coord_system.desc, Some("東京都心部の座標系".to_string()));
    assert_eq!(coord_system.geoid_name, Some("GSIGEO2011".to_string()));
    assert_eq!(coord_system.plane_coordinate_zone, Some(JapanPlaneCoordinateSystem::Zone9));
    assert_eq!(coord_system.differ_tp, None); // T.P基準なので補正不要
    
    println!("✅ Coordinate system builder pattern test passed");
}

#[test] 
fn test_tp_elevation_conversion() {
    // 各高さ系からT.P基準への変換テスト
    let test_cases = vec![
        (VerticalDatum::TP, 10.0, 10.0),      // T.P基準：変換なし
        (VerticalDatum::OP, 10.0, 8.7),       // O.P基準：10.0 + (-1.3) = 8.7
        (VerticalDatum::KP, 10.0, 9.1255),    // K.P基準：10.0 + (-0.8745) = 9.1255
        (VerticalDatum::BSL, 10.0, 94.371),   // B.S.L基準：10.0 + 84.371 = 94.371
    ];
    
    for (vertical_datum, raw_elevation, expected_tp) in test_cases {
        let coord_system = JLandXmlCoordinateSystem::new(
            "Test".to_string(),
            HorizontalDatum::JGD2011,
            vertical_datum,
            "9(X,Y)".to_string(),
        ).expect("Should create successfully");
        
        let tp_elevation = coord_system.to_tp_elevation(raw_elevation);
        assert_eq!(tp_elevation, expected_tp, 
                   "T.P conversion failed for {:?}: {} -> {}", 
                   vertical_datum, raw_elevation, tp_elevation);
    }
    
    println!("✅ T.P elevation conversion test passed");
}

#[test]
fn test_coordinate_system_info() {
    let coord_system = JLandXmlCoordinateSystem::new(
        "Test-Info".to_string(),
        HorizontalDatum::JGD2011,
        VerticalDatum::OP,
        "6(X,Y)".to_string(),
    ).expect("Should create successfully")
        .with_description("情報取得テスト".to_string());
    
    let info = coord_system.get_coordinate_system_info();
    
    assert_eq!(info.name, "Test-Info");
    assert_eq!(info.desc, Some("情報取得テスト".to_string()));
    assert_eq!(info.horizontal_datum, HorizontalDatum::JGD2011);
    assert_eq!(info.vertical_datum, VerticalDatum::OP);
    assert_eq!(info.horizontal_coordinate_system_name, "6(X,Y)");
    assert_eq!(info.differ_tp, Some(-1.3000));
    assert_eq!(info.plane_coordinate_zone, Some(JapanPlaneCoordinateSystem::Zone6));
    assert_eq!(info.epsg_code, Some(6674));
    
    println!("✅ Coordinate system info test passed");
}

#[test]
fn test_validation_warning_display() {
    // バリデーション警告のDisplay実装テスト
    let warnings = vec![
        ValidationWarning::DifferTpMismatch {
            vertical_datum: VerticalDatum::OP,
            provided: -1.0,
            expected: -1.3,
        },
        ValidationWarning::MissingDifferTp {
            vertical_datum: VerticalDatum::OP,
            expected: -1.3,
        },
        ValidationWarning::UnnecessaryDifferTp,
        ValidationWarning::LegacyDatumUsage {
            datum: HorizontalDatum::TD,
        },
    ];
    
    for warning in &warnings {
        let display_str = format!("{}", warning);
        assert!(!display_str.is_empty(), "Warning display should not be empty");
        println!("Warning: {}", display_str);
    }
    
    println!("✅ Validation warning display test passed");
}

#[test]
fn test_all_plane_coordinate_zones_with_complete_spec() {
    // 全19系の平面直角座標系を完全仕様でテスト
    for zone_num in 1..=19 {
        let zone = JapanPlaneCoordinateSystem::from_zone_number(zone_num)
            .expect(&format!("Zone {} should be valid", zone_num));
        
        let coord_name = format!("{}(X,Y)", zone_num);
        let coord_system = JLandXmlCoordinateSystem::new(
            format!("Test-Zone{}", zone_num),
            HorizontalDatum::JGD2011,
            VerticalDatum::TP,
            coord_name.clone(),
        ).expect(&format!("Should create coordinate system for zone {}", zone_num));
        
        assert_eq!(coord_system.plane_coordinate_zone, Some(zone));
        assert_eq!(coord_system.horizontal_coordinate_system_name, coord_name);
        assert_eq!(coord_system.get_plane_coordinate_epsg(), Some(zone.epsg_code()));
        
        // 各系の説明が適切に設定されているかチェック
        assert!(!zone.description().is_empty());
        assert!(zone.epsg_code() >= 6669 && zone.epsg_code() <= 6687);
    }
    
    println!("✅ All plane coordinate zones with complete spec test passed");
}