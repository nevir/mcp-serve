//! Tool discovery system for MCP (Model Context Protocol) servers.
//!
//! This module provides data structures and functionality for discovering,
//! parsing, and validating tool definitions from executable files and
//! associated metadata sources.
//!
//! The design separates pure MCP protocol structures from mcp-serve's custom
//! YAML format that includes templates for command-line argument generation
//! and output parsing.
//!
//! JSON schemas are represented as opaque `serde_json::Value` objects,
//! allowing for flexible schema definitions without needing to model
//! the entire JSON Schema specification.

use faccess::PathExt;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Pure MCP tool definition as specified in the Model Context Protocol.
///
/// This structure represents the exact MCP specification format and is used
/// when communicating with MCP clients. It contains no mcp-serve specific
/// extensions.
///
/// JSON schemas are represented as opaque `serde_json::Value` objects that can
/// contain any valid JSON Schema structure.
///
/// # Examples
///
/// ```
/// use mcp_serve::tool_discovery::McpTool;
/// use serde_json::json;
///
/// let tool = McpTool {
///     name: "calculate_sum".to_string(),
///     title: Some("Calculator".to_string()),
///     description: "Add two numbers together".to_string(),
///     input_schema: json!({
///         "type": "object",
///         "properties": {
///             "a": {"type": "number"},
///             "b": {"type": "number"}
///         },
///         "required": ["a", "b"]
///     }),
///     output_schema: None,
///     annotations: None,
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct McpTool {
    /// Unique identifier for the tool (required by MCP spec)
    pub name: String,

    /// Optional human-readable display name
    pub title: Option<String>,

    /// Human-readable description of functionality (required by MCP spec)
    pub description: String,

    /// JSON Schema for input parameters (required by MCP spec)
    ///
    /// This is an opaque JSON Schema object that can contain any valid
    /// JSON Schema structure for parameter validation.
    #[serde(rename = "input_schema")]
    pub input_schema: serde_json::Value,

    /// Optional JSON Schema for output structure
    ///
    /// When provided, tool outputs should conform to this schema structure.
    #[serde(rename = "output_schema")]
    pub output_schema: Option<serde_json::Value>,

    /// Optional metadata annotations
    pub annotations: Option<HashMap<String, serde_yaml_ng::Value>>,
}

/// mcp-serve tool definition with custom extensions for template-based execution.
///
/// This structure represents the YAML format used in mcp-serve tool definitions,
/// which includes templates for converting between JSON and command-line arguments
/// as well as parsing script output back to structured JSON.
///
/// The format differs from pure MCP by using `input: { schema, template }` instead
/// of `input_schema`, and adding `output: { schema, template }` for output parsing.
///
/// Both input and output are required since every tool needs to define its interface
/// and how to parse its results.
///
/// # Examples
///
/// ```
/// use mcp_serve::tool_discovery::ToolDefinition;
///
/// let yaml = r#"
/// name: create_ticket
/// title: Create Ticket
/// description: Creates a new feature ticket
/// input:
///   template: "--title {{title}} {{body}}"
///   schema:
///     type: object
///     properties:
///       title:
///         type: string
///       body:
///         type: string
///     required: ["title", "body"]
/// output:
///   template: "Created: (?<url>https://.*)"
///   schema:
///     type: object
///     properties:
///       url:
///         type: string
/// "#;
///
/// let tool = ToolDefinition::from_yaml(yaml).unwrap();
/// assert_eq!(tool.name, "create_ticket");
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolDefinition {
    /// Unique identifier for the tool
    pub name: String,

    /// Optional human-readable display name
    pub title: Option<String>,

    /// Human-readable description of the tool's functionality
    pub description: String,

    /// Input specification with schema and template (required)
    pub input: ToolInput,

    /// Output specification with schema and template (required)
    pub output: ToolOutput,

    /// Optional metadata annotations
    pub annotations: Option<HashMap<String, serde_yaml_ng::Value>>,
}

