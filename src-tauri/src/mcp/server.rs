/// MCP Server — ADLER yeteneklerini dış dünyaya açar (§9.1).
pub struct McpServer;

impl McpServer {
    pub fn new() -> Self {
        Self
    }

    pub async fn start(&self, _port: u16) -> Result<(), String> {
        Ok(())
    }
}
