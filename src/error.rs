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
    
    #[error("Invalid coordinate system: {0}")]
    InvalidCoordinateSystem(String),
    
    #[error("Geometry calculation error: {message}")]
    GeometryError { message: String },
    
    // DEM related errors
    #[error("GDAL error: {0}")]
    GdalError(String),
    
    #[error("Invalid grid index: row {row}, col {col} (max: {max_row}, {max_col})")]
    InvalidGridIndex { row: usize, col: usize, max_row: usize, max_col: usize },
    
    #[error("Invalid grid size: expected {expected}, got {actual}")]
    InvalidGridSize { expected: usize, actual: usize },
    
    #[error("Invalid resolution: x_res={x_res}, y_res={y_res}")]
    InvalidResolution { x_res: f64, y_res: f64 },
    
    #[error("Missing surface definition")]
    MissingSurfaceDefinition,
    
    #[error("Empty point cloud")]
    EmptyPointCloud,
    
    // J-LandXML specific errors
    #[error("Parse error: {0}")]
    ParseError(String),
    
    #[error("Unsupported J-LandXML version: {version}")]
    UnsupportedJLandXmlVersion { version: String },
    
    #[error("Invalid plane coordinate zone: {zone}")]
    InvalidPlaneCoordinateZone { zone: u8 },
    
    #[error("Missing horizontal coordinate system name")]
    MissingHorizontalCoordinateSystemName,
    
    #[error("Invalid coordinate system name format: {name}")]
    InvalidCoordinateSystemNameFormat { name: String },
    
    #[error("J-LandXML validation error: {message}")]
    JLandXmlValidationError { message: String },
}

pub type Result<T> = std::result::Result<T, LandXMLError>;