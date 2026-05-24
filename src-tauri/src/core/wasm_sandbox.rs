pub struct WasmSandbox;

impl WasmSandbox {
    pub fn new() -> Self {
        Self
    }

    pub fn execute(&self, wasm_bytes: &[u8]) -> Result<String, String> {
        let engine = wasmtime::Engine::default();
        let module = wasmtime::Module::new(&engine, wasm_bytes)
            .map_err(|e| format!("Wasm module parse error: {}", e))?;
        let mut store = wasmtime::Store::new(&engine, ());
        let instance = wasmtime::Instance::new(&mut store, &module, &[])
            .map_err(|e| format!("Wasm instantiate error: {}", e))?;

        if let Some(func) = instance.get_func(&mut store, "main") {
            let result = func.call(&mut store, &[], &mut [])
                .map_err(|e| format!("Wasm exec error: {}", e))?;
            Ok(format!("Wasm execution complete: {:?}", result))
        } else {
            Ok("Wasm module loaded (no 'main' export)".into())
        }
    }

    pub fn compile_and_execute(&self, _code: &str) -> Result<String, String> {
        Err("Direct source compilation not supported — use pre-compiled .wasm bytes".into())
    }
}
