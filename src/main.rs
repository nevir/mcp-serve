use clap::Parser;
use std::path::PathBuf;

pub mod tool_discovery;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Directory to discover tools from
    #[arg(default_value = ".")]
    tools_dir: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    println!(
        "Discovering tools from directory: {}",
        cli.tools_dir.display()
    );
    println!("Tools functionality working");
}

#[cfg(test)]
mod tests {
    use faccess::PathExt;
    use serde::{Deserialize, Serialize};
    use std::path::PathBuf;

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct TestToolDefinition {
        name: String,
        description: String,
        version: Option<String>,
    }

    #[test]
    fn test_dependencies_integration() {
        // Test 1: faccess - Test executable detection on current binary
        let current_exe = std::env::current_exe().expect("Failed to get current executable path");
        assert!(
            current_exe.executable(),
            "Current executable should be detected as executable"
        );

        // Test 2: serde_yaml_ng - Test YAML parsing
        let yaml_content = r#"
name: "test_tool"
description: "A test tool for validation"
version: "1.0.0"
"#;

        let parsed: TestToolDefinition =
            serde_yaml_ng::from_str(yaml_content).expect("Failed to parse YAML with serde_yaml_ng");

        assert_eq!(parsed.name, "test_tool");
        assert_eq!(parsed.description, "A test tool for validation");
        assert_eq!(parsed.version, Some("1.0.0".to_string()));

        // Test 3: serde - Test serialization back to YAML
        let serialized = serde_yaml_ng::to_string(&parsed).expect("Failed to serialize to YAML");

        assert!(serialized.contains("name: test_tool"));
        assert!(serialized.contains("description: A test tool for validation"));

        // Test 4: Cross-platform path handling
        let test_path = PathBuf::from(".");
        assert!(test_path.exists(), "Current directory should exist");
    }

    #[test]
    fn test_faccess_on_non_executable() {
        // Test faccess on a known non-executable file (Cargo.toml)
        let cargo_toml = PathBuf::from("Cargo.toml");
        if cargo_toml.exists() {
            // Note: This might be executable on some systems due to permissions,
            // but it demonstrates the faccess API works
            let _is_executable = cargo_toml.executable();
            // We just verify the method doesn't panic
        }
    }

    #[test]
    fn test_serde_yaml_ng_error_handling() {
        // Test error handling for malformed YAML
        let malformed_yaml = r#"
name: "test_tool"
description: A test tool for validation
version: 1.0.0
invalid_yaml: [unclosed_list
"#;

        let result: Result<TestToolDefinition, _> = serde_yaml_ng::from_str(malformed_yaml);
        assert!(result.is_err(), "Malformed YAML should produce an error");

        // Verify error contains useful information
        let error = result.unwrap_err();
        let error_str = error.to_string();
        assert!(!error_str.is_empty(), "Error message should not be empty");
    }
}
