use adler_asi_lib::mcp::registry::{MCPToolRegistry, MCPTool};

#[test]
fn mcp_tool_registry_initializes_with_builtin_tools() {
    let registry = MCPToolRegistry::new();
    let tools = registry.list();
    assert_eq!(tools.len(), 5, "Should have 5 built-in tools");
}

#[test]
fn mcp_tool_get_builtin() {
    let registry = MCPToolRegistry::new();
    let tool = registry.get("read_file");
    assert!(tool.is_some());
    assert_eq!(tool.unwrap().name, "read_file");
}

#[test]
fn mcp_tool_get_nonexistent() {
    let registry = MCPToolRegistry::new();
    assert!(registry.get("nonexistent").is_none());
}

#[test]
fn mcp_tool_register_new() {
    let mut registry = MCPToolRegistry::new();
    let tool = MCPTool {
        name: "custom_tool".into(),
        description: "test".into(),
        approval_required: false,
        version: 1,
        input_schema: serde_json::json!({}),
        output_schema: serde_json::json!({}),
    };
    assert!(registry.register(tool).is_ok());
    assert!(registry.get("custom_tool").is_some());
}

#[test]
fn mcp_tool_register_duplicate_fails() {
    let mut registry = MCPToolRegistry::new();
    let tool = MCPTool {
        name: "read_file".into(),
        description: "dup".into(),
        approval_required: false,
        version: 1,
        input_schema: serde_json::json!({}),
        output_schema: serde_json::json!({}),
    };
    assert!(registry.register(tool).is_err());
}
