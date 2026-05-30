use adler_asi_lib::core::self_healing::log_analyzer::{analyze, extract_context};
use adler_asi_lib::core::self_healing::patch::generate_fix;

#[test]
fn log_analyzer_detects_syntax_error() {
    let result = analyze("syntax error: unexpected token");
    assert_eq!(result, "syntax_error", "Should detect syntax error category");
}

#[test]
fn log_analyzer_detects_module_not_found() {
    let result = analyze("cannot find module `foo`");
    assert_eq!(result, "module_not_found");
}

#[test]
fn log_analyzer_detects_connection_error() {
    let result = analyze("connection refused: 127.0.0.1:11434");
    assert_eq!(result, "connection_error");
}

#[test]
fn log_analyzer_returns_unknown_for_generic_errors() {
    let result = analyze("INFO: Sistem baslatildi");
    assert_eq!(result, "bilinmeyen hata");
}

#[test]
fn log_analyzer_empty_input_returns_unknown() {
    let result = analyze("");
    assert_eq!(result, "bilinmeyen hata");
}

#[test]
fn log_analyzer_extract_context_with_error_format() {
    let error = "error[E0432]: cannot find module\n --> src/main.rs:10:5\n";
    let ctx = extract_context(error);
    assert!(!ctx.is_empty(), "Should extract context from formatted error");
}

#[test]
fn log_analyzer_extract_context_empty_input() {
    let ctx = extract_context("");
    assert!(ctx.is_empty(), "Empty input should produce empty context");
}

#[test]
fn patch_generate_fix_with_syntax_error() {
    let bad_code = "fn main { }";
    let result = generate_fix(bad_code, "syntax error: unexpected token");
    assert!(result.is_ok(), "Should be able to fix fn main syntax: {:?}", result.err());
    let fixed = result.unwrap();
    assert!(fixed.contains("main()"), "Fixed code should have main()");
}

#[test]
fn patch_generate_fix_returns_err_for_unfixable() {
    let code = "valid code";
    let result = generate_fix(code, "cannot find module `xyz`");
    assert!(result.is_err(), "Module-not-found errors are not auto-fixable");
}
