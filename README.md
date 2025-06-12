# MCP Protocol Client

[![Crates.io](https://img.shields.io/crates/v/mcp-protocol-client.svg)](https://crates.io/crates/mcp-protocol-client)
[![Documentation](https://docs.rs/mcp-protocol-client/badge.svg)](https://docs.rs/mcp-protocol-client)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

**Client library for the Model Context Protocol (MCP)**

This crate provides a high-level, ergonomic API for building MCP clients in Rust. It handles connection management, capability negotiation, and transport abstraction, allowing you to focus on using MCP servers' capabilities.

## ✨ Features

- 🦀 **Pure Rust** - Zero-cost abstractions with memory safety
- 🎯 **Type-Safe** - Compile-time guarantees using mcp-protocol-types
- 🚀 **Async/Await** - Built on Tokio for high performance
- 🔌 **Multiple Transports** - STDIO, HTTP, WebSocket support
- 🛠️ **Complete MCP Support** - Tools, resources, prompts, logging
- 📦 **Lightweight** - Minimal dependencies for fast builds
- 🧪 **Well Tested** - Comprehensive test suite
- 📖 **Great Documentation** - Examples and guides

## 🚀 Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
mcp-protocol-client = "0.1.0"
mcp-protocol-types = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
serde_json = "1.0"
```

### Basic Client Example

```rust
use mcp_protocol_client::{Client, ClientBuilder};
use mcp_protocol_types::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create and connect client
    let mut client = ClientBuilder::new("my-client", "1.0.0")
        .build();

    // Connect via STDIO transport
    client.connect_stdio().await?;
    
    // Initialize connection
    let init_result = client.initialize().await?;
    println!("Connected to: {}", init_result.server_info.name);
    
    // List available tools
    let tools = client.list_tools().await?;
    for tool in tools {
        println!("Available tool: {} - {}", tool.name, 
                 tool.description.unwrap_or_default());
    }
    
    // Call a tool
    let result = client.call_tool("my-tool", json!({
        "param": "value"
    })).await?;
    
    println!("Tool result: {:?}", result);
    
    Ok(())
}
```

## 🏗️ Architecture

The client library provides a layered architecture:

```
┌─────────────────────────────────────────────┐
│           Your Application                   │
├─────────────────────────────────────────────┤
│         MCP Protocol Client                 │ ← This crate
├─────────────────────────────────────────────┤
│         MCP Protocol Types                  │
├─────────────────────────────────────────────┤
│           Transport Layer                   │ (STDIO, HTTP, WebSocket)
└─────────────────────────────────────────────┘
```

## 📋 Core Concepts

### Client Builder

The `ClientBuilder` provides a fluent API for configuring your client:

```rust
use mcp_protocol_client::ClientBuilder;

let client = ClientBuilder::new("my-client", "1.0.0")
    .with_description("An awesome MCP client")
    .with_timeout(Duration::from_secs(30))
    .with_retry_policy(RetryPolicy::exponential())
    .build();
```

### Connection Management

Connect to servers using different transports:

```rust
// STDIO transport (for local servers)
client.connect_stdio().await?;

// HTTP transport (for remote servers)
client.connect_http("http://localhost:8080/mcp").await?;

// WebSocket transport (for real-time communication)
client.connect_websocket("ws://localhost:8080/mcp").await?;
```

### Server Interaction

Use the client to interact with MCP servers:

```rust
// Initialize connection and exchange capabilities
let init_result = client.initialize().await?;

// List available tools
let tools = client.list_tools().await?;

// Call a tool
let result = client.call_tool("calculate", json!({
    "expression": "2 + 2"
})).await?;

// List resources
let resources = client.list_resources().await?;

// Read a resource
let content = client.read_resource("file://config.json").await?;

// Get prompt templates
let prompts = client.list_prompts().await?;

// Get a prompt with arguments
let prompt = client.get_prompt("generate-code", Some(json!({
    "language": "rust",
    "description": "HTTP client"
}))).await?;
```

## 🔧 Feature Flags

| Feature | Description | Default |
|---------|-------------|---------|
| `stdio` | STDIO transport support | ✅ |
| `http` | HTTP transport support | ❌ |
| `websocket` | WebSocket transport support | ❌ |

## 📊 Usage Patterns

### Tool Discovery and Execution

```rust
// Discover what tools are available
let tools = client.list_tools().await?;

// Find a specific tool
let calc_tool = tools.iter()
    .find(|t| t.name == "calculate")
    .ok_or("Calculator tool not found")?;

// Examine tool schema
println!("Tool input schema: {:#}", calc_tool.input_schema);

// Call the tool with proper parameters
let result = client.call_tool("calculate", json!({
    "expression": "sqrt(16) + 2 * 3"
})).await?;
```

### Resource Access

```rust
// List available resources
let resources = client.list_resources().await?;

// Read configuration files
let config = client.read_resource("config://database").await?;

// Process resource content
match config.contents.first() {
    Some(content) => {
        if content.mime_type.as_deref() == Some("application/json") {
            let json: serde_json::Value = serde_json::from_str(&content.text)?;
            println!("Database config: {:#}", json);
        }
    }
    None => println!("No content found"),
}
```

### Error Handling

```rust
use mcp_protocol_client::{ClientError, ClientResult};

async fn handle_tool_call() -> ClientResult<()> {
    match client.call_tool("risky-operation", json!({})).await {
        Ok(result) => {
            println!("Success: {:?}", result);
        }
        Err(ClientError::ToolNotFound(name)) => {
            eprintln!("Tool '{}' not available on server", name);
        }
        Err(ClientError::InvalidParams(msg)) => {
            eprintln!("Invalid parameters: {}", msg);
        }
        Err(ClientError::Transport(e)) => {
            eprintln!("Connection error: {}", e);
            // Attempt reconnection
            client.reconnect().await?;
        }
        Err(e) => {
            eprintln!("Unexpected error: {}", e);
        }
    }
    
    Ok(())
}
```

## 🧪 Testing

```rust
use mcp_protocol_client::testing::MockServer;

#[tokio::test]
async fn test_client_tool_call() {
    let mut mock_server = MockServer::new();
    
    // Configure mock responses
    mock_server.expect_tool_call("echo")
        .with_params(json!({"message": "hello"}))
        .returning(json!({"response": "hello"}));
    
    let mut client = ClientBuilder::new("test-client", "1.0.0")
        .build();
    
    client.connect_mock(mock_server).await?;
    client.initialize().await?;
    
    let result = client.call_tool("echo", json!({
        "message": "hello"
    })).await?;
    
    assert_eq!(result.content[0].text, "hello");
}
```

## 🛠️ Development

```bash
# Build the crate
cargo build

# Run tests
cargo test

# Run with all features
cargo check --all-features

# Generate documentation
cargo doc --open
```

## 🔗 Related Crates

- [`mcp-protocol-types`](https://github.com/mcp-rust/mcp-protocol-types) - Core protocol types
- [`mcp-protocol-server`](https://github.com/mcp-rust/mcp-protocol-server) - Server library
- [`mcp-protocol-sdk`](https://github.com/mcp-rust/mcp-protocol-sdk) - Full-featured SDK

## 🤝 Contributing

This crate is part of the [MCP Rust ecosystem](https://github.com/mcp-rust). Contributions are welcome!

### Guidelines
- **API Design** - Keep the API simple and ergonomic
- **Performance** - Optimize for low latency and memory usage
- **Documentation** - All public APIs need examples
- **Testing** - Comprehensive test coverage required

## 📋 Protocol Compliance

✅ **MCP 2024-11-05 Specification**

This library implements the complete MCP client specification:
- JSON-RPC 2.0 protocol handling
- Capability negotiation and initialization
- Tool discovery and execution
- Resource access and content retrieval
- Prompt template processing
- Logging and debugging support
- Error handling and recovery

## 📄 License

Licensed under the [MIT License](./LICENSE).

## 🙏 Acknowledgments

- **Anthropic** - For creating the MCP specification
- **Tokio Team** - For the excellent async runtime
- **Rust Community** - For the amazing ecosystem

---

*Lightweight MCP client library for Rust 🦀*
