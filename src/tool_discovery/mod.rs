//! Tool discovery system for MCP (Model Context Protocol) servers.
//!
//! This module provides functionality for discovering, parsing, and validating
//! tool definitions from executable files and associated metadata sources.
//!
//! The functionality is organized into top-level modules:
//! - [`crate::definitions`]: Tool definition data structures (MCP protocol and mcp-serve YAML format)
//! - [`crate::scanner`]: Directory scanning and executable discovery functionality
//!
//! # Examples
//!
//! ## Discovering tools in a directory
//!
//! ```
//! use mcp_serve::scanner::DirectoryScanner;
//! use std::path::Path;
//!
//! let mut scanner = DirectoryScanner::new();
//! let tools = scanner.scan_directory(Path::new("./examples/tools")).unwrap();
//!
//! for tool in tools {
//!     println!("Found tool: {}", tool.executable_path.display());
//! }
//! ```
//!
//! ## Parsing tool definitions from YAML
//!
//! ```
//! use mcp_serve::definitions::ToolDefinition;
//!
//! let yaml = r#"
//! name: example_tool
//! description: An example tool
//! input:
//!   template: "--name {{name}}"
//!   schema:
//!     type: object
//!     properties:
//!       name:
//!         type: string
//! output:
//!   template: "Result: (?<result>.*)"
//!   schema:
//!     type: object
//!     properties:
//!       result:
//!         type: string
//! "#;
//!
//! let tool = ToolDefinition::from_yaml(yaml).unwrap();
//! let mcp_tool = tool.to_mcp_tool();
//! ```

// Note: Tool discovery functionality has been moved to top-level modules.
// Import directly from mcp_serve::definitions and mcp_serve::scanner instead of this module.
