/// MCP Client — Harici MCP server'lara bağlanır (§9.1).
pub struct McpClient;

impl McpClient {
    pub fn new() -> Self {
        Self
    }

    pub async fn connect(&self, _url: &str) -> Result<(), String> {
        Ok(())
    }
}
