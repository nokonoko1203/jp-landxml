//! Project module for J-LandXML parser
//!
//! This module handles project metadata including:
//! - Project name and description
//! - Project classification (road, river, etc.)
//! - Phase information

use serde::{Deserialize, Serialize};

/// Project information
///
/// Reference: LandXML 1.2 Project element
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Project {
    /// Project name
    pub name: Option<String>,
    /// Project description
    pub desc: Option<String>,
    /// Project state (existing, proposed, etc.)
    pub state: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_creation() {
        let project = Project {
            name: Some("Test Project".to_string()),
            desc: Some("A test project".to_string()),
            state: Some("proposed".to_string()),
        };
        assert_eq!(project.name, Some("Test Project".to_string()));
    }
}
