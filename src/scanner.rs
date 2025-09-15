//! Directory scanning functionality for discovering executable tools.
//!
//! This module provides the `DirectoryScanner` component that traverses directories,
//! identifies executable files using cross-platform permission checks, and locates
//! associated metadata sources (embedded or sidecar files).

use faccess::PathExt;
use std::fs;
use std::path::{Path, PathBuf};

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
/// use mcp_serve::scanner::DirectoryScanner;
/// use std::path::Path;
///
/// let mut scanner = DirectoryScanner::new();
/// match scanner.scan_directory(Path::new(".")) {
///     Ok(tools) => println!("Found {} tools", tools.len()),
///     Err(e) => eprintln!("Scan failed: {}", e),
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
    /// Returns all discovered tools and collects any non-fatal errors
    /// encountered during scanning. Non-fatal errors (like individual file
    /// permission issues) are stored internally and can be retrieved with `take_errors()`.
    ///
    /// # Error Handling
    ///
    /// - **Fatal errors** (e.g., directory doesn't exist or can't be read) return `Err`
    /// - **Non-fatal errors** (e.g., individual file permission issues) are collected
    ///   internally and scanning continues
    ///
    /// # Arguments
    ///
    /// * `directory` - The directory to scan for tools
    ///
    /// # Returns
    ///
    /// A vector of discovered tools, or a fatal error if the directory
    /// cannot be read or traversed.
    ///
    /// # Examples
    ///
    /// ```
    /// use mcp_serve::scanner::DirectoryScanner;
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
        if !path.executable() {
            return None;
        }

        let metadata_source = self.find_metadata_source(path);

        Some(DiscoveredTool {
            executable_path: path.to_path_buf(),
            metadata_source,
        })
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
            fs::set_permissions(&script_path, perms).expect("Failed to set executable permissions");
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
        let mut win_file = File::create(windows_exe).expect("Failed to create Windows executable");
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
