use std::collections::HashMap;

pub enum WasmValue {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
}

pub struct WasmRuntime {
    pub fuel_limit: u64,
    pub memory_limit: usize,
}

impl WasmRuntime {
    pub fn new() -> Self {
        Self { fuel_limit: 100_000, memory_limit: 128 * 1024 * 1024 }
    }

    pub fn with_limits(fuel: u64, memory_mb: usize) -> Self {
        Self { fuel_limit: fuel, memory_limit: memory_mb * 1024 * 1024 }
    }

    pub fn compile_rust(source: &str) -> Result<Vec<u8>, String> {
        if !source.contains("fn ") {
            return Err("Kaynak kodda fonksiyon bulunamadi".into());
        }
        let wrapped = format!(
            "#[no_mangle]\npub extern \"C\" fn process(input: i32) -> i32 {{\n{}\n}}\n",
            source
        );
        Ok(wrapped.into_bytes())
    }

    pub fn validate_module(&self, wasm_bytes: &[u8]) -> Result<(), String> {
        if wasm_bytes.is_empty() {
            return Err("Wasm modulu bos".into());
        }
        if wasm_bytes.len() > self.memory_limit {
            return Err(format!("Wasm modulu cok buyuk: {} byte", wasm_bytes.len()));
        }
        Ok(())
    }

    pub fn estimate_fuel(source: &str) -> u64 {
        let lines = source.lines().count() as u64;
        let ops = source.len() as u64 / 4;
        (lines * 10 + ops).min(100_000)
    }
}

impl Default for WasmRuntime {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_runtime_defaults() {
        let rt = WasmRuntime::new();
        assert_eq!(rt.fuel_limit, 100_000);
        assert_eq!(rt.memory_limit, 128 * 1024 * 1024);
    }

    #[test]
    fn with_limits_sets_values() {
        let rt = WasmRuntime::with_limits(50_000, 64);
        assert_eq!(rt.fuel_limit, 50_000);
        assert_eq!(rt.memory_limit, 64 * 1024 * 1024);
    }

    #[test]
    fn compile_rust_basic() {
        let code = "fn process(input: i32) -> i32 { input + 1 }";
        let result = WasmRuntime::compile_rust(code);
        assert!(result.is_ok());
        assert!(!result.unwrap().is_empty());
    }

    #[test]
    fn compile_rust_no_function() {
        let code = "let x = 5;";
        assert!(WasmRuntime::compile_rust(code).is_err());
    }

    #[test]
    fn validate_empty_module() {
        let rt = WasmRuntime::new();
        assert!(rt.validate_module(&[]).is_err());
    }

    #[test]
    fn validate_oversized_module() {
        let rt = WasmRuntime::with_limits(100, 1);
        let huge = vec![0u8; 2 * 1024 * 1024];
        assert!(rt.validate_module(&huge).is_err());
    }

    #[test]
    fn validate_valid_module() {
        let rt = WasmRuntime::new();
        let data = vec![0u8; 100];
        assert!(rt.validate_module(&data).is_ok());
    }

    #[test]
    fn estimate_fuel_for_source() {
        let source = "fn main() {\n    let x = 1;\n    let y = 2;\n}\n";
        let fuel = WasmRuntime::estimate_fuel(source);
        assert!(fuel > 0);
        assert!(fuel <= 100_000);
    }

    #[test]
    fn compile_rust_with_extern() {
        let code = "pub fn run(task: &str) -> String { format!(\"OK: {}\", task) }";
        let result = WasmRuntime::compile_rust(code);
        assert!(result.is_ok());
    }
}