/// Input specification for mcp-serve tools.
///
/// Combines JSON Schema validation with template-based command-line generation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolInput {
    /// Template for converting JSON input to command-line arguments.
    ///
    /// Uses `{{property}}` for basic substitution, `[...]` for optional sections,
    /// and `[...item...]` for array repetition.
    ///
    /// # Examples
    ///
    /// - `"--title {{title}} {{body}}"` - Basic substitution
    /// - `"--title {{title}} [--parent {{parent_id}}]"` - Optional argument
    /// - `"[--label {{label}}...]"` - Repeated array items
    pub template: String,

    /// JSON Schema defining the input parameters
    ///
    /// This is an opaque JSON Schema object that can contain any valid
    /// JSON Schema structure for parameter validation.
    pub schema: serde_json::Value,
}

/// Output specification for mcp-serve tools.
///
/// Combines JSON Schema validation with regex-based output parsing.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolOutput {
    /// Regex template for parsing script output into JSON.
    ///
    /// Uses named capture groups `(?<name>pattern)` to extract values
    /// that become JSON properties.
    ///
    /// # Examples
    ///
    /// ```text
    /// Ticket created: (?<url>https://.*)\nID: (?<id>\d+)
    /// ```
    pub template: String,

    /// JSON Schema defining the output structure
    ///
    /// This is an opaque JSON Schema object that can contain any valid
    /// JSON Schema structure for result validation.
    pub schema: serde_json::Value,
}

impl ToolDefinition {
    /// Parse a tool definition from YAML string.
    ///
    /// This is the primary way to create `ToolDefinition` instances from
    /// YAML metadata found in script files or sidecar files.
    ///
    /// # Examples
    ///
    /// ```
    /// use mcp_serve::tool_discovery::ToolDefinition;
    ///
    /// let yaml = r#"
    /// name: example_tool
    /// description: An example tool
    /// input:
    ///   template: "--name {{name}}"
    ///   schema:
    ///     type: object
    ///     properties:
    ///       name:
    ///         type: string
    /// output:
    ///   template: "Result: (?<result>.*)"
    ///   schema:
    ///     type: object
    ///     properties:
    ///       result:
    ///         type: string
    /// "#;
    ///
    /// let tool = ToolDefinition::from_yaml(yaml).unwrap();
    /// assert_eq!(tool.name, "example_tool");
    /// ```
    pub fn from_yaml(yaml: &str) -> Result<Self, serde_yaml_ng::Error> {
        serde_yaml_ng::from_str(yaml)
    }

    /// Convert this mcp-serve tool definition to a pure MCP tool.
    ///
    /// This extracts the schema information and discards the template-specific
    /// extensions, creating a tool definition that conforms to the MCP specification.
    ///
    /// # Examples
    ///
    /// ```
    /// use mcp_serve::tool_discovery::{ToolDefinition, ToolInput, ToolOutput};
    /// use serde_json::json;
    ///
    /// let input = ToolInput {
    ///     template: "--name {{name}}".to_string(),
    ///     schema: json!({"type": "object"}),
    /// };
    ///
    /// let output = ToolOutput {
    ///     template: "Result: (?<value>.*)".to_string(),
    ///     schema: json!({"type": "string"}),
    /// };
    ///
    /// let tool = ToolDefinition {
    ///     name: "test".to_string(),
    ///     title: None,
    ///     description: "Test tool".to_string(),
    ///     input,
    ///     output,
    ///     annotations: None,
    /// };
    /// let mcp_tool = tool.to_mcp_tool();
    ///
    /// assert_eq!(mcp_tool.name, "test");
    /// assert_eq!(mcp_tool.description, "Test tool");
    /// ```
    pub fn to_mcp_tool(&self) -> McpTool {
        McpTool {
            name: self.name.clone(),
            title: self.title.clone(),
            description: self.description.clone(),
            input_schema: self.input.schema.clone(),
            output_schema: Some(self.output.schema.clone()),
            annotations: self.annotations.clone(),
        }
    }
}

/// Represents a discovered tool file and its associated metadata source.
///
/// Tools can have metadata embedded directly in the file or in a separate
/// sidecar `.yaml` file with the same name as the executable.
#[derive(Debug, Clone, PartialEq)]
pub struct DiscoveredTool {
    /// Path to the executable file
    pub executable_path: PathBuf,

    /// Path to the metadata source (embedded or sidecar file)
    pub metadata_source: MetadataSource,
}

