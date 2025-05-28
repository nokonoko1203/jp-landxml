use thiserror::Error;

#[derive(Error, Debug)]
pub enum LandXMLError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("XML parse error: {0}")]
    XmlParse(#[from] quick_xml::Error),
    
    #[error("XML attribute error: {0}")]
    XmlAttribute(#[from] quick_xml::events::attributes::AttrError),
    
    #[error("Serde JSON error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("Invalid LandXML format: {message}")]
    InvalidFormat { message: String },
    
    #[error("Unsupported version: {version}")]
    UnsupportedVersion { version: String },
    
    #[error("Missing required element: {element}")]
    MissingElement { element: String },
    
    #[error("Invalid coordinate system: {system}")]
    InvalidCoordinateSystem { system: String },
    
    #[error("Geometry calculation error: {message}")]
    GeometryError { message: String },
}

pub type Result<T> = std::result::Result<T, LandXMLError>;