use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct MCPTool {
    pub name: String,
    pub description: String,
    pub approval_required: bool,
    pub version: u32,
    pub input_schema: serde_json::Value,
    pub output_schema: serde_json::Value,
}

pub struct MCPToolRegistry {
    tools: HashMap<String, MCPTool>,
}

impl MCPToolRegistry {
    pub fn new() -> Self {
        let mut tools = HashMap::new();
        tools.insert("read_file".into(), MCPTool {
            name: "read_file".into(),
            description: "Dosya icerigini okur".into(),
            approval_required: false,
            version: 1,
            input_schema: serde_json::json!({"type": "object", "properties": {"path": {"type": "string"}}}),
            output_schema: serde_json::json!({"type": "string"}),
        });
        tools.insert("search_memory".into(), MCPTool {
            name: "search_memory".into(),
            description: "Bellekte semantik arama yapar".into(),
            approval_required: false,
            version: 1,
            input_schema: serde_json::json!({"type": "object", "properties": {"query": {"type": "string"}}}),
            output_schema: serde_json::json!({"type": "array"}),
        });
        tools.insert("execute_skill".into(), MCPTool {
            name: "execute_skill".into(),
            description: "Bir yetenegi calistirir".into(),
            approval_required: true,
            version: 1,
            input_schema: serde_json::json!({"type": "object", "properties": {"name": {"type": "string"}}}),
            output_schema: serde_json::json!({"type": "string"}),
        });
        tools.insert("system_status".into(), MCPTool {
            name: "system_status".into(),
            description: "Sistem durumunu raporlar".into(),
            approval_required: false,
            version: 1,
            input_schema: serde_json::json!({"type": "object"}),
            output_schema: serde_json::json!({"type": "object"}),
        });
        tools.insert("control_hardware".into(), MCPTool {
            name: "control_hardware".into(),
            description: "GPIO pin veya role kontrolu".into(),
            approval_required: true,
            version: 1,
            input_schema: serde_json::json!({"type": "object", "properties": {"pin": {"type": "number"}, "state": {"type": "string"}}}),
            output_schema: serde_json::json!({"type": "string"}),
        });
        Self { tools }
    }

    pub fn list(&self) -> Vec<MCPTool> {
        self.tools.values().cloned().collect()
    }

    pub fn get(&self, name: &str) -> Option<&MCPTool> {
        self.tools.get(name)
    }

    pub fn register(&mut self, tool: MCPTool) -> Result<(), String> {
        if self.tools.contains_key(&tool.name) {
            return Err(format!("Tool '{}' zaten kayitli", tool.name));
        }
        self.tools.insert(tool.name.clone(), tool);
        Ok(())
    }

    pub fn remove(&mut self, name: &str) -> Result<(), String> {
        let _ = self.tools.remove(name);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_builtin_tools() {
        let registry = MCPToolRegistry::new();
        let tools = registry.list();
        assert_eq!(tools.len(), 5);
    }

    #[test]
    fn test_get_tool() {
        let registry = MCPToolRegistry::new();
        let tool = registry.get("read_file");
        assert!(tool.is_some());
        assert_eq!(tool.unwrap().name, "read_file");
    }

    #[test]
    fn test_get_nonexistent() {
        let registry = MCPToolRegistry::new();
        assert!(registry.get("nonexistent").is_none());
    }

    #[test]
    fn test_register_new() {
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
    fn test_register_duplicate() {
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

    #[test]
    fn test_remove_tool() {
        let mut registry = MCPToolRegistry::new();
        assert!(registry.remove("read_file").is_ok());
        assert!(registry.get("read_file").is_none());
    }
}
