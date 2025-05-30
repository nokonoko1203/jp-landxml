use jp_landxml::{LandXML, Surface, SurfaceType, SurfaceDefinition, Point3D, Face};
use jp_landxml::export::{export_to_json, export_debug_info};

#[test]
fn test_json_export() {
    let landxml = create_sample_landxml();
    
    let json_result = export_to_json(&landxml);
    assert!(json_result.is_ok());
    
    let json = json_result.unwrap();
    assert!(json.contains("\"version\": \"1.2\""));
    assert!(json.contains("\"surfaces\""));
    assert!(json.contains("ExistingGround"));
}

#[test]
fn test_debug_info_export() {
    let landxml = create_sample_landxml();
    
    let debug_info = export_debug_info(&landxml);
    
    assert!(debug_info.contains("LandXML Version: 1.2"));
    assert!(debug_info.contains("Surfaces: 1"));
    assert!(debug_info.contains("Alignments: 0"));
    assert!(debug_info.contains("Features: 0"));
    assert!(debug_info.contains("Surface 1: Test Surface"));
}

#[test]
fn test_empty_landxml_export() {
    let landxml = LandXML {
        version: "1.2".to_string(),
        coordinate_system: None,
        surfaces: Vec::new(),
        alignments: Vec::new(),
        features: Vec::new(),
    };
    
    let json_result = export_to_json(&landxml);
    assert!(json_result.is_ok());
    
    let debug_info = export_debug_info(&landxml);
    assert!(debug_info.contains("Surfaces: 0"));
    assert!(debug_info.contains("Alignments: 0"));
    assert!(debug_info.contains("Features: 0"));
}

fn create_sample_landxml() -> LandXML {
    let surface = Surface {
        name: "Test Surface".to_string(),
        surface_type: SurfaceType::ExistingGround,
        definition: SurfaceDefinition {
            points: vec![
                Point3D { x: 0.0, y: 0.0, z: 100.0, id: Some(1) },
                Point3D { x: 10.0, y: 0.0, z: 101.0, id: Some(2) },
                Point3D { x: 5.0, y: 10.0, z: 102.0, id: Some(3) },
            ],
            faces: vec![
                Face { p1: 0, p2: 1, p3: 2 },
            ],
        },
    };
    
    LandXML {
        version: "1.2".to_string(),
        coordinate_system: None,
        surfaces: vec![surface],
        alignments: Vec::new(),
        features: Vec::new(),
    }
}