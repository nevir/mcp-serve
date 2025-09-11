//! Tool discovery system for MCP (Model Context Protocol) servers.
//!
//! This module provides data structures and functionality for discovering,
//! parsing, and validating tool definitions from executable files and
//! associated metadata sources.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Core tool definition representing a parsed MCP tool with all its metadata.
///
/// This structure follows the Model Context Protocol specification for tool
/// definitions and supports both embedded YAML frontmatter and sidecar file
/// metadata formats.
///
/// # Examples
///
/// ```
/// use mcp_serve::tool_discovery::ToolDefinition;
/// use serde_yaml_ng;
///
/// let yaml = r#"
/// name: calculate_sum
/// description: Add two numbers together
/// input_schema:
///   type: object
///   properties:
///     a:
///       type: number
///     b:
///       type: number
///   required: [a, b]
/// "#;
///
/// let tool: ToolDefinition = serde_yaml_ng::from_str(yaml).unwrap();
/// assert_eq!(tool.name, "calculate_sum");
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolDefinition {
    /// Unique identifier for the tool.
    ///
    /// This must be unique within a tool discovery scope and is used to
    /// identify and invoke the tool through the MCP protocol.
    pub name: String,

    /// Optional human-readable display name for the tool.
    ///
    /// If provided, this should be used for display purposes in user interfaces.
    /// If not provided, the `name` field can be used for display.
    pub title: Option<String>,

    /// Human-readable description of the tool's functionality.
    ///
    /// This should clearly explain what the tool does and when it should be used.
    pub description: String,

    /// JSON Schema defining the expected input parameters for the tool.
    ///
    /// This schema is used to validate input parameters before tool execution
    /// and to provide parameter information to MCP clients.
    #[serde(rename = "input_schema")]
    pub input_schema: JsonSchema,

    /// Optional JSON Schema defining the expected output structure.
    ///
    /// When provided, tool implementations should return results that conform
    /// to this schema, and clients may validate results against it.
    #[serde(rename = "output_schema")]
    pub output_schema: Option<JsonSchema>,

    /// Optional metadata annotations providing additional information about the tool.
    ///
    /// This can include information about trust levels, safety considerations,
    /// or other metadata that affects how the tool should be used.
    pub annotations: Option<HashMap<String, serde_yaml_ng::Value>>,
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
    /// This is a convenience constructor for creating tool definitions programmatically.
    ///
    /// # Examples
    ///
    /// ```
    /// use mcp_serve::tool_discovery::{ToolDefinition, JsonSchema, JsonSchemaType};
    ///
    /// let tool = ToolDefinition::new(
    ///     "example_tool",
    ///     "An example tool for demonstration",
    ///     JsonSchema {
    ///         schema_type: Some(JsonSchemaType::Object),
    ///         ..Default::default()
    ///     }
    /// );
    ///
    /// assert_eq!(tool.name, "example_tool");
    /// assert_eq!(tool.description, "An example tool for demonstration");
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

    /// Set the optional title for this tool definition.
    ///
    /// # Examples
    ///
    /// ```
    /// use mcp_serve::tool_discovery::{ToolDefinition, JsonSchema};
    ///
    /// let tool = ToolDefinition::new("calc", "Calculator", JsonSchema::default())
    ///     .with_title("Advanced Calculator");
    ///
    /// assert_eq!(tool.title, Some("Advanced Calculator".to_string()));
    /// ```
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the optional output schema for this tool definition.
    ///
    /// # Examples
    ///
    /// ```
    /// use mcp_serve::tool_discovery::{ToolDefinition, JsonSchema, JsonSchemaType};
    ///
    /// let output_schema = JsonSchema {
    ///     schema_type: Some(JsonSchemaType::Number),
    ///     description: Some("The calculation result".to_string()),
    ///     ..Default::default()
    /// };
    ///
    /// let tool = ToolDefinition::new("calc", "Calculator", JsonSchema::default())
    ///     .with_output_schema(output_schema);
    ///
    /// assert!(tool.output_schema.is_some());
    /// ```
    pub fn with_output_schema(mut self, output_schema: JsonSchema) -> Self {
        self.output_schema = Some(output_schema);
        self
    }

    /// Set annotations for this tool definition.
    ///
    /// # Examples
    ///
    /// ```
    /// use mcp_serve::tool_discovery::ToolDefinition;
    /// use std::collections::HashMap;
    ///
    /// let mut annotations = HashMap::new();
    /// annotations.insert("safe".to_string(), serde_yaml_ng::Value::Bool(true));
    ///
    /// let tool = ToolDefinition::new("calc", "Calculator", Default::default())
    ///     .with_annotations(annotations);
    ///
    /// assert!(tool.annotations.is_some());
    /// ```
    pub fn with_annotations(mut self, annotations: HashMap<String, serde_yaml_ng::Value>) -> Self {
        self.annotations = Some(annotations);
        self
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
                    "value".to_string(),
                    JsonSchema {
                        schema_type: Some(JsonSchemaType::String),
                        description: Some("Input value".to_string()),
                        ..Default::default()
                    },
                );
                props
            }),
            required: Some(vec!["value".to_string()]),
            ..Default::default()
        };

        let tool = ToolDefinition::new("test_tool", "A test tool", input_schema);

        assert_eq!(tool.name, "test_tool");
        assert_eq!(tool.description, "A test tool");
        assert!(tool.title.is_none());
        assert!(tool.output_schema.is_none());
        assert!(tool.annotations.is_none());
    }

    #[test]
    fn test_tool_definition_with_optional_fields() {
        let tool = ToolDefinition::new("test", "Test tool", JsonSchema::default())
            .with_title("Test Tool")
            .with_output_schema(JsonSchema {
                schema_type: Some(JsonSchemaType::String),
                ..Default::default()
            });

        assert_eq!(tool.title, Some("Test Tool".to_string()));
        assert!(tool.output_schema.is_some());
    }

    #[test]
    fn test_yaml_serialization() {
        let input_schema = JsonSchema {
            schema_type: Some(JsonSchemaType::Object),
            properties: Some({
                let mut props = HashMap::new();
                props.insert(
                    "a".to_string(),
                    JsonSchema {
                        schema_type: Some(JsonSchemaType::Number),
                        description: Some("First number".to_string()),
                        ..Default::default()
                    },
                );
                props.insert(
                    "b".to_string(),
                    JsonSchema {
                        schema_type: Some(JsonSchemaType::Number),
                        description: Some("Second number".to_string()),
                        ..Default::default()
                    },
                );
                props
            }),
            required: Some(vec!["a".to_string(), "b".to_string()]),
            ..Default::default()
        };

        let tool = ToolDefinition::new("calculate_sum", "Add two numbers together", input_schema);

        // Test serialization
        let yaml = serde_yaml_ng::to_string(&tool).expect("Should serialize to YAML");
        assert!(yaml.contains("name: calculate_sum"));
        assert!(yaml.contains("description: Add two numbers together"));
        assert!(yaml.contains("input_schema:"));

        // Test deserialization
        let parsed: ToolDefinition =
            serde_yaml_ng::from_str(&yaml).expect("Should deserialize from YAML");

        assert_eq!(parsed.name, "calculate_sum");
        assert_eq!(parsed.description, "Add two numbers together");
        assert!(parsed.input_schema.properties.is_some());
    }

    #[test]
    fn test_yaml_deserialization_from_string() {
        let yaml = r#"
name: example_tool
title: Example Tool
description: An example tool for testing
input_schema:
  type: object
  properties:
    param1:
      type: string
      description: First parameter
    param2:
      type: number
      description: Second parameter
  required: [param1, param2]
output_schema:
  type: object
  properties:
    result:
      type: string
annotations:
  safe: true
  version: "1.0"
"#;

        let tool: ToolDefinition = serde_yaml_ng::from_str(yaml).expect("Should parse YAML");

        assert_eq!(tool.name, "example_tool");
        assert_eq!(tool.title, Some("Example Tool".to_string()));
        assert_eq!(tool.description, "An example tool for testing");

        // Verify input schema
        assert_eq!(tool.input_schema.schema_type, Some(JsonSchemaType::Object));
        assert!(tool.input_schema.properties.is_some());
        assert_eq!(
            tool.input_schema.required,
            Some(vec!["param1".to_string(), "param2".to_string()])
        );

        // Verify output schema
        assert!(tool.output_schema.is_some());

        // Verify annotations
        assert!(tool.annotations.is_some());
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
input_schema:
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
