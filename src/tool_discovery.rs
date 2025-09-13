//! Tool discovery system for MCP (Model Context Protocol) servers.
//!
//! This module provides data structures and functionality for discovering,
//! parsing, and validating tool definitions from executable files and
//! associated metadata sources.
//!
//! The design separates pure MCP protocol structures from mcp-serve's custom
//! YAML format that includes templates for command-line argument generation
//! and output parsing.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Pure MCP tool definition as specified in the Model Context Protocol.
///
/// This structure represents the exact MCP specification format and is used
/// when communicating with MCP clients. It contains no mcp-serve specific
/// extensions.
///
/// # Examples
///
/// ```
/// use mcp_serve::tool_discovery::{McpTool, JsonSchema, JsonSchemaType};
///
/// let tool = McpTool {
///     name: "calculate_sum".to_string(),
///     title: Some("Calculator".to_string()),
///     description: "Add two numbers together".to_string(),
///     input_schema: JsonSchema {
///         schema_type: Some(JsonSchemaType::Object),
///         ..Default::default()
///     },
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
    #[serde(rename = "input_schema")]
    pub input_schema: JsonSchema,

    /// Optional JSON Schema for output structure
    #[serde(rename = "output_schema")]
    pub output_schema: Option<JsonSchema>,

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
/// # Examples
///
/// ```
/// use serde_yaml_ng;
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
///     required: [title, body]
/// "#;
///
/// let tool: ToolDefinition = serde_yaml_ng::from_str(yaml).unwrap();
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

    /// Input specification with schema and template
    pub input: ToolInput,

    /// Optional output specification with schema and template
    pub output: Option<ToolOutput>,

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
    pub schema: JsonSchema,
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
    pub schema: JsonSchema,
}

/// JSON Schema representation for tool input and output parameter definitions.
///
/// This structure represents a JSON Schema object as defined in the JSON Schema
/// specification, providing type validation and documentation for tool parameters.
///
/// # Examples
///
/// ```
/// use mcp_serve::tool_discovery::{JsonSchema, JsonSchemaType};
/// use std::collections::HashMap;
///
/// let schema = JsonSchema {
///     schema_type: Some(JsonSchemaType::Object),
///     properties: Some({
///         let mut props = HashMap::new();
///         props.insert("name".to_string(), JsonSchema {
///             schema_type: Some(JsonSchemaType::String),
///             description: Some("The name parameter".to_string()),
///             ..Default::default()
///         });
///         props
///     }),
///     required: Some(vec!["name".to_string()]),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct JsonSchema {
    /// The JSON Schema type (object, string, number, etc.)
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub schema_type: Option<JsonSchemaType>,

    /// Description of this schema element
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Properties for object-type schemas
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, JsonSchema>>,

    /// Required property names for object-type schemas
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,

    /// Items schema for array-type schemas
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Box<JsonSchema>>,

    /// Enumeration of allowed values
    #[serde(rename = "enum", skip_serializing_if = "Option::is_none")]
    pub enum_values: Option<Vec<serde_yaml_ng::Value>>,

    /// Default value for this schema element
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<serde_yaml_ng::Value>,

    /// Additional properties flag for object schemas
    #[serde(
        rename = "additionalProperties",
        skip_serializing_if = "Option::is_none"
    )]
    pub additional_properties: Option<bool>,

    /// Minimum value for numeric schemas
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<f64>,

    /// Maximum value for numeric schemas
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<f64>,

    /// Minimum length for string schemas
    #[serde(rename = "minLength", skip_serializing_if = "Option::is_none")]
    pub min_length: Option<usize>,

    /// Maximum length for string schemas
    #[serde(rename = "maxLength", skip_serializing_if = "Option::is_none")]
    pub max_length: Option<usize>,

    /// Pattern (regex) for string validation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
}

/// JSON Schema type enumeration.
///
/// Represents the core JSON Schema types used for parameter validation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum JsonSchemaType {
    /// Object type with properties
    Object,
    /// Array type with items
    Array,
    /// String type
    String,
    /// Numeric type (integer or float)
    Number,
    /// Integer type (subset of number)
    Integer,
    /// Boolean type
    Boolean,
    /// Null type
    Null,
}

