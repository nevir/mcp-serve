# High-Level Design: mcp-serve

## Vision

**mcp-serve** is a lightweight server that instantly turns a directory of executable scripts into a powerful toolset for an AI agent.

Our goal is to provide a developer experience as simple and immediate as `python -m http.server`. With a single command, any developer can take their existing shell scripts, Python programs, or compiled binaries and securely expose them to an AI model using the Model-Context Protocol (MCP).

### Core Principles

- **Simplicity:** Get up and running with a single command. No boilerplate required.
- **Convention over Configuration:** Sensible defaults allow immediate use, with powerful configuration available when needed.
- **Language Agnostic:** If it can be executed in a shell, it can be an AI tool.
- **Stateless:** The server manages no state, leaving all logic and state management to the scripts themselves.

## The Core Workflow

1. **Create Your Scripts:** A developer has a local directory containing the scripts they want to expose as tools.
   ```
   /path/to/my/project/.tools/
   ├── create-ticket
   └── update-ticket-status
   ```

2. **Define the Tool's Interface:** Each script is paired with a definition that tells `mcp-serve` how it works.

   - **Embedded Metadata:** For any script with a shebang (`#!`), the tool definition is embedded as a YAML block in the comments immediately following it. `mcp-serve` intelligently handles various comment styles (`#`, `//`, etc.). This keeps the script and its AI interface in a single, self-contained file.
     ```bash
     #!/bin/bash
     # ---
     # name: CreateTicket
     # description: Creates a new feature ticket.
     # ... (rest of definition)
     # ---

     # The script's logic begins here
     echo "Ticket created..."
     ```
   - **Sidecar File:** For compiled binaries or when embedding isn't possible, a separate `<tool-name>.yaml` file can be placed alongside the executable.

3. **Run the Server:** The developer starts `mcp-serve` from their tools directory.
   ```bash
   mcp-serve
   ```
   The server instantly scans the directory (or a different one specified with `--tools /path/to/tools`), parses the tool definitions, and exposes them on a local HTTP server.

4. **Interact via API:** An AI agent can now communicate with `mcp-serve` using standard MCP HTTP requests:
   - `GET /`: Discovers the list of available tools and their capabilities.
   - `POST /tools/create-ticket`: Executes the `create-ticket` script with specific parameters.

## Deployment Flexibility

`mcp-serve` is designed for any environment:

- **Local Binary:** Run `mcp-serve` directly for instant testing and development.
- **Global Binary:** Install via a package manager like Homebrew for easy access anywhere on your system.
- **Docker Image:** Use the official Docker image for deployment in containerized environments. The tools directory and other configurations are typically baked into the image.
  ```bash
  docker run --rm -p 8080:8080 your/mcp-serve-image
  ```

## Architecture

- **File System Scanner:** On startup, scans the target directory for executables and their definitions (from embedded metadata or sidecar files). Can optionally watch for file changes to reload tools without a restart.
- **MCP HTTP Server:** Implements the standard MCP endpoints (`/`, `/tools`, `/tools/{tool_id}`) for agent communication.
- **Tool Registry:** An in-memory catalog of all parsed and validated tool definitions.
- **Execution Engine:** Spawns sandboxed child processes to run scripts, securely capturing their `stdout`, `stderr`, and exit codes.
- **Marshaller:** Translates data between the AI's JSON-based world and the script's command-line world.

## Tool Definition & Metadata

Every script requires a YAML definition that adheres to the [MCP specification](https://modelcontextprotocol.io/specification/2025-06-18/server/tools#tool). This definition serves as the bridge between the AI model and your code.

**Example: Embedded Definition in a `create-ticket` script**

```bash
#!/bin/bash
# ---
# # The official name for the tool, used in API calls.
# name: CreateTicket
#
# # A human-friendly title.
# title: Create Ticket
#
# # A clear, concise description that helps the AI decide when to use this tool.
# description: Creates a new feature ticket in the project tracking system.
#
# input:
#   # The template that transforms the JSON input into a command-line execution string.
#   template: '--title {{title}} [--parent {{parent_id}}] [--label {{label}}...] {{body}}'
#
#   # A JSON Schema defining the parameters the tool accepts (inputSchema).
#   schema:
#     type: object
#     properties:
#       title:
#         type: string
#         description: "The title of the feature ticket."
#       body:
#         type: string
#         description: "A detailed description of the feature in markdown."
#       parent_id:
#         type: string
#         description: "Optional: The ID of the parent ticket."
#       label:
#         type: array
#         items: { type: string }
#         description: "Optional: A list of labels to apply."
#     required: [ "title", "body" ]
#
# output:
#   # A regex that parses the script's text output back into a structured JSON object.
#   template: |-
#     Ticket created: (?<url>https://.*)
#     ID: (?<id>\d+)
#
#   # The JSON Schema of the object this tool returns on success.
#   schema:
#     type: object
#     properties:
#       url: { type: string }
#       id: { type: string }
# ---

# --- Script logic starts here ---
# This is a mock script for demonstration.
echo "Ticket created: https://jira.example.com/T-1234"
echo "ID: 999999999"
```

## Bridging the Gap: How mcp-serve Communicates with Scripts

An AI model communicates in structured JSON, while a shell script understands command-line arguments and produces plain text output. The `input.template` and `output.template` fields are powerful templates that act as the translator between these two worlds.

### `input`: From JSON to Command-Line Arguments

The `input` template defines how to convert the AI's JSON request into a command-line string that your script can understand.

**1. Basic Placeholders:** `{{property}}` inserts the value of a JSON property, ensuring it's properly escaped.

| JSON Input                                  | `input` Template             | Resulting Command                        |
| :------------------------------------------ | :--------------------------- | :--------------------------------------- |
| `{"title": "My Ticket", "body": "Details"}` | `--title {{title}} {{body}}` | `./script --title "My Ticket" "Details"` |

**2. Optional Arguments:** `[...]` defines a section that is only included if all properties within it are present in the JSON input. This is perfect for optional flags.

| JSON Input                                   | `input` Template                             | Resulting Command                             |
| :------------------------------------------- | :------------------------------------------- | :-------------------------------------------- |
| `{"title": "My Ticket"}`                     | `--title {{title}} [--parent {{parent_id}}]` | `./script --title "My Ticket"`                |
| `{"title": "My Ticket", "parent_id": "123"}` | `--title {{title}} [--parent {{parent_id}}]` | `./script --title "My Ticket" --parent "123"` |

**3. Handling Lists:** `[... ...]` defines a repeating section for each item in an array. This is ideal for arguments that can be specified multiple times.

| JSON Input                 | `input` Template         | Resulting Command                     |
| :------------------------- | :----------------------- | :------------------------------------ |
| `{"label": ["ux", "api"]}` | `[--label {{label}}...]` | `./script --label "ux" --label "api"` |

### `output`: From Plain Text to Structured JSON

The `output` template defines how to parse the plain text `stdout` from your script back into the structured JSON that the AI model expects. It uses a regular expression with **named capture groups** (`(?<name>...)`) to create the JSON properties.

**Example:**
Imagine your script produces the following text on `stdout`:

```
Ticket created: https://jira.example.com/T-167823
ID: 98765
```

The `output` template can precisely extract the needed data:

| `stdout` from Script                     | `output` Template                                    | Resulting JSON                                                |
| :--------------------------------------- | :--------------------------------------------------- | :------------------------------------------------------------ |
| `Ticket created: https://...\nID: 98765` | `Ticket created: (?<url>https://.*)\nID: (?<id>\d+)` | `{"url": "https://jira.example.com/T-167823", "id": "98765"}` |