/// Represents the source of tool metadata.
#[derive(Debug, Clone, PartialEq)]
pub enum MetadataSource {
    /// Metadata is embedded in the executable file itself
    Embedded(PathBuf),

    /// Metadata is in a sidecar `.yaml` file
    Sidecar(PathBuf),
}

/// Errors that can occur during directory scanning.
#[derive(Debug, thiserror::Error)]
pub enum ScanError {
    #[error("IO error scanning directory: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Permission denied accessing path: {path}")]
    PermissionDenied { path: PathBuf },
}

/// Scanner that traverses directories to discover potential tools.
///
/// The `DirectoryScanner` identifies executable files and their associated
/// metadata sources (embedded or sidecar YAML files) while handling
/// permission errors gracefully.
///
/// # Examples
///
/// ```
/// use mcp_serve::tool_discovery::DirectoryScanner;
/// use std::path::Path;
///
/// let mut scanner = DirectoryScanner::new();
/// // Use examples/tools if it exists, otherwise use current directory
/// let tools_dir = if Path::new("examples/tools").exists() {
///     Path::new("examples/tools")
/// } else {
///     Path::new(".")
/// };
/// let tools = scanner.scan_directory(tools_dir).unwrap();
///
/// for tool in tools {
///     println!("Found tool: {}", tool.executable_path.display());
/// }
/// ```
pub struct DirectoryScanner {
    /// Collected errors during scanning
    errors: Vec<ScanError>,
}

impl DirectoryScanner {
    /// Create a new directory scanner.
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    /// Scan a directory for discoverable tools.
    ///
    /// Returns all discovered tools and collects any permission errors
    /// encountered during scanning. Permission errors are stored internally
    /// and can be retrieved with `take_errors()`.
    ///
    /// # Arguments
    ///
    /// * `directory` - The directory to scan for tools
    ///
    /// # Returns
    ///
    /// A vector of discovered tools, or an error if the directory
    /// cannot be read or traversed.
    ///
    /// # Examples
    ///
    /// ```
    /// use mcp_serve::tool_discovery::DirectoryScanner;
    /// use std::path::Path;
    ///
    /// let mut scanner = DirectoryScanner::new();
    /// match scanner.scan_directory(Path::new("./tools")) {
    ///     Ok(tools) => println!("Found {} tools", tools.len()),
    ///     Err(e) => eprintln!("Scan failed: {}", e),
    /// }
    ///
    /// // Check for permission errors that were collected
    /// let errors = scanner.take_errors();
    /// for error in errors {
    ///     eprintln!("Warning: {}", error);
    /// }
    /// ```
    pub fn scan_directory(&mut self, directory: &Path) -> Result<Vec<DiscoveredTool>, ScanError> {
        let mut discovered_tools = Vec::new();

        // Read directory entries
        let entries = match fs::read_dir(directory) {
            Ok(entries) => entries,
            Err(e) => return Err(ScanError::IoError(e)),
        };

        for entry in entries {
            let entry = match entry {
                Ok(entry) => entry,
                Err(e) => {
                    self.errors.push(ScanError::IoError(e));
                    continue;
                }
            };

            let path = entry.path();

            // Skip directories for now (could be extended for recursive scanning)
            if path.is_dir() {
                continue;
            }

            // Check if this file is executable
            if let Some(tool) = self.check_executable(&path) {
                discovered_tools.push(tool);
            }
        }

        Ok(discovered_tools)
    }

    /// Check if a file is executable and create a DiscoveredTool if so.
    ///
    /// Uses `faccess::PathExt::executable()` for cross-platform executable
    /// detection. Also checks for associated sidecar `.yaml` files.
    fn check_executable(&mut self, path: &Path) -> Option<DiscoveredTool> {
        // Use faccess for cross-platform executable detection
        // Note: This is treated as an optimization hint, not a security decision
        let is_executable = match path.executable() {
            true => true,
            false => {
                // On some systems, files might be executable but faccess might
                // not detect it perfectly. We could add fallback logic here
                // based on file extension for Windows (.exe, .bat, .cmd) etc.
                self.check_executable_by_extension(path)
            }
        };

        if !is_executable {
            return None;
        }

        let metadata_source = self.find_metadata_source(path);

        Some(DiscoveredTool {
            executable_path: path.to_path_buf(),
            metadata_source,
        })
    }

