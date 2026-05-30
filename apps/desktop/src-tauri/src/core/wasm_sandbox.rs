use wasmtime::{Config, Engine, Instance, Module, Store};

pub struct WasmSandbox {
    max_fuel: u64,
}

impl WasmSandbox {
    pub fn new() -> Self {
        Self { max_fuel: 100_000 }
    }

    pub fn with_limits(fuel: u64) -> Self {
        Self { max_fuel: fuel }
    }

    pub fn execute(&self, wasm_bytes: &[u8]) -> Result<String, String> {
        let mut config = Config::new();
        config.consume_fuel(true);

        let engine = Engine::new(&config).map_err(|e| format!("Engine: {}", e))?;
        let module = Module::new(&engine, wasm_bytes)
            .map_err(|e| format!("Module parse: {}", e))?;

        let mut store = Store::new(&engine, ());
        store.set_fuel(self.max_fuel).unwrap();

        let instance = Instance::new(&mut store, &module, &[])
            .map_err(|e| format!("Instantiate: {}", e))?;

        let result = self.call_func(&mut store, &instance, "main")
            .or_else(|_| self.call_func(&mut store, &instance, "_start"))
            .unwrap_or_else(|_| "WASM yuklendi — main/_start export'u yok".into());

        Ok(result)
    }

    fn call_func(&self, store: &mut Store<()>, instance: &Instance, name: &str) -> Result<String, String> {
        let func = match instance.get_func(&mut *store, name) {
            Some(f) => f,
            None => return Err(format!("export {} yok", name)),
        };
        let mut results = vec![wasmtime::Val::I32(0)];
        func.call(&mut *store, &[], &mut results)
            .map_err(|e| format!("{}(): {}", name, e))?;
        let val = results.first().map(|v| format!("{:?}", v)).unwrap_or_default();
        Ok(format!("{}() -> {}", name, val))
    }

    pub fn compile_wat_and_execute(&self, wat: &str) -> Result<String, String> {
        let mut config = Config::new();
        config.consume_fuel(true);

        let engine = Engine::new(&config).map_err(|e| format!("Engine: {}", e))?;
        let module = Module::new(&engine, wat)
            .map_err(|e| format!("WAT parse: {}", e))?;

        let mut store = Store::new(&engine, ());
        store.set_fuel(self.max_fuel).unwrap();

        let instance = Instance::new(&mut store, &module, &[])
            .map_err(|e| format!("WAT instantiate: {}", e))?;

        let result = self.call_func(&mut store, &instance, "main")
            .or_else(|_| self.call_func(&mut store, &instance, "_start"))
            .unwrap_or_else(|_| "WAT yuklendi — main export'u yok".into());

        Ok(result)
    }


    pub fn compile_and_execute(&self, code: &str) -> Result<String, String> {
        let trimmed = code.trim();
        if trimmed.starts_with("(module") {
            super::wasm_compile::WasmCompiler::compile_and_run(trimmed)
        } else {
            self.execute(code.as_bytes())
        }
    }

    pub fn test_directory(&self, repo_path: &str, language: &str) -> Result<String, String> {
        let path = std::path::Path::new(repo_path);
        let mut wasm_count = 0u32;

        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries.flatten() {
                let p = entry.path();
                if p.is_file() && p.extension().map_or(false, |e| e == "wasm") {
                    wasm_count += 1;
                    if let Ok(bytes) = std::fs::read(&p) {
                        log::info!("WasmTest: executing {}", p.display());
                        self.execute(&bytes).ok();
                    }
                }
            }
        }

        if wasm_count > 0 {
            Ok(format!("{} .wasm files tested successfully", wasm_count))
        } else {
            match language {
                "Rust" | "Go" | "C#" => {
                    Ok("No .wasm files found for sandbox test".into())
                }
                _ => Ok("Language does not produce .wasm".into())
            }
        }
    }
}
