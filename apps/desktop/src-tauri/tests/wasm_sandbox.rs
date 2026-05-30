use adler_asi_lib::core::wasm_sandbox::WasmSandbox;

#[test]
fn wasm_sandbox_creation_default() {
    let sandbox = WasmSandbox::new();
    let _ = sandbox; // No panic on creation
}

#[test]
fn wasm_sandbox_creation_with_limits() {
    let sandbox = WasmSandbox::with_limits(200_000);
    let _ = sandbox;
}

#[test]
fn wasm_sandbox_execute_invalid_bytes() {
    let sandbox = WasmSandbox::new();
    let result = sandbox.execute(&[0x00, 0x61, 0x73, 0x6d]); // partial wasm magic
    assert!(result.is_err(), "Invalid wasm should fail gracefully");
}

#[test]
fn wasm_sandbox_compile_and_execute_valid_wat() {
    let sandbox = WasmSandbox::new();
    let wat = r#"(module (func (export "main") (result i32) i32.const 42))"#;
    let result = sandbox.compile_and_execute(wat);
    assert!(result.is_ok(), "Valid WAT should compile and execute");
}