    /// Check if file might be executable based on file extension.
    ///
    /// This serves as a fallback for systems where `faccess` might not
    /// perfectly detect executables (e.g., Windows executable extensions).
    fn check_executable_by_extension(&self, path: &Path) -> bool {
        if let Some(extension) = path.extension() {
            let ext = extension.to_string_lossy().to_lowercase();
            matches!(ext.as_str(), "exe" | "bat" | "cmd" | "ps1" | "sh")
        } else {
            // For extensionless files, check if the file is actually executable using faccess
            path.executable()
        }
    }

    /// Find the metadata source for a given executable.
    ///
    /// First checks for a sidecar `.yaml` file, then assumes metadata
    /// is embedded in the executable itself.
    fn find_metadata_source(&mut self, executable_path: &Path) -> MetadataSource {
        // Check for sidecar .yaml file
        let sidecar_path = executable_path.with_extension("yaml");

        if sidecar_path.exists() {
            // Verify we can read the sidecar file
            match fs::metadata(&sidecar_path) {
                Ok(_) => MetadataSource::Sidecar(sidecar_path),
                Err(_) => {
                    // Permission error accessing sidecar - fall back to embedded
                    self.errors
                        .push(ScanError::PermissionDenied { path: sidecar_path });
                    MetadataSource::Embedded(executable_path.to_path_buf())
                }
            }
        } else {
            // No sidecar file - assume embedded metadata
            MetadataSource::Embedded(executable_path.to_path_buf())
        }
    }

    /// Take all collected errors from the scanner.
    ///
    /// This allows callers to handle permission errors and other issues
    /// that occurred during scanning without failing the entire operation.
    ///
    /// # Returns
    ///
    /// A vector of all errors collected during scanning, clearing the
    /// internal error collection.
    pub fn take_errors(&mut self) -> Vec<ScanError> {
        std::mem::take(&mut self.errors)
    }

    /// Get a reference to collected errors without taking ownership.
    pub fn errors(&self) -> &[ScanError] {
        &self.errors
    }
}

impl Default for DirectoryScanner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_tool_definition_creation() {
        let yaml = r#"
name: test_tool
description: A test tool
input:
  template: "--name {{name}}"
  schema:
    type: object
    properties:
      name:
        type: string
        description: Name parameter
    required: [name]
output:
  template: "Result: (?<result>.*)"
  schema:
    type: object
    properties:
      result:
        type: string
        description: Operation result
"#;

        let tool = ToolDefinition::from_yaml(yaml).expect("Should parse YAML");

