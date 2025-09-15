# Example Tools

This directory contains example tools that demonstrate various patterns for the DirectoryScanner:

## Tool Types

### 1. `create-ticket` - Bash script with embedded metadata

- **Type**: Bash script with shebang
- **Metadata**: Embedded YAML in comments following the shebang
- **Demonstrates**: Complex input template with optional and array parameters

### 2. `calculator` + `calculator.yaml` - Python script with sidecar metadata

- **Type**: Python script with separate YAML file
- **Metadata**: Sidecar `.yaml` file
- **Demonstrates**: Python tool with structured error handling, enum validation

### 3. `file-info.sh` - Shell script with embedded metadata

- **Type**: Shell script with `.sh` extension
- **Metadata**: Embedded YAML in comments
- **Demonstrates**: Cross-platform file operations, error handling

### 4. `simple-exe` - Node.js script without metadata

- **Type**: Node.js script (extensionless executable)
- **Metadata**: None (demonstrates embedded metadata fallback)
- **Demonstrates**: Minimal executable without formal tool definition

### 5. `non-executable.txt` - Non-executable file

- **Type**: Text file
- **Purpose**: Should be ignored by DirectoryScanner
- **Demonstrates**: Proper filtering of non-executable files

## Usage for Testing

```bash
# Test the DirectoryScanner with these examples
cd /path/to/mcp-serve
cargo run -- examples/tools

# Or run specific tests
cargo test directory_scanner_tests
```

## Integration Testing

These examples can be used for end-to-end testing:

1. **Discovery**: DirectoryScanner should find all executable files
2. **Metadata Detection**: Should correctly identify sidecar vs embedded metadata
3. **Cross-platform**: Should work on Unix, macOS, and Windows
4. **Error Handling**: Should handle permission issues gracefully
