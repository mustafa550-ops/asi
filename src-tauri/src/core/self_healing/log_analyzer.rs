pub fn analyze(error: &str) -> String {
    let error_lower = error.to_lowercase();

    if error_lower.contains("cannot find") || error_lower.contains("no such") || error_lower.contains("module not found") || error_lower.contains("unresolved import") {
        "module_not_found".into()
    } else if error_lower.contains("syntax") || error_lower.contains("parse error") || error_lower.contains("expected") || error_lower.contains("unexpected") {
        "syntax_error".into()
    } else if error_lower.contains("memory") || error_lower.contains("alloc") || error_lower.contains("out of") {
        "memory_error".into()
    } else if error_lower.contains("mismatch") || error_lower.contains("cannot convert") || error_lower.contains("type") {
        "type_mismatch".into()
    } else if error_lower.contains("connection") || error_lower.contains("timeout") || error_lower.contains("refused") || error_lower.contains("dns") {
        "connection_error".into()
    } else if error_lower.contains("permission") || error_lower.contains("denied") || error_lower.contains("forbidden") || error_lower.contains("access") {
        "permission_error".into()
    } else if error_lower.contains("404") || error_lower.contains("not found") {
        "not_found".into()
    } else if error_lower.contains("panic") || error_lower.contains("unwrapping") || error_lower.contains("unreachable") {
        "panic".into()
    } else {
        "bilinmeyen hata".into()
    }
}

pub fn extract_context(error: &str) -> Vec<(String, String)> {
    let mut contexts = Vec::new();

    for line in error.lines() {
        let line = line.trim();

        if line.contains("-->") {
            contexts.push(("location".into(), line.to_string()));
        }

        if line.starts_with("error[") {
            let end = line.find(']').unwrap_or(line.len());
            let code = &line[6..end];
            contexts.push(("error_code".into(), code.to_string()));
        }

        if line.contains("-->") {
            contexts.push(("location".into(), line.to_string()));
        }
    }

    contexts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyze_module_not_found() {
        assert_eq!(analyze("cannot find module `foo`"), "module_not_found");
        assert_eq!(analyze("unresolved import `bar::baz`"), "module_not_found");
    }

    #[test]
    fn test_analyze_syntax_error() {
        assert_eq!(analyze("syntax error: unexpected token"), "syntax_error");
        assert_eq!(analyze("expected `;`"), "syntax_error");
    }

    #[test]
    fn test_analyze_memory_error() {
        assert_eq!(analyze("out of memory"), "memory_error");
        assert_eq!(analyze("allocation error"), "memory_error");
    }

    #[test]
    fn test_analyze_unknown() {
        assert_eq!(analyze("something completely different happened"), "bilinmeyen hata");
    }

    #[test]
    fn test_extract_context() {
        let error = "error[E0432]: cannot find module\n --> src/main.rs:10:5\n";
        let ctx = extract_context(error);
        assert!(!ctx.is_empty());
    }
}
