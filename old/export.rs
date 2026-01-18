use crate::error::Result;
use crate::models::LandXML;
use serde_json;

pub fn export_to_json(landxml: &LandXML) -> Result<String> {
    let json = serde_json::to_string_pretty(landxml)?;
    Ok(json)
}

pub fn export_debug_info(landxml: &LandXML) -> String {
    let mut info = String::new();

    info.push_str(&format!("LandXML Version: {}\n", landxml.version));
    info.push_str(&format!("Surfaces: {}\n", landxml.surfaces.len()));
    info.push_str(&format!("Alignments: {}\n", landxml.alignments.len()));
    info.push_str(&format!("Features: {}\n", landxml.features.len()));

    if let Some(ref coord_sys) = landxml.coordinate_system {
        info.push_str(&format!("Coordinate System: {}\n", coord_sys.name));
    }

    for (i, surface) in landxml.surfaces.iter().enumerate() {
        info.push_str(&format!(
            "  Surface {}: {} (type: {:?})\n",
            i + 1,
            surface.name,
            surface.surface_type
        ));
        info.push_str(&format!(
            "    Points: {}, Faces: {}\n",
            surface.definition.points.len(),
            surface.definition.faces.len()
        ));
    }

    for (i, alignment) in landxml.alignments.iter().enumerate() {
        info.push_str(&format!("  Alignment {}: {}\n", i + 1, alignment.name));
        info.push_str(&format!(
            "    Geometry elements: {}\n",
            alignment.coord_geom.elements.len()
        ));
        if alignment.profile.is_some() {
            info.push_str("    Has profile\n");
        }
        info.push_str(&format!(
            "    Cross sections: {}\n",
            alignment.cross_sections.len()
        ));
    }

    info
}
