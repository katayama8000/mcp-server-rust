# Cat Database MCP Server

A Model Context Protocol (MCP) server implementation in Rust that provides a cat database management system. This server allows AI assistants and MCP clients to interact with a collection of cat data through a standardized protocol.

## Features

- **Cat Database Management**: Store and retrieve information about cats including ID, name, age, breed, color, indoor status, and favorite toys
- **Multiple Query Tools**: Four different tools to access cat data
- **MCP Protocol Compliance**: Full implementation of Model Context Protocol for seamless integration with AI assistants
- **Async Processing**: Built with Tokio for efficient non-blocking operations
- **Error Handling**: Proper MCP error codes and structured error responses

## Available Tools

1. **`list_all_cats`** - Get a list of all registered cats
   - No parameters required
   - Returns: Complete list of all cats with their details

2. **`get_cat_by_id`** - Get detailed information about a specific cat
   - Parameters: `id` (number) - The cat's ID
   - Returns: Detailed information about the specified cat

3. **`search_by_breed`** - Search for cats by breed
   - Parameters: `breed` (string) - Breed name to search for
   - Returns: List of cats matching the breed (partial match supported)

4. **`get_indoor_cats`** - Get only indoor cats
   - No parameters required
   - Returns: List of cats that are kept indoors

## Sample Data

The server comes pre-populated with 4 sample cats:

- **Mike** (ID: 1) - 3-year-old Calico, indoor cat, loves mouse toys
- **Shiro** (ID: 2) - 5-year-old Persian, indoor cat, loves yarn balls  
- **Kuro** (ID: 3) - 2-year-old Black cat, outdoor cat, loves butterflies
- **Chatora** (ID: 4) - 7-year-old Orange tabby, indoor cat, loves catnip

## Prerequisites

- Rust (latest stable version)
- Cargo package manager

## Installation & Usage

1. **Clone or create the project**:
```bash
git clone <repository-url>
cd mcp-server-rust
```

2. **Build the project**:
```bash
cargo build --release
```

3. **Run the server**:
```bash
cargo run
```

The server will start and listen for MCP protocol messages via standard input/output (stdio).

## Testing with MCP Inspector

You can test this server using the MCP Inspector tool:

```bash
npx @modelcontextprotocol/inspector cargo run
```

This will start the inspector and automatically connect to your cat database server.

## Integration with AI Assistants

This server can be integrated with various AI assistants that support MCP:

- **Claude Desktop**: Add server configuration to your Claude Desktop settings
- **VS Code Extensions**: Use with MCP-compatible VS Code extensions
- **Custom Clients**: Integrate with any application that supports MCP protocol

## Example Usage

Once connected through an MCP client, you can use commands like:

```json
// Get all cats
{"tool": "list_all_cats"}

// Get specific cat by ID
{"tool": "get_cat_by_id", "arguments": {"id": 1}}

// Search by breed
{"tool": "search_by_breed", "arguments": {"breed": "Persian"}}

// Get indoor cats only
{"tool": "get_indoor_cats"}
```

## Development

To modify or extend the server:

1. Edit `src/main.rs` to add new cats or tools
2. Run `cargo check` to verify compilation
3. Test with `cargo run`

## Dependencies

- `rmcp`: Rust MCP SDK for protocol implementation
- `tokio`: Async runtime
- `serde`: Serialization framework
- `tracing`: Structured logging
- `anyhow`: Error handling

## License

MIT License - see LICENSE file for details.
