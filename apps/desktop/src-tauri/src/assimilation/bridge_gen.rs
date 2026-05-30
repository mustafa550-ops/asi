use crate::assimilation::analyzer::Analysis;

pub struct BridgeGenerator;

impl BridgeGenerator {
    pub fn generate_tauri_command(name: &str, analysis: &Analysis) -> String {
        let safe_name = name.replace('-', "_").replace(' ', "_");
        match analysis.language.as_str() {
            "Rust" => Self::rust_bridge(&safe_name),
            "TypeScript" | "JavaScript" => Self::js_bridge(&safe_name),
            "Python" => Self::python_bridge(&safe_name),
            _ => format!("// {} icin bridge kodu otomatik uretilemedi", name),
        }
    }

    fn rust_bridge(name: &str) -> String {
        format!(
            r##"// Auto-generated Tauri command for {name}
use tauri::State;
use crate::AppState;

#[tauri::command]
fn {name}_command(state: State<AppState>, input: String) -> Result<String, String> {{
    log::info!("{name}_command called with: {{}}", input);
    // TODO: Implement {name} integration
    Ok(format!("{name} processed: {{}}", input))
}}
"##,
            name = name
        )
    }

    fn js_bridge(name: &str) -> String {
        format!(
            r##"// Auto-generated invoke wrapper for {name}
import {{ invoke }} from "../lib/tauri";

export async function {name}Command(input: string): Promise<string> {{
    try {{
        return await invoke<string>("{name}_command", {{ input }});
    }} catch (err) {{
        console.error("{name} error:", err);
        return `Error: ${{err}}`;
    }}
}}
"##,
            name = name
        )
    }

    fn python_bridge(name: &str) -> String {
        format!(
            r##"# Auto-generated Python bridge for {name}
import subprocess
import json

class {name}Bridge:
    def __init__(self, host="127.0.0.1", port=9876):
        self.host = host
        self.port = port
    
    def call(self, method: str, params: dict) -> dict:
        payload = json.dumps({{
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
            "id": 1
        }})
        # TODO: Implement MCP call
        return {{"result": "not implemented"}}

    def process(self, input_data: str) -> str:
        return self.call("{name}_process", {{"input": input_data}})
"##,
            name = name
        )
    }
}
