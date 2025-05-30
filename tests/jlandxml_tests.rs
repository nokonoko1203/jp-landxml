/// J-LandXML Ver.1.6 機能テスト
use jp_landxml::{
    CoordinateSystemNameParser, JLandXml, JLandXmlParser, JapanPlaneCoordinateSystem,
};

#[test]
fn test_jlandxml_sample_parsing() {
    let parser =
        JLandXmlParser::from_file("tests/jlandxml_sample.xml").expect("Failed to create parser");

    let jlandxml_doc = parser.parse().expect("Failed to parse J-LandXML");

    // J-LandXML判定
    assert!(
        jlandxml_doc.is_j_landxml(),
        "Should be recognized as J-LandXML"
    );

    // バージョン確認
    assert_eq!(jlandxml_doc.j_landxml_version, Some("1.6".to_string()));
    assert_eq!(
        jlandxml_doc.application_criterion,
        Some("MlitLandXmlVer1.6".to_string())
    );

    // 座標系確認
    let coordinate_system = jlandxml_doc
        .coordinate_system
        .expect("Coordinate system should be present");

    assert_eq!(
        coordinate_system.horizontal_coordinate_system_name,
        "9(X,Y)".to_string()
    );

    assert_eq!(
        coordinate_system.plane_coordinate_zone,
        Some(JapanPlaneCoordinateSystem::Zone9)
    );

    assert_eq!(coordinate_system.get_plane_coordinate_epsg(), Some(6677));

    // 基本データ確認
    assert_eq!(jlandxml_doc.base.version, "1.2");
    assert_eq!(jlandxml_doc.base.surfaces.len(), 2);
    assert_eq!(jlandxml_doc.base.alignments.len(), 1);

    println!("✅ J-LandXML parsing test passed");
}

#[test]
fn test_coordinate_system_name_parsing() {
    // 正常なケース
    let result = CoordinateSystemNameParser::parse("9(X,Y)").expect("Should parse successfully");
    assert_eq!(result, Some(JapanPlaneCoordinateSystem::Zone9));

    let result = CoordinateSystemNameParser::parse("1(X,Y)").expect("Should parse successfully");
    assert_eq!(result, Some(JapanPlaneCoordinateSystem::Zone1));

    let result = CoordinateSystemNameParser::parse("19(X,Y)").expect("Should parse successfully");
    assert_eq!(result, Some(JapanPlaneCoordinateSystem::Zone19));

    // 無効なケース
    let result = CoordinateSystemNameParser::parse("20(X,Y)").expect("Should parse without error");
    assert_eq!(result, None);

    let result = CoordinateSystemNameParser::parse("invalid").expect("Should parse without error");
    assert_eq!(result, None);

    println!("✅ Coordinate system name parsing test passed");
}

#[test]
fn test_japan_plane_coordinate_systems() {
    // 全座標系のテスト
    let zones = JapanPlaneCoordinateSystem::all_zones();
    assert_eq!(zones.len(), 19);

    // 個別のテスト
    let zone9 = JapanPlaneCoordinateSystem::Zone9;
    assert_eq!(zone9.zone_number(), 9);
    assert_eq!(zone9.epsg_code(), 6677);
    assert!(zone9.description().contains("東京都"));

    let zone1 = JapanPlaneCoordinateSystem::Zone1;
    assert_eq!(zone1.zone_number(), 1);
    assert_eq!(zone1.epsg_code(), 6669);
    assert!(zone1.description().contains("長崎県"));

    // 系番号からの変換
    let zone_from_num =
        JapanPlaneCoordinateSystem::from_zone_number(9).expect("Should convert successfully");
    assert_eq!(zone_from_num, JapanPlaneCoordinateSystem::Zone9);

    // 無効な系番号
    let invalid_zone = JapanPlaneCoordinateSystem::from_zone_number(20);
    assert!(invalid_zone.is_err());

    println!("✅ Japan plane coordinate systems test passed");
}

#[test]
fn test_jlandxml_unified_api() {
    // 統一APIのテスト
    let result =
        JLandXml::parse_coordinate_system_name("9(X,Y)").expect("Should parse successfully");
    assert_eq!(result, Some(JapanPlaneCoordinateSystem::Zone9));

    let epsg_code = JLandXml::get_epsg_code(JapanPlaneCoordinateSystem::Zone9);
    assert_eq!(epsg_code, 6677);

    let description = JLandXml::get_zone_description(JapanPlaneCoordinateSystem::Zone9);
    assert!(description.contains("東京都"));

    println!("✅ J-LandXML unified API test passed");
}

#[test]
fn test_coordinate_system_validation() {
    assert!(CoordinateSystemNameParser::validate("1(X,Y)"));
    assert!(CoordinateSystemNameParser::validate("9(X,Y)"));
    assert!(CoordinateSystemNameParser::validate("19(X,Y)"));

    assert!(!CoordinateSystemNameParser::validate("0(X,Y)"));
    assert!(!CoordinateSystemNameParser::validate("20(X,Y)"));
    assert!(!CoordinateSystemNameParser::validate("9(X,Z)"));
    assert!(!CoordinateSystemNameParser::validate("invalid"));

    println!("✅ Coordinate system validation test passed");
}

#[test]
fn test_coordinate_system_info() {
    let info =
        CoordinateSystemNameParser::get_info("9(X,Y)").expect("Should get info successfully");
    assert!(info.is_some());

    let info_str = info.unwrap();
    assert!(info_str.contains("平面直角座標系9系"));
    assert!(info_str.contains("6677"));

    let invalid_info =
        CoordinateSystemNameParser::get_info("invalid").expect("Should handle invalid input");
    assert!(invalid_info.is_none());

    println!("✅ Coordinate system info test passed");
}
