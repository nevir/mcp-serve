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

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
/// use serde_yaml_ng;
/// use mcp_serve::tool_discovery::{ToolDefinition, ToolInput, ToolOutput};
/// use serde_json::json;
///
/// // Both input and output are now required
/// let input = ToolInput::new(
///     "--title {{title}} {{body}}",
///     json!({
///         "type": "object",
///         "properties": {
///             "title": {"type": "string"},
///             "body": {"type": "string"}
///         },
///         "required": ["title", "body"]
///     })
/// );
///
/// let output = ToolOutput::new(
///     "Created: (?<url>https://.*)",
///     json!({
///         "type": "object",
///         "properties": {
///             "url": {"type": "string"}
///         }
///     })
/// );
///
/// let tool = ToolDefinition::new("create_ticket", "Creates a new feature ticket", input, output)
///     .with_title("Create Ticket");
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
    /// let tool = ToolDefinition::new("test", "Test tool", input, output);
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
