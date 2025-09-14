//! Tool discovery system for MCP (Model Context Protocol) servers.
//!
//! This module provides functionality for discovering, parsing, and validating
//! tool definitions from executable files and associated metadata sources.
//!
//! The module is organized into separate concerns:
//! - [`definitions`]: Tool definition data structures (MCP protocol and mcp-serve YAML format)
//! - [`scanner`]: Directory scanning and executable discovery functionality
//!
//! # Examples
//!
//! ## Discovering tools in a directory
//!
//! ```
//! use mcp_serve::tool_discovery::scanner::DirectoryScanner;
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
//! use mcp_serve::tool_discovery::definitions::ToolDefinition;
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

pub mod definitions;
pub mod scanner;

// Re-export commonly used types for convenience
pub use definitions::{McpTool, ToolDefinition, ToolInput, ToolOutput};
pub use scanner::{DirectoryScanner, DiscoveredTool, MetadataSource, ScanError};