impl ToolDefinition {
    /// Create a new tool definition with required fields.
    ///
    /// # Examples
    ///
    /// ```
    /// use mcp_serve::tool_discovery::{ToolDefinition, ToolInput, JsonSchema};
    ///
    /// let input = ToolInput {
    ///     template: "--name {{name}}".to_string(),
    ///     schema: JsonSchema::default(),
    /// };
    ///
    /// let tool = ToolDefinition::new("example_tool", "An example tool", input);
    /// assert_eq!(tool.name, "example_tool");
    /// ```
    pub fn new(name: impl Into<String>, description: impl Into<String>, input: ToolInput) -> Self {
        Self {
            name: name.into(),
            title: None,
            description: description.into(),
            input,
            output: None,
            annotations: None,
        }
    }

    /// Set the optional title for this tool definition.
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the optional output specification.
    pub fn with_output(mut self, output: ToolOutput) -> Self {
        self.output = Some(output);
        self
    }

    /// Set annotations for this tool definition.
    pub fn with_annotations(mut self, annotations: HashMap<String, serde_yaml_ng::Value>) -> Self {
        self.annotations = Some(annotations);
        self
    }

    /// Convert this mcp-serve tool definition to a pure MCP tool.
    ///
    /// This extracts the schema information and discards the template-specific
    /// extensions, creating a tool definition that conforms to the MCP specification.
    ///
    /// # Examples
    ///
    /// ```
    /// use mcp_serve::tool_discovery::{ToolDefinition, ToolInput, JsonSchema};
    ///
    /// let input = ToolInput {
    ///     template: "--name {{name}}".to_string(),
    ///     schema: JsonSchema::default(),
    /// };
    ///
    /// let tool = ToolDefinition::new("test", "Test tool", input);
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
            output_schema: self.output.as_ref().map(|o| o.schema.clone()),
            annotations: self.annotations.clone(),
        }
    }
}

impl McpTool {
    /// Create a new MCP tool with required fields.
    ///
    /// # Examples
    ///
    /// ```
    /// use mcp_serve::tool_discovery::{McpTool, JsonSchema};
    ///
    /// let tool = McpTool::new("test", "Test tool", JsonSchema::default());
    /// assert_eq!(tool.name, "test");
    /// ```
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        input_schema: JsonSchema,
    ) -> Self {
        Self {
            name: name.into(),
            title: None,
            description: description.into(),
            input_schema,
            output_schema: None,
            annotations: None,
        }
    }

    /// Set the optional title.
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the optional output schema.
    pub fn with_output_schema(mut self, output_schema: JsonSchema) -> Self {
        self.output_schema = Some(output_schema);
        self
    }

    /// Set annotations.
    pub fn with_annotations(mut self, annotations: HashMap<String, serde_yaml_ng::Value>) -> Self {
        self.annotations = Some(annotations);
        self
    }
}

impl ToolInput {
    /// Create a new tool input specification.
    ///
    /// # Examples
    ///
    /// ```
    /// use mcp_serve::tool_discovery::{ToolInput, JsonSchema};
    ///
    /// let input = ToolInput::new("--name {{name}}", JsonSchema::default());
    /// assert_eq!(input.template, "--name {{name}}");
    /// ```
    pub fn new(template: impl Into<String>, schema: JsonSchema) -> Self {
        Self {
            template: template.into(),
            schema,
        }
    }
}