        assert_eq!(tool.name, "test_tool");
        assert_eq!(tool.description, "A test tool");
        assert!(tool.title.is_none());
        assert!(tool.annotations.is_none());
        assert_eq!(tool.input.template, "--name {{name}}");
        assert_eq!(tool.output.template, "Result: (?<result>.*)");
    }

    #[test]
    fn test_tool_definition_with_optional_fields() {
        let yaml = r#"
name: test
title: Test Tool
description: Test tool
input:
  template: "--test"
  schema:
    type: object
output:
  template: "Result: (?<value>.*)"
  schema:
    type: string
"#;

        let tool = ToolDefinition::from_yaml(yaml).expect("Should parse YAML");

        assert_eq!(tool.title, Some("Test Tool".to_string()));
        assert_eq!(tool.output.template, "Result: (?<value>.*)");
    }

    #[test]
    fn test_mcp_tool_creation() {
        // Test McpTool via conversion from ToolDefinition
        let yaml = r#"
name: mcp_test
description: MCP test tool
input:
  template: "--test"
  schema:
    type: object
output:
  template: "Result: (?<value>.*)"
  schema:
    type: string
"#;

        let tool = ToolDefinition::from_yaml(yaml).expect("Should parse YAML");
        let mcp_tool = tool.to_mcp_tool();

        assert_eq!(mcp_tool.name, "mcp_test");
        assert_eq!(mcp_tool.description, "MCP test tool");
        assert!(mcp_tool.title.is_none());
        assert!(mcp_tool.output_schema.is_some());
    }

    #[test]
    fn test_conversion_to_mcp_tool() {
        let yaml = r#"
name: convert_test
title: Convert Test
description: Conversion test
input:
  template: "--param {{param}}"
  schema:
    type: object
    properties:
      param:
        type: string
output:
  template: "Result: (?<result>.*)"
  schema:
    type: string
"#;

        let tool = ToolDefinition::from_yaml(yaml).expect("Should parse YAML");
        let mcp_tool = tool.to_mcp_tool();

        assert_eq!(mcp_tool.name, "convert_test");
        assert_eq!(mcp_tool.title, Some("Convert Test".to_string()));
        assert_eq!(mcp_tool.description, "Conversion test");
        assert_eq!(mcp_tool.input_schema["type"], "object");
        assert_eq!(mcp_tool.output_schema.unwrap()["type"], "string");
    }

    #[test]
    fn test_yaml_serialization_tool_definition() {
        let yaml = r#"
name: create_ticket
title: Create Ticket
description: Creates a ticket
input:
  template: "--title {{title}} {{body}}"
  schema:
    type: object
    properties:
      title:
        type: string
        description: Ticket title
      body:
        type: string
        description: Ticket body
    required: [title, body]
output:
  template: "Created: (?<url>https://.*)"
  schema:
    type: object
    properties:
      url:
        type: string
"#;

        // Test deserialization
        let tool = ToolDefinition::from_yaml(yaml).expect("Should deserialize from YAML");

        assert_eq!(tool.name, "create_ticket");
        assert_eq!(tool.title, Some("Create Ticket".to_string()));
        assert_eq!(tool.description, "Creates a ticket");
        assert_eq!(tool.input.template, "--title {{title}} {{body}}");
        assert!(tool.input.schema["properties"].is_object());
        assert_eq!(tool.output.template, "Created: (?<url>https://.*)");

        // Test round-trip serialization
        let serialized = serde_yaml_ng::to_string(&tool).expect("Should serialize to YAML");
        let reparsed = ToolDefinition::from_yaml(&serialized).expect("Should deserialize again");
        assert_eq!(tool, reparsed);
    }

    #[test]
    fn test_yaml_deserialization_from_design_example() {
        // This matches the format from docs/Design.md
        let yaml = r#"
name: CreateTicket
title: Create Ticket
description: Creates a new feature ticket in the project tracking system.
input:
  template: '--title {{title}} [--parent {{parent_id}}] [--label {{label}}...] {{body}}'
  schema:
    type: object
    properties:
      title:
        type: string
        description: "The title of the feature ticket."
      body:
        type: string
        description: "A detailed description of the feature in markdown."
      parent_id:
        type: string
        description: "Optional: The ID of the parent ticket."
      label:
        type: array
        items: { type: string }
        description: "Optional: A list of labels to apply."
    required: [ "title", "body" ]
output:
  template: |-
    Ticket created: (?<url>https://.*)
    ID: (?<id>\d+)
  schema:
    type: object
    properties:
      url: { type: string }
      id: { type: string }
"#;

        let tool = ToolDefinition::from_yaml(yaml).expect("Should parse YAML");

        assert_eq!(tool.name, "CreateTicket");
        assert_eq!(tool.title, Some("Create Ticket".to_string()));
        assert_eq!(
            tool.description,
            "Creates a new feature ticket in the project tracking system."
        );

        // Verify input
        assert_eq!(
            tool.input.template,
            "--title {{title}} [--parent {{parent_id}}] [--label {{label}}...] {{body}}"
        );
        assert_eq!(tool.input.schema["type"], "object");
        assert!(tool.input.schema["properties"].is_object());
        assert_eq!(tool.input.schema["required"], json!(["title", "body"]));

        // Verify output
        assert!(tool
            .output
            .template
            .contains("Ticket created: (?<url>https://.*)"));
        assert!(tool.output.template.contains("ID: (?<id>\\d+)"));
        assert_eq!(tool.output.schema["type"], "object");
    }

    #[test]
    fn test_mcp_tool_yaml_serialization() {
        // Test McpTool serialization via conversion from ToolDefinition
        let yaml = r#"
name: mcp_tool
description: MCP tool
input:
  template: "--param {{param}}"
  schema:
    type: object
    properties:
      param:
        type: string
output:
  template: "Result: (?<value>.*)"
  schema:
    type: string
"#;

        let tool = ToolDefinition::from_yaml(yaml).expect("Should parse YAML");
        let mcp_tool = tool.to_mcp_tool();

        let mcp_yaml = serde_yaml_ng::to_string(&mcp_tool).expect("Should serialize");
        assert!(mcp_yaml.contains("name: mcp_tool"));
        assert!(mcp_yaml.contains("input_schema:"));
        assert!(!mcp_yaml.contains("template:")); // Should not have template fields

        let parsed: McpTool = serde_yaml_ng::from_str(&mcp_yaml).expect("Should parse");
        assert_eq!(parsed.name, "mcp_tool");
        assert_eq!(parsed.description, "MCP tool");
    }

    #[test]
    fn test_json_value_schema_flexibility() {
        // Test that we can handle various JSON Schema formats as opaque values
        let simple_yaml = r#"
template: "--name {{name}}"
schema:
  type: string
"#;

        let complex_yaml = r#"
template: "--name {{name}} --age {{age}}"
schema:
  type: object
  properties:
    name:
      type: string
    age:
      type: integer
      minimum: 0
  required: [name]
"#;

        // Both should serialize and deserialize fine
        let input1: ToolInput = serde_yaml_ng::from_str(simple_yaml).unwrap();
        let input2: ToolInput = serde_yaml_ng::from_str(complex_yaml).unwrap();

        let yaml1 = serde_yaml_ng::to_string(&input1).unwrap();
        let yaml2 = serde_yaml_ng::to_string(&input2).unwrap();

        let _parsed1: ToolInput = serde_yaml_ng::from_str(&yaml1).unwrap();
        let _parsed2: ToolInput = serde_yaml_ng::from_str(&yaml2).unwrap();
    }

    // DirectoryScanner tests
    mod directory_scanner_tests {
        use super::*;
        use std::fs::{self, File};
        use std::io::Write;
        use tempfile::TempDir;

        /// Create a temporary directory with test files for scanning
        fn setup_test_directory() -> TempDir {
            let temp_dir = TempDir::new().expect("Failed to create temp directory");
            let temp_path = temp_dir.path();

            // Create various test files

            // 1. Executable script with sidecar YAML
            let script_path = temp_path.join("test_script");
            let mut script_file = File::create(&script_path).expect("Failed to create script");
            script_file
                .write_all(b"#!/bin/bash\necho 'Hello World'")
                .expect("Failed to write script");

            // Make script executable (on Unix systems)
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mut perms = fs::metadata(&script_path).unwrap().permissions();
                perms.set_mode(0o755);
                fs::set_permissions(&script_path, perms)
                    .expect("Failed to set executable permissions");
            }

            // Create sidecar YAML file
            let yaml_path = temp_path.join("test_script.yaml");
            let mut yaml_file = File::create(yaml_path).expect("Failed to create YAML");
            yaml_file
                .write_all(b"name: test_script\ndescription: A test script")
                .expect("Failed to write YAML");

            // 2. Executable without sidecar (embedded metadata)
            let executable_path = temp_path.join("standalone_tool");
            let mut exe_file = File::create(&executable_path).expect("Failed to create executable");
            exe_file
                .write_all(b"#!/bin/python3\nprint('Standalone tool')")
                .expect("Failed to write executable");

            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mut perms = fs::metadata(&executable_path).unwrap().permissions();
                perms.set_mode(0o755);
                fs::set_permissions(&executable_path, perms)
                    .expect("Failed to set executable permissions");
            }

            // 3. Windows executable
            let windows_exe = temp_path.join("windows_tool.exe");
            let mut win_file =
                File::create(windows_exe).expect("Failed to create Windows executable");
            win_file
                .write_all(b"Windows executable content")
                .expect("Failed to write Windows executable");

            // 4. Non-executable file
            let text_file = temp_path.join("readme.txt");
            let mut txt_file = File::create(text_file).expect("Failed to create text file");
            txt_file
                .write_all(b"This is not an executable file")
                .expect("Failed to write text file");

            // 5. Directory (should be skipped)
            fs::create_dir(temp_path.join("subdir")).expect("Failed to create subdirectory");

            temp_dir
        }

        #[test]
        fn test_directory_scanner_basic() {
            let temp_dir = setup_test_directory();
            let mut scanner = DirectoryScanner::new();

            let discovered_tools = scanner
                .scan_directory(temp_dir.path())
                .expect("Failed to scan directory");

            // We should discover tools (exact count depends on platform executable detection)
            assert!(
                !discovered_tools.is_empty(),
                "Should discover at least one tool"
            );

            // Check that all discovered items are tools with proper paths
            for tool in &discovered_tools {
                assert!(
                    tool.executable_path.exists(),
                    "Tool executable path should exist"
                );
                match &tool.metadata_source {
                    MetadataSource::Embedded(path) => {
                        assert!(path.exists(), "Embedded metadata path should exist");
                    }
                    MetadataSource::Sidecar(path) => {
                        assert!(path.exists(), "Sidecar metadata path should exist");
                        assert!(
                            path.extension().unwrap() == "yaml",
                            "Sidecar should be YAML file"
                        );
                    }
                }
            }
        }

        #[test]
        fn test_sidecar_yaml_detection() {
            let temp_dir = setup_test_directory();
            let mut scanner = DirectoryScanner::new();

            let discovered_tools = scanner
                .scan_directory(temp_dir.path())
                .expect("Failed to scan directory");

            // Find the tool with sidecar YAML
            let script_tool = discovered_tools
                .iter()
                .find(|tool| tool.executable_path.file_name().unwrap() == "test_script")
                .expect("Should find test_script tool");

            match &script_tool.metadata_source {
                MetadataSource::Sidecar(sidecar_path) => {
                    assert_eq!(sidecar_path.file_name().unwrap(), "test_script.yaml");
                    assert!(sidecar_path.exists());
                }
                MetadataSource::Embedded(_) => {
                    panic!("Expected sidecar metadata source for test_script");
                }
            }
        }

        #[test]
        fn test_embedded_metadata_fallback() {
            let temp_dir = setup_test_directory();
            let mut scanner = DirectoryScanner::new();

            let discovered_tools = scanner
                .scan_directory(temp_dir.path())
                .expect("Failed to scan directory");

            // Find the standalone tool (should have embedded metadata)
            let standalone_tool = discovered_tools
                .iter()
                .find(|tool| tool.executable_path.file_name().unwrap() == "standalone_tool");

            if let Some(tool) = standalone_tool {
                match &tool.metadata_source {
                    MetadataSource::Embedded(embedded_path) => {
                        assert_eq!(*embedded_path, tool.executable_path);
                    }
                    MetadataSource::Sidecar(_) => {
                        panic!("Expected embedded metadata source for standalone_tool");
                    }
                }
            }
        }

        #[test]
        fn test_windows_executable_detection() {
            let temp_dir = setup_test_directory();
            let mut scanner = DirectoryScanner::new();

            let discovered_tools = scanner
                .scan_directory(temp_dir.path())
                .expect("Failed to scan directory");

            // Check if Windows executable is detected based on extension
            let windows_tool = discovered_tools
                .iter()
                .find(|tool| tool.executable_path.file_name().unwrap() == "windows_tool.exe");

            // On Windows or when extension-based detection is used, this should be found
            if cfg!(windows) || windows_tool.is_some() {
                let tool = windows_tool.expect("Should find Windows executable");
                assert!(tool.executable_path.extension().unwrap() == "exe");
            }
        }

        #[test]
        fn test_non_executable_files_ignored() {
            let temp_dir = setup_test_directory();
            let mut scanner = DirectoryScanner::new();

            let discovered_tools = scanner
                .scan_directory(temp_dir.path())
                .expect("Failed to scan directory");

            // Verify that readme.txt is not discovered as a tool
            let text_file_tool = discovered_tools
                .iter()
                .find(|tool| tool.executable_path.file_name().unwrap() == "readme.txt");

            assert!(
                text_file_tool.is_none(),
                "Non-executable text files should not be discovered"
            );
        }

        #[test]
        fn test_directories_are_skipped() {
            let temp_dir = setup_test_directory();
            let mut scanner = DirectoryScanner::new();

            let discovered_tools = scanner
                .scan_directory(temp_dir.path())
                .expect("Failed to scan directory");

            // Verify that subdirectory is not discovered as a tool
            let subdir_tool = discovered_tools
                .iter()
                .find(|tool| tool.executable_path.file_name().unwrap() == "subdir");

            assert!(
                subdir_tool.is_none(),
                "Directories should not be discovered as tools"
            );
        }

        #[test]
        fn test_error_handling_invalid_directory() {
            let mut scanner = DirectoryScanner::new();

            let result = scanner.scan_directory(Path::new("/nonexistent/directory"));

            assert!(
                result.is_err(),
                "Should return error for nonexistent directory"
            );
            match result {
                Err(ScanError::IoError(_)) => {
                    // Expected error type
                }
                Err(e) => panic!("Unexpected error type: {:?}", e),
                Ok(_) => panic!("Should not succeed scanning nonexistent directory"),
            }
        }

        #[test]
        fn test_error_collection() {
            let mut scanner = DirectoryScanner::new();

            // Initially no errors
            assert!(scanner.errors().is_empty());

            // Try to scan a valid directory - this might collect some permission errors
            let temp_dir = setup_test_directory();
            let _ = scanner.scan_directory(temp_dir.path());

            // Test error access methods
            let error_count = scanner.errors().len();
            let taken_errors = scanner.take_errors();

            assert_eq!(taken_errors.len(), error_count);
            assert!(
                scanner.errors().is_empty(),
                "Errors should be cleared after taking them"
            );
        }

        #[test]
        fn test_discovered_tool_equality() {
            let path1 = PathBuf::from("/path/to/tool");
            let path2 = PathBuf::from("/path/to/tool");
            let path3 = PathBuf::from("/path/to/other");

            let tool1 = DiscoveredTool {
                executable_path: path1.clone(),
                metadata_source: MetadataSource::Embedded(path1.clone()),
            };

            let tool2 = DiscoveredTool {
                executable_path: path2.clone(),
                metadata_source: MetadataSource::Embedded(path2.clone()),
            };

            let tool3 = DiscoveredTool {
                executable_path: path3.clone(),
                metadata_source: MetadataSource::Embedded(path3.clone()),
            };

            assert_eq!(tool1, tool2, "Tools with same paths should be equal");
            assert_ne!(
                tool1, tool3,
                "Tools with different paths should not be equal"
            );
        }

        #[test]
        fn test_metadata_source_equality() {
            let path1 = PathBuf::from("/path/to/tool");
            let path2 = PathBuf::from("/path/to/tool");
            let path3 = PathBuf::from("/path/to/other");

            let embedded1 = MetadataSource::Embedded(path1.clone());
            let embedded2 = MetadataSource::Embedded(path2.clone());
            let embedded3 = MetadataSource::Embedded(path3.clone());
            let sidecar1 = MetadataSource::Sidecar(path1.clone());

            assert_eq!(
                embedded1, embedded2,
                "Embedded sources with same path should be equal"
            );
            assert_ne!(
                embedded1, embedded3,
                "Embedded sources with different paths should not be equal"
            );
            assert_ne!(
                embedded1, sidecar1,
                "Embedded and sidecar sources should not be equal"
            );
        }

        #[test]
        fn test_scanner_default() {
            let scanner1 = DirectoryScanner::new();
            let scanner2 = DirectoryScanner::default();

            // Both should start with no errors
            assert_eq!(scanner1.errors().len(), scanner2.errors().len());
        }
    }

    #[test]
    fn test_error_handling_malformed_yaml() {
        let malformed_yaml = r#"
name: "test_tool"
description: A test tool
input:
  template: "--test"
  schema:
    type: object
    properties:
      invalid: [unclosed
output:
  template: "Result: (?<result>.*)"
  schema:
    type: string
"#;

        let result: Result<ToolDefinition, _> = serde_yaml_ng::from_str(malformed_yaml);
        assert!(result.is_err(), "Malformed YAML should produce an error");

        let error = result.unwrap_err();
        let error_str = error.to_string();
        assert!(!error_str.is_empty(), "Error message should not be empty");
    }
}
