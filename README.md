# mcp-serve

A lightweight server that instantly turns a directory of executable scripts into a powerful toolset for AI agents using the [Model Context Protocol (MCP)](https://modelcontextprotocol.io/).

## Overview

**mcp-serve** provides a developer experience as simple as `python -m http.server`. With a single command, you can take your existing shell scripts, Python programs, or compiled binaries and securely expose them to AI models.

### Core Principles

- **Simplicity:** Get up and running with a single command. No boilerplate required.
- **Convention over Configuration:** Sensible defaults allow immediate use, with powerful configuration available when needed.
- **Language Agnostic:** If it can be executed in a shell, it can be an AI tool.
- **Stateless:** The server manages no state, leaving all logic and state management to the scripts themselves.

## Quick Start

1. **Create your tools directory:**
   ```bash
   mkdir my-tools
   cd my-tools
   ```

2. **Add a script with embedded metadata:**
   ```bash
   cat > create-ticket << 'EOF'
   #!/bin/bash
   # ---
   # name: CreateTicket
   # description: Creates a new feature ticket in the project tracking system.
   # input:
   #   template: '--title {{title}} [--parent {{parent_id}}] {{body}}'
   #   schema:
   #     type: object
   #     properties:
   #       title: { type: string, description: "The title of the feature ticket." }
   #       body: { type: string, description: "A detailed description of the feature." }
   #       parent_id: { type: string, description: "Optional: The ID of the parent ticket." }
   #     required: ["title", "body"]
   # output:
   #   template: 'Ticket created: (?<url>https://.*)\nID: (?<id>\d+)'
   #   schema:
   #     type: object
   #     properties:
   #       url: { type: string }
   #       id: { type: string }
   # ---
   
   echo "Ticket created: https://jira.example.com/T-$RANDOM"
   echo "ID: $RANDOM"
   EOF
   
   chmod +x create-ticket
   ```

3. **Start the server:**
   ```bash
   mcp-serve
   ```

4. **Your tools are now available via MCP:**
   - `GET /` - Discover available tools
   - `POST /tools/CreateTicket` - Execute the create-ticket script

## Installation

### Requirements

- Rust 1.70 or later

### From Source

```bash
git clone https://github.com/nevir/mcp-serve.git
cd mcp-serve
cargo install --path .
```

### Using Cargo

```bash
cargo install mcp-serve
```

### Docker

```bash
docker run --rm -p 8080:8080 -v ./my-tools:/tools mcp-serve/mcp-serve
```

## Usage

### Basic Usage

```bash
# Serve tools from current directory
mcp-serve

# Serve tools from specific directory
mcp-serve /path/to/my/tools

# Show help
mcp-serve --help
```

### Command Options

```
mcp-serve [TOOLS_DIR]

Arguments:
  [TOOLS_DIR]  Directory to discover tools from [default: .]

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Tool Definition

Each script requires metadata that defines its interface for AI agents. You can embed this metadata directly in the script or use a separate YAML file.

### Embedded Metadata (Recommended)

For scripts with shebangs, embed the YAML definition in comments immediately following the shebang:

```bash
#!/bin/bash
# ---
# name: MyTool
# description: What this tool does
# input:
#   template: '--flag {{param}} {{body}}'
#   schema:
#     type: object
#     properties:
#       param: { type: string }
#       body: { type: string }
#     required: ["param", "body"]
# ---

# Your script logic here
echo "Tool executed with: $1 $2"
```

### Sidecar Files

For compiled binaries or when embedding isn't possible, create a `<script-name>.yaml` file:

```yaml
# my-binary.yaml
name: MyBinary
description: What this binary does
input:
  template: '--input {{data}}'
  schema:
    type: object
    properties:
      data: { type: string }
    required: ["data"]
```

### Template System

mcp-serve uses templates to bridge JSON (AI world) and command-line arguments (script world):

#### Input Templates
- `{{property}}` - Insert JSON property value
- `[--flag {{property}}]` - Optional section, included only if property exists
- `[--item {{array}}...]` - Repeat section for each array item

#### Output Templates
Use regex with named capture groups to parse script output:
```yaml
output:
  template: 'Result: (?<status>\w+)\nValue: (?<value>\d+)'
  schema:
    type: object
    properties:
      status: { type: string }
      value: { type: string }
```

## Architecture

- **File System Scanner:** Discovers executables and their definitions on startup
- **MCP HTTP Server:** Implements standard MCP endpoints for AI agent communication
- **Tool Registry:** In-memory catalog of parsed and validated tool definitions
- **Execution Engine:** Spawns sandboxed child processes to run scripts
- **Marshaller:** Translates between JSON (AI) and command-line (scripts)

## Examples

See the [examples directory](examples/) for complete working examples of various tool types and patterns.

## License

This project is licensed under the [Blue Oak Model License 1.0.0](LICENSE.md).