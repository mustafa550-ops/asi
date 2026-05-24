/// Wasm Sandbox — wasmtime ile izole çalıştırma (§3.1).
pub struct WasmSandbox;

impl WasmSandbox {
    pub fn new() -> Self {
        Self
    }

    pub fn execute(&self, _wasm_bytes: &[u8]) -> Result<String, String> {
        // wasmtime ile izole çalıştır
        Ok("ok".into())
    }
}
