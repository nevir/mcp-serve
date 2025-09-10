# mcp-serve

🚀 **Turn any script into an AI tool in seconds**

Transform a directory of executables into a powerful [MCP](https://modelcontextprotocol.io/) server for AI agents. As simple as `python -m http.server`.

```bash
mcp-serve my-tools/  # ✨ That's it
```

**Why mcp-serve?**

- 🔧 **Any language** — Bash, Python, compiled binaries, whatever
- ⚡ **Zero config** — Just add YAML metadata to your scripts
- 🏠 **Stateless** — Your scripts manage their own data
- 🔒 **Secure** — Sandboxed execution

## Quick Start

**1. Create a tool** 📝

`./tools/create-ticket`:

```bash
#!/usr/bin/env bash
# ---
# description: Creates a new feature ticket
#
# input:
#   template: '--title {{title}} {{body}}'
#   schema:
#     type: object
#     properties:
#       title: { type: string }
#       body: { type: string }
#     required: ["title", "body"]
#
# output:
#   template: 'Ticket created: (?<url>https://.*)\nID: (?<id>T-\d+)'
#   schema:
#     type: object
#     properties:
#       url: { type: string }
#       id: { type: string }
# ---
ID=$RANDOM
echo "Ticket created: https://jira.example.com/T-$ID"
echo "ID: T-$ID"
```

**2. Start the server** 🚀

```bash
mcp-serve ./tools
```

**3. AI agents can now call your tools** 🤖

```bash
POST /tools/create-ticket
# → Executes your script with JSON input
```

## Installation

**Requirements:** Rust 1.70+

```bash
# From crates.io
cargo install mcp-serve

# From source
git clone https://github.com/nevir/mcp-serve.git
cargo install --path mcp-serve

# Docker
docker run -p 8080:8080 -v ./tools:/tools mcp-serve/mcp-serve
```

## Usage

```bash
mcp-serve                    # Current directory
mcp-serve /path/to/tools     # Custom directory
mcp-serve --help             # Show options
```

## How It Works

**🔌 Metadata in scripts:** Add YAML headers to define the AI interface

```bash
#!/bin/bash
# ---
# description: What this tool does
# input:
#   template: '--flag {{param}} {{body}}'
#   schema: { /* JSON schema */ }
# ---
echo "Tool executed with: $1 $2"
```

**🔗 Template magic:** Bridge JSON ↔ command-line

- `{{property}}` → inserts values
- `[--flag {{optional}}]` → conditional flags
- `[--item {{array}}...]` → repeat for arrays

**📤 Parse output:** Use regex to extract structured results

```yaml
output:
  template: 'Status: (?<status>\w+)'
```

## What's Next?

Check out [examples/](examples/) for more patterns and use cases.

## License

[Blue Oak 1.0.0](LICENSE.md)
