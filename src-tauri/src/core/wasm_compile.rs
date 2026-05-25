use wasmtime::{Engine, Module, Store, Instance};

pub struct WasmCompiler;

impl WasmCompiler {
    pub fn new() -> Self {
        Self
    }

    pub fn compile_wat(wat_source: &str) -> Result<Vec<u8>, String> {
        let engine = Engine::default();
        let module = Module::new(&engine, wat_source)
            .map_err(|e| format!("WAT derleme hatasi: {}", e))?;

        let bytes = module.serialize()
            .map_err(|e| format!("Serilestirme hatasi: {}", e))?;

        Ok(bytes.to_vec())
    }

    pub fn compile_and_run(wat_source: &str) -> Result<String, String> {
        let engine = Engine::default();
        let module = Module::new(&engine, wat_source)
            .map_err(|e| format!("Derleme hatasi: {}", e))?;

        let mut store = Store::new(&engine, ());
        let instance = Instance::new(&mut store, &module, &[])
            .map_err(|e| format!("Instance hatasi: {}", e))?;

        if let Ok(func) = instance.get_typed_func::<(), i32>(&mut store, "main") {
            let result = func.call(&mut store, ())
                .map_err(|e| format!("Calistirma hatasi: {}", e))?;
            Ok(format!("WAT execution complete — result: {}", result))
        } else {
            Ok("WAT module loaded (no 'main' export)".into())
        }
    }

    pub fn compile_rust_source(_source: &str) -> Result<Vec<u8>, String> {
        Err("Rust kaynaktan WASM derlemesi icin wasm-pack veya cargo-wasi gerekir. \
             Mevcut mod: WAT veya pre-compiled .wasm kabul edilir.".into())
    }

    pub fn verify_wasm_bytes(wasm_bytes: &[u8]) -> Result<String, String> {
        let engine = Engine::default();
        let module = Module::new(&engine, wasm_bytes)
            .map_err(|e| format!("Gecersiz WASM: {}", e))?;

        let exports: Vec<String> = module.exports().map(|e| {
            format!("{}: {:?}", e.name(), e.ty())
        }).collect();

        Ok(format!("WASM gecerli — exportlar: [{}]", exports.join(", ")))
    }
}
