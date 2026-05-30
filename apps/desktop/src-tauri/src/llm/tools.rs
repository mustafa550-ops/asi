#[derive(Debug, Clone)]
pub enum ToolCall {
    ReadFile { path: String },
    SearchMemory { query: String },
    ExecuteCommand { command: String },
    AnalyzeMarket { symbol: String },
    ControlHardware { pin: u32, state: String },
    GenerateReport { format: String },
}

pub struct FunctionCallParser;

impl FunctionCallParser {
    pub fn parse(response: &str) -> Option<ToolCall> {
        let cleaned = response.trim();

        if let Some(tool) = Self::try_parse_json(cleaned) {
            return Some(tool);
        }

        if let Some(tool) = Self::try_parse_text(cleaned) {
            return Some(tool);
        }

        None
    }

    fn try_parse_json(text: &str) -> Option<ToolCall> {
        let json: serde_json::Value = serde_json::from_str(text).ok()?;
        let tool = json.get("tool").or_else(|| json.get("name"))?.as_str()?;
        let args = json.get("args").or_else(|| json.get("arguments"));
        match tool {
            "read_file" | "readFile" | "oku" => {
                let path = args?.get("path")?.as_str()?;
                Some(ToolCall::ReadFile { path: path.to_string() })
            }
            "search_memory" | "search" | "ara" => {
                let query = args?.get("query")?.as_str()?;
                Some(ToolCall::SearchMemory { query: query.to_string() })
            }
            "execute" | "run" | "calistir" => {
                let cmd = args?.get("command").or_else(|| args?.get("cmd"))?.as_str()?;
                Some(ToolCall::ExecuteCommand { command: cmd.to_string() })
            }
            "analyze_market" | "market_analysis" | "analiz" => {
                let symbol = args?.get("symbol")?.as_str()?;
                Some(ToolCall::AnalyzeMarket { symbol: symbol.to_string() })
            }
            "control_hardware" | "hardware" | "gpio" => {
                let pin = args?.get("pin")?.as_u64()? as u32;
                let state = args?.get("state")?.as_str().unwrap_or("on").to_string();
                Some(ToolCall::ControlHardware { pin, state })
            }
            _ => None,
        }
    }

    fn try_parse_text(text: &str) -> Option<ToolCall> {
        let lower = text.to_lowercase();
        if lower.starts_with("oku ") || lower.starts_with("read ") {
            let path = lower[4..].trim().to_string();
            return Some(ToolCall::ReadFile { path });
        }
        if lower.starts_with("ara ") || lower.starts_with("search ") {
            let query = lower[4..].trim().to_string();
            return Some(ToolCall::SearchMemory { query });
        }
        if lower.starts_with("calistir ") || lower.starts_with("run ") {
            let cmd = lower.split_at(4).1.trim().to_string();
            return Some(ToolCall::ExecuteCommand { command: cmd });
        }
        None
    }
}
