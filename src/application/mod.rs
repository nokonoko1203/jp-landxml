//! Application module for J-LandXML parser
//!
//! This module handles application metadata:
//! - Creator application name and version
//! - Creation timestamp

use serde::{Deserialize, Serialize};

/// Application that created the LandXML document
///
/// Reference: LandXML 1.2 Application element
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Application {
    /// Application name
    pub name: Option<String>,
    /// Manufacturer name
    pub manufacturer: Option<String>,
    /// Application version
    pub version: Option<String>,
    /// Manufacturer URL
    pub manufacturer_url: Option<String>,
    /// Creation timestamp
    pub time_stamp: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_application_creation() {
        let app = Application {
            name: Some("TestApp".to_string()),
            version: Some("1.0.0".to_string()),
            ..Default::default()
        };
        assert_eq!(app.name, Some("TestApp".to_string()));
    }
}