impl ToolOutput {
    /// Create a new tool output specification.
    ///
    /// # Examples
    ///
    /// ```
    /// use mcp_serve::tool_discovery::{ToolOutput, JsonSchema};
    ///
    /// let output = ToolOutput::new("Result: (?<value>.*)", JsonSchema::default());
    /// assert_eq!(output.template, "Result: (?<value>.*)");
    /// ```
    pub fn new(template: impl Into<String>, schema: JsonSchema) -> Self {
        Self {
            template: template.into(),
            schema,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_tool_definition_creation() {
        let input_schema = JsonSchema {
            schema_type: Some(JsonSchemaType::Object),
            properties: Some({
                let mut props = HashMap::new();
                props.insert(
                    "name".to_string(),
                    JsonSchema {
                        schema_type: Some(JsonSchemaType::String),
                        description: Some("Name parameter".to_string()),
                        ..Default::default()
                    },
                );
                props
            }),
            required: Some(vec!["name".to_string()]),
            ..Default::default()
        };

        let input = ToolInput {
            template: "--name {{name}}".to_string(),
            schema: input_schema,
        };

        let tool = ToolDefinition::new("test_tool", "A test tool", input);

        assert_eq!(tool.name, "test_tool");
        assert_eq!(tool.description, "A test tool");
        assert!(tool.title.is_none());
        assert!(tool.output.is_none());
        assert!(tool.annotations.is_none());
        assert_eq!(tool.input.template, "--name {{name}}");
    }

    #[test]
    fn test_tool_definition_with_optional_fields() {
        let input = ToolInput::new("--test", JsonSchema::default());
        let output = ToolOutput::new(
            "Result: (?<value>.*)",
            JsonSchema {
                schema_type: Some(JsonSchemaType::String),
                ..Default::default()
            },
        );

        let tool = ToolDefinition::new("test", "Test tool", input)
            .with_title("Test Tool")
            .with_output(output);

        assert_eq!(tool.title, Some("Test Tool".to_string()));
        assert!(tool.output.is_some());
        assert_eq!(tool.output.unwrap().template, "Result: (?<value>.*)");
    }

    #[test]
    fn test_mcp_tool_creation() {
        let input_schema = JsonSchema {
            schema_type: Some(JsonSchemaType::Object),
            ..Default::default()
        };

        let tool = McpTool::new("mcp_test", "MCP test tool", input_schema);

        assert_eq!(tool.name, "mcp_test");
        assert_eq!(tool.description, "MCP test tool");
        assert!(tool.title.is_none());
        assert!(tool.output_schema.is_none());
    }

    #[test]
    fn test_conversion_to_mcp_tool() {
        let input_schema = JsonSchema {
            schema_type: Some(JsonSchemaType::Object),
            properties: Some({
                let mut props = HashMap::new();
                props.insert(
                    "param".to_string(),
                    JsonSchema {
                        schema_type: Some(JsonSchemaType::String),
                        ..Default::default()
                    },
                );
                props
            }),
            ..Default::default()
        };

        let output_schema = JsonSchema {
            schema_type: Some(JsonSchemaType::String),
            ..Default::default()
        };

        let input = ToolInput::new("--param {{param}}", input_schema.clone());
        let output = ToolOutput::new("Result: (?<result>.*)", output_schema.clone());

        let tool = ToolDefinition::new("convert_test", "Conversion test", input)
            .with_title("Convert Test")
            .with_output(output);

        let mcp_tool = tool.to_mcp_tool();

        assert_eq!(mcp_tool.name, "convert_test");
        assert_eq!(mcp_tool.title, Some("Convert Test".to_string()));
        assert_eq!(mcp_tool.description, "Conversion test");
        assert_eq!(mcp_tool.input_schema, input_schema);
        assert_eq!(mcp_tool.output_schema, Some(output_schema));
    }

    #[test]
    fn test_yaml_serialization_tool_definition() {
        let input_schema = JsonSchema {
            schema_type: Some(JsonSchemaType::Object),
            properties: Some({
                let mut props = HashMap::new();
                props.insert(
                    "title".to_string(),
                    JsonSchema {
                        schema_type: Some(JsonSchemaType::String),
                        description: Some("Ticket title".to_string()),
                        ..Default::default()
                    },
                );
                props.insert(
                    "body".to_string(),
                    JsonSchema {
                        schema_type: Some(JsonSchemaType::String),
                        description: Some("Ticket body".to_string()),
                        ..Default::default()
                    },
                );
                props
            }),
            required: Some(vec!["title".to_string(), "body".to_string()]),
            ..Default::default()
        };

        let output_schema = JsonSchema {
            schema_type: Some(JsonSchemaType::Object),
            properties: Some({
                let mut props = HashMap::new();
                props.insert(
                    "url".to_string(),
                    JsonSchema {
                        schema_type: Some(JsonSchemaType::String),
                        ..Default::default()
                    },
                );
                props
            }),
            ..Default::default()
        };

        let input = ToolInput::new("--title {{title}} {{body}}", input_schema);
        let output = ToolOutput::new("Created: (?<url>https://.*)", output_schema);

        let tool = ToolDefinition::new("create_ticket", "Creates a ticket", input)
            .with_title("Create Ticket")
            .with_output(output);

        // Test serialization
        let yaml = serde_yaml_ng::to_string(&tool).expect("Should serialize to YAML");
        assert!(yaml.contains("name: create_ticket"));
        assert!(yaml.contains("title: Create Ticket"));
        assert!(yaml.contains("description: Creates a ticket"));
        assert!(yaml.contains("input:"));
        assert!(yaml.contains("template: --title {{title}} {{body}}"));
        assert!(yaml.contains("schema:"));
        assert!(yaml.contains("output:"));

        // Test deserialization
        let parsed: ToolDefinition =
            serde_yaml_ng::from_str(&yaml).expect("Should deserialize from YAML");

        assert_eq!(parsed.name, "create_ticket");
        assert_eq!(parsed.title, Some("Create Ticket".to_string()));
        assert_eq!(parsed.description, "Creates a ticket");
        assert_eq!(parsed.input.template, "--title {{title}} {{body}}");
        assert!(parsed.input.schema.properties.is_some());
        assert!(parsed.output.is_some());
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

        let tool: ToolDefinition = serde_yaml_ng::from_str(yaml).expect("Should parse YAML");

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
        assert_eq!(tool.input.schema.schema_type, Some(JsonSchemaType::Object));
        assert!(tool.input.schema.properties.is_some());
        assert_eq!(
            tool.input.schema.required,
            Some(vec!["title".to_string(), "body".to_string()])
        );

        // Verify output
        let output = tool.output.expect("Should have output");
        assert!(output
            .template
            .contains("Ticket created: (?<url>https://.*)"));
        assert!(output.template.contains("ID: (?<id>\\d+)"));
        assert_eq!(output.schema.schema_type, Some(JsonSchemaType::Object));
    }

    #[test]
    fn test_mcp_tool_yaml_serialization() {
        let input_schema = JsonSchema {
            schema_type: Some(JsonSchemaType::Object),
            properties: Some({
                let mut props = HashMap::new();
                props.insert(
                    "param".to_string(),
                    JsonSchema {
                        schema_type: Some(JsonSchemaType::String),
                        ..Default::default()
                    },
                );
                props
            }),
            ..Default::default()
        };

        let tool = McpTool::new("mcp_tool", "MCP tool", input_schema);

        let yaml = serde_yaml_ng::to_string(&tool).expect("Should serialize");
        assert!(yaml.contains("name: mcp_tool"));
        assert!(yaml.contains("input_schema:"));
        assert!(!yaml.contains("template:")); // Should not have template fields

        let parsed: McpTool = serde_yaml_ng::from_str(&yaml).expect("Should parse");
        assert_eq!(parsed.name, "mcp_tool");
        assert_eq!(parsed.description, "MCP tool");
    }

    #[test]
    fn test_json_schema_types() {
        let schemas = vec![
            (JsonSchemaType::Object, "object"),
            (JsonSchemaType::Array, "array"),
            (JsonSchemaType::String, "string"),
            (JsonSchemaType::Number, "number"),
            (JsonSchemaType::Integer, "integer"),
            (JsonSchemaType::Boolean, "boolean"),
            (JsonSchemaType::Null, "null"),
        ];

        for (schema_type, expected_string) in schemas {
            let json_schema = JsonSchema {
                schema_type: Some(schema_type.clone()),
                ..Default::default()
            };

            let yaml = serde_yaml_ng::to_string(&json_schema).unwrap();

            // Check for both possible formats: "type: string" and "type: 'string'"
            let type_line = format!("type: {}", expected_string);
            let quoted_type_line = format!("type: '{}'", expected_string);
            assert!(
                yaml.contains(&type_line) || yaml.contains(&quoted_type_line),
                "YAML should contain type field. YAML: {}",
                yaml
            );
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
"#;

        let result: Result<ToolDefinition, _> = serde_yaml_ng::from_str(malformed_yaml);
        assert!(result.is_err(), "Malformed YAML should produce an error");

        let error = result.unwrap_err();
        let error_str = error.to_string();
        assert!(!error_str.is_empty(), "Error message should not be empty");
    }
}
