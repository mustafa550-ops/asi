pub struct CodeAdapter;

impl CodeAdapter {
    pub fn new() -> Self {
        Self
    }

    pub fn adapt(&self, code: &str, source_lang: &str) -> String {
        match source_lang {
            "python" | "Python" => self.python_to_rust(code),
            "javascript" | "js" | "JavaScript" => self.js_to_rust(code),
            _ => code.to_string(),
        }
    }

    fn python_to_rust(&self, code: &str) -> String {
        let mut out = String::new();
        for line in code.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("def ") {
                let rest = &trimmed[4..];
                if let Some(paren) = rest.find('(') {
                    let name = &rest[..paren];
                    out.push_str(&format!("pub fn {}(", to_snake_case(name)));
                    let args = &rest[paren + 1..];
                    if let Some(end) = args.rfind(')') {
                        out.push_str(&args[..end]);
                        out.push_str(": String");
                    }
                    out.push_str(") -> Result<String, String> {\n");
                }
            } else if trimmed.starts_with("return ") {
                out.push_str(&format!("    Ok({}.into())\n", &trimmed[7..]));
            } else if trimmed.starts_with("import ") || trimmed.starts_with("from ") {
                // skip imports
            } else if trimmed == "pass" {
                out.push_str("    todo!()\n");
            } else if !trimmed.is_empty() {
                out.push_str(line);
                out.push('\n');
            }
        }
        out
    }

    fn js_to_rust(&self, code: &str) -> String {
        let mut out = String::new();
        for line in code.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("function ") {
                let rest = &trimmed[9..];
                if let Some(paren) = rest.find('(') {
                    let name = &rest[..paren];
                    out.push_str(&format!("pub fn {}(", to_snake_case(name)));
                    let args = &rest[paren + 1..];
                    if let Some(end) = args.rfind(')') {
                        out.push_str(&args[..end]);
                    }
                    out.push_str(": String) -> Result<String, String> {\n");
                }
            } else if trimmed.starts_with("async function ") {
                let rest = &trimmed[15..];
                if let Some(paren) = rest.find('(') {
                    let name = &rest[..paren];
                    out.push_str(&format!("pub async fn {}(", to_snake_case(name)));
                    let args = &rest[paren + 1..];
                    if let Some(end) = args.rfind(')') {
                        out.push_str(&args[..end]);
                    }
                    out.push_str(": String) -> Result<String, String> {\n");
                }
            } else if trimmed.starts_with("return ") {
                out.push_str(&format!("    Ok({}.into())\n", &trimmed[7..].trim_end_matches(';')));
            } else if trimmed.starts_with("const ") || trimmed.starts_with("let ") || trimmed.starts_with("var ") {
                if let Some(eq) = trimmed.find('=') {
                    let var_name = &trimmed[..eq].trim_start_matches("const ")
                        .trim_start_matches("let ").trim_start_matches("var ").trim();
                    if let Some(semi) = trimmed.rfind(';') {
                        let value = &trimmed[eq + 1..semi].trim();
                        out.push_str(&format!("    let {} = {};\n", var_name, value));
                    } else {
                        let value = &trimmed[eq + 1..].trim();
                        out.push_str(&format!("    let {} = {};\n", var_name, value));
                    }
                }
            } else if !trimmed.is_empty() {
                out.push_str(line);
                out.push('\n');
            }
        }
        out
    }
}

fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() && i > 0 {
            result.push('_');
        }
        result.push(c.to_lowercase().next().unwrap_or(c));
    }
    result
}
