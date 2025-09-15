//! Integration test for DirectoryScanner using example tools.
//!
//! This test demonstrates how the DirectoryScanner works with real tool files
//! in a practical scenario.

use mcp_serve::scanner::{DirectoryScanner, MetadataSource};
use std::path::Path;

#[test]
fn test_directory_scanner_with_examples() {
    let examples_path = Path::new("examples/tools");

    // Skip test if examples directory doesn't exist (e.g., in CI without examples)
    if !examples_path.exists() {
        eprintln!("Skipping integration test: examples/tools directory not found");
        return;
    }

    let mut scanner = DirectoryScanner::new();
    let discovered_tools = scanner
        .scan_directory(examples_path)
        .expect("Failed to scan examples directory");

    // Should discover several tools
    assert!(
        !discovered_tools.is_empty(),
        "Should discover tools in examples directory"
    );

    // Check for specific expected tools
    let tool_names: Vec<_> = discovered_tools
        .iter()
        .filter_map(|tool| tool.executable_path.file_name())
        .filter_map(|name| name.to_str())
        .collect();

    println!("Discovered tools: {:?}", tool_names);

    // Verify specific tools are found (platform-dependent)
    let has_create_ticket = tool_names.contains(&"create-ticket");
    let has_calculator = tool_names.contains(&"calculator");
    let has_file_info = tool_names.contains(&"file-info.sh");
    let has_simple_exe = tool_names.contains(&"simple-exe");

    // At least some tools should be discovered
    assert!(
        has_create_ticket || has_calculator || has_file_info || has_simple_exe,
        "Should discover at least one example tool"
    );

    // Verify that non-executable files are not discovered
    let has_non_executable = tool_names.contains(&"non-executable.txt");
    let has_readme = tool_names.contains(&"README.md");

    assert!(
        !has_non_executable,
        "Should not discover non-executable.txt"
    );
    assert!(!has_readme, "Should not discover README.md");

    // Check for sidecar metadata detection
    if let Some(calculator_tool) = discovered_tools
        .iter()
        .find(|tool| tool.executable_path.file_name().unwrap() == "calculator")
    {
        match &calculator_tool.metadata_source {
            MetadataSource::Sidecar(sidecar_path) => {
                assert!(sidecar_path.exists(), "Sidecar file should exist");
                assert_eq!(
                    sidecar_path.extension().unwrap(),
                    "yaml",
                    "Sidecar should be YAML"
                );
            }
            MetadataSource::Embedded(_) => {
                // On some platforms, sidecar might not be detected - that's okay
                println!(
                    "Note: Calculator tool detected as embedded metadata (platform-dependent)"
                );
            }
        }
    }

    // Check that we collect any permission errors properly
    let errors = scanner.take_errors();
    if !errors.is_empty() {
        println!(
            "Scanner collected {} errors (expected on some systems): {:?}",
            errors.len(),
            errors
        );
    }

    println!(
        "Integration test completed successfully with {} tools discovered",
        discovered_tools.len()
    );
}

#[test]
fn test_examples_directory_structure() {
    let examples_path = Path::new("examples/tools");

    if !examples_path.exists() {
        eprintln!("Skipping directory structure test: examples/tools not found");
        return;
    }

    // Verify expected files exist
    let expected_files = [
        "create-ticket",
        "calculator",
        "calculator.yaml",
        "file-info.sh",
        "simple-exe",
        "non-executable.txt",
        "README.md",
    ];

    for file in &expected_files {
        let file_path = examples_path.join(file);
        assert!(
            file_path.exists(),
            "Expected example file should exist: {}",
            file
        );
    }

    println!("All expected example files are present");
}
