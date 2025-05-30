use crate::error::{LandXMLError, Result};
use crate::models::*;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::collections::HashMap;

pub struct LandXMLParser {
    reader: Reader<BufReader<File>>,
}

impl LandXMLParser {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = File::open(path)?;
        let buf_reader = BufReader::new(file);
        let mut reader = Reader::from_reader(buf_reader);
        reader.trim_text(true);
        
        Ok(LandXMLParser { reader })
    }
    
    pub fn parse(mut self) -> Result<LandXML> {
        let mut buf = Vec::new();
        let mut landxml = LandXML {
            version: String::new(),
            coordinate_system: None,
            surfaces: Vec::new(),
            alignments: Vec::new(),
            features: Vec::new(),
        };
        
        let mut in_landxml = false;
        
        loop {
            match self.reader.read_event_into(&mut buf)? {
                Event::Start(ref e) => {
                    match e.name().as_ref() {
                        b"LandXML" => {
                            in_landxml = true;
                            for attr in e.attributes() {
                                let attr = attr?;
                                if attr.key.as_ref() == b"version" {
                                    landxml.version = String::from_utf8_lossy(&attr.value).to_string();
                                }
                            }
                        }
                        b"CoordinateSystem" if in_landxml => {
                            landxml.coordinate_system = Some(self.parse_coordinate_system(e)?);
                        }
                        b"Surface" if in_landxml => {
                            let surface = self.parse_surface(e)?;
                            landxml.surfaces.push(surface);
                        }
                        b"Alignment" if in_landxml => {
                            let alignment = self.parse_alignment(e)?;
                            landxml.alignments.push(alignment);
                        }
                        b"Feature" if in_landxml => {
                            let feature = self.parse_feature(e)?;
                            landxml.features.push(feature);
                        }
                        _ => {}
                    }
                }
                Event::End(ref e) => {
                    if e.name().as_ref() == b"LandXML" {
                        break;
                    }
                }
                Event::Eof => break,
                _ => {}
            }
            buf.clear();
        }
        
        if landxml.version.is_empty() {
            return Err(LandXMLError::InvalidFormat {
                message: "No LandXML version found".to_string(),
            });
        }
        
        Ok(landxml)
    }
    
    fn parse_coordinate_system(&mut self, start_element: &quick_xml::events::BytesStart) -> Result<CoordinateSystem> {
        let mut name = String::new();
        let mut epsg_code = None;
        let mut proj4_string = None;
        
        for attr in start_element.attributes() {
            let attr = attr?;
            match attr.key.as_ref() {
                b"name" => name = String::from_utf8_lossy(&attr.value).to_string(),
                b"epsgCode" => epsg_code = Some(String::from_utf8_lossy(&attr.value).to_string()),
                b"proj4String" => proj4_string = Some(String::from_utf8_lossy(&attr.value).to_string()),
                _ => {}
            }
        }
        
        Ok(CoordinateSystem {
            name,
            epsg_code,
            proj4_string,
        })
    }
    
    fn parse_surface(&mut self, start_element: &quick_xml::events::BytesStart) -> Result<Surface> {
        let mut name = String::new();
        let mut surface_type = SurfaceType::Other("Unknown".to_string());
        
        for attr in start_element.attributes() {
            let attr = attr?;
            match attr.key.as_ref() {
                b"name" => name = String::from_utf8_lossy(&attr.value).to_string(),
                b"desc" => {
                    let desc = String::from_utf8_lossy(&attr.value);
                    surface_type = match desc.as_ref() {
                        "ExistingGround" => SurfaceType::ExistingGround,
                        "DesignGround" => SurfaceType::DesignGround,
                        _ => SurfaceType::Other(desc.to_string()),
                    };
                }
                _ => {}
            }
        }
        
        let mut buf = Vec::new();
        let mut points = Vec::new();
        let mut faces = Vec::new();
        let mut in_definition = false;
        let mut in_points = false;
        let mut in_faces = false;
        
        loop {
            match self.reader.read_event_into(&mut buf)? {
                Event::Start(ref e) => {
                    match e.name().as_ref() {
                        b"Definition" => in_definition = true,
                        b"Pnts" if in_definition => in_points = true,
                        b"Faces" if in_definition => in_faces = true,
                        b"P" if in_points => {
                            let point = self.parse_point(e)?;
                            points.push(point);
                        }
                        b"F" if in_faces => {
                            let face = self.parse_face()?;
                            faces.push(face);
                        }
                        _ => {}
                    }
                }
                Event::End(ref e) => {
                    match e.name().as_ref() {
                        b"Surface" => break,
                        b"Definition" => in_definition = false,
                        b"Pnts" => in_points = false,
                        b"Faces" => in_faces = false,
                        _ => {}
                    }
                }
                Event::Eof => break,
                _ => {}
            }
            buf.clear();
        }
        
        Ok(Surface {
            name,
            surface_type,
            definition: SurfaceDefinition { points, faces },
        })
    }
    
    fn parse_point(&mut self, start_element: &quick_xml::events::BytesStart) -> Result<Point3D> {
        let mut id = None;
        
        for attr in start_element.attributes() {
            let attr = attr?;
            if attr.key.as_ref() == b"id" {
                id = Some(String::from_utf8_lossy(&attr.value).to_string());
            }
        }
        
        let mut buf = Vec::new();
        let mut point_data = String::new();
        
        loop {
            match self.reader.read_event_into(&mut buf)? {
                Event::Text(ref e) => {
                    point_data = String::from_utf8_lossy(&e).trim().to_string();
                }
                Event::End(ref e) => {
                    if e.name().as_ref() == b"P" {
                        break;
                    }
                }
                Event::Eof => break,
                _ => {}
            }
            buf.clear();
        }
        
        let coords: Vec<&str> = point_data.split_whitespace().collect();
        if coords.len() != 3 {
            return Err(LandXMLError::InvalidFormat {
                message: format!("Invalid point coordinates: {}", point_data),
            });
        }
        
        let x = coords[0].parse::<f64>().map_err(|_| LandXMLError::InvalidFormat {
            message: format!("Invalid X coordinate: {}", coords[0]),
        })?;
        let y = coords[1].parse::<f64>().map_err(|_| LandXMLError::InvalidFormat {
            message: format!("Invalid Y coordinate: {}", coords[1]),
        })?;
        let z = coords[2].parse::<f32>().map_err(|_| LandXMLError::InvalidFormat {
            message: format!("Invalid Z coordinate: {}", coords[2]),
        })?;
        
        let id_u32 = id.as_ref().and_then(|s| s.parse::<u32>().ok());
        
        Ok(Point3D { x, y, z, id: id_u32 })
    }
    
    fn parse_face(&mut self) -> Result<Face> {
        let mut buf = Vec::new();
        let mut face_data = String::new();
        
        loop {
            match self.reader.read_event_into(&mut buf)? {
                Event::Text(ref e) => {
                    face_data = String::from_utf8_lossy(&e).trim().to_string();
                }
                Event::End(ref e) => {
                    if e.name().as_ref() == b"F" {
                        break;
                    }
                }
                Event::Eof => break,
                _ => {}
            }
            buf.clear();
        }
        
        let indices: Vec<&str> = face_data.split_whitespace().collect();
        if indices.len() != 3 {
            return Err(LandXMLError::InvalidFormat {
                message: format!("Invalid face indices: {}", face_data),
            });
        }
        
        let p1 = indices[0].parse::<usize>().map_err(|_| LandXMLError::InvalidFormat {
            message: format!("Invalid face index: {}", indices[0]),
        })? - 1; // 1-based to 0-based
        let p2 = indices[1].parse::<usize>().map_err(|_| LandXMLError::InvalidFormat {
            message: format!("Invalid face index: {}", indices[1]),
        })? - 1;
        let p3 = indices[2].parse::<usize>().map_err(|_| LandXMLError::InvalidFormat {
            message: format!("Invalid face index: {}", indices[2]),
        })? - 1;
        
        Ok(Face { p1, p2, p3 })
    }
    
    fn parse_alignment(&mut self, start_element: &quick_xml::events::BytesStart) -> Result<Alignment> {
        let mut name = String::new();
        
        for attr in start_element.attributes() {
            let attr = attr?;
            if attr.key.as_ref() == b"name" {
                name = String::from_utf8_lossy(&attr.value).to_string();
            }
        }
        
        // 簡易実装 - 実際のAlignment要素をスキップ
        let mut buf = Vec::new();
        let mut depth = 1;
        
        loop {
            match self.reader.read_event_into(&mut buf)? {
                Event::Start(ref e) => {
                    if e.name().as_ref() == b"Alignment" {
                        depth += 1;
                    }
                }
                Event::End(ref e) => {
                    if e.name().as_ref() == b"Alignment" {
                        depth -= 1;
                        if depth == 0 {
                            break;
                        }
                    }
                }
                Event::Eof => break,
                _ => {}
            }
            buf.clear();
        }
        
        Ok(Alignment {
            name,
            coord_geom: CoordGeom {
                elements: Vec::new(),
            },
            profile: None,
            cross_sections: Vec::new(),
        })
    }
    
    fn parse_feature(&mut self, start_element: &quick_xml::events::BytesStart) -> Result<Feature> {
        let mut code = String::new();
        
        for attr in start_element.attributes() {
            let attr = attr?;
            if attr.key.as_ref() == b"code" {
                code = String::from_utf8_lossy(&attr.value).to_string();
            }
        }
        
        // 簡易実装 - 実際のFeature要素をスキップ
        let mut buf = Vec::new();
        let mut depth = 1;
        
        loop {
            match self.reader.read_event_into(&mut buf)? {
                Event::Start(ref e) => {
                    if e.name().as_ref() == b"Feature" {
                        depth += 1;
                    }
                }
                Event::End(ref e) => {
                    if e.name().as_ref() == b"Feature" {
                        depth -= 1;
                        if depth == 0 {
                            break;
                        }
                    }
                }
                Event::Eof => break,
                _ => {}
            }
            buf.clear();
        }
        
        Ok(Feature {
            code,
            properties: HashMap::new(),
            geometry: None,
        })
    }
}