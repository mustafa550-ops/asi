use std::path::Path;

pub struct CodeAdapter;

impl CodeAdapter {
    pub fn new() -> Self {
        Self
    }

    pub fn adapt(&self, code: &str, source_lang: &str) -> String {
        let header = "// [ADLER-ADAPTED] Converted from ".to_string() + source_lang + " to Rust\n\n";
        let body = match source_lang {
            "python" | "Python" => self.python_to_rust(code),
            "javascript" | "js" | "JavaScript" => self.js_to_rust(code),
            "typescript" | "ts" | "TypeScript" => self.js_to_rust(code),
            "go" | "Go" => self.go_to_rust(code),
            "ruby" | "Ruby" => self.ruby_to_rust(code),
            "csharp" | "c#" | "C#" => self.csharp_to_rust(code),
            _ => code.to_string(),
        };
        header + &body
    }

    pub fn adapt_directory(&self, repo_path: &str, language: &str) -> Vec<String> {
        let mut adapted = Vec::new();
        self.adapt_dir_recursive(Path::new(repo_path), language, &mut adapted);
        adapted
    }

    fn adapt_dir_recursive(&self, dir: &Path, language: &str, adapted: &mut Vec<String>) {
        if !dir.is_dir() { return; }
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    if !entry.file_name().to_string_lossy().starts_with('.') {
                        self.adapt_dir_recursive(&path, language, adapted);
                    }
                } else if path.is_file() {
                    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
                    let should_adapt = match language {
                        "Python" => ext == "py",
                        "JavaScript/TypeScript" | "JS/TS" => matches!(ext, "js" | "jsx" | "ts" | "tsx"),
                        "Go" => ext == "go",
                        "Ruby" => ext == "rb",
                        "C#" => ext == "cs",
                        "Rust" => ext == "rs",
                        _ => false,
                    };
                    if should_adapt {
                        if let Ok(content) = std::fs::read_to_string(&path) {
                            if language == "Rust" {
                                let result = self.adapt_rust_to_rust(&content, &path);
                                if std::fs::write(&path, &result).is_ok() {
                                    adapted.push(path.to_string_lossy().to_string());
                                }
                            } else {
                                let result = self.adapt(&content, language);
                                if result != content {
                                    let adapted_path = path.with_extension("rs");
                                    if std::fs::write(&adapted_path, &result).is_ok() {
                                        adapted.push(adapted_path.to_string_lossy().to_string());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn python_to_rust(&self, code: &str) -> String {
        let mut out = String::new();
        let mut indent = 0usize;

        for line in code.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() { out.push('\n'); continue; }

            if trimmed.starts_with("def ") {
                let rest = &trimmed[4..];
                if let Some(paren) = rest.find('(') {
                    let name = to_snake_case(&rest[..paren]);
                    out.push_str(&format!("{:indent$}pub fn {}(", "", name, indent = indent));
                    let args = &rest[paren + 1..];
                    if let Some(end) = args.rfind(')') {
                        let params = args[..end].trim();
                        if !params.is_empty() {
                            for (i, p) in params.split(',').enumerate() {
                                let p = p.trim().split(':').next().unwrap_or(p.trim()).trim();
                                if i > 0 { out.push_str(", "); }
                                out.push_str(&format!("{}: String", to_snake_case(p)));
                            }
                        }
                    }
                    out.push_str(") -> Result<String, String> {\n");
                    indent += 4;
                }
            } else if trimmed.starts_with("class ") {
                let name = &trimmed[6..].split('(').next().unwrap_or("").split(':').next().unwrap_or("").trim();
                out.push_str(&format!("{:indent$}pub struct {} {{\n", "", name, indent = indent));
                indent += 4;
            } else if trimmed.starts_with("return ") {
                let val = trimmed[7..].trim().trim_end_matches(';');
                out.push_str(&format!("{:indent$}Ok({}.into())\n", "", val, indent = indent));
            } else if trimmed.starts_with("import ") || trimmed.starts_with("from ") {
                // skip
            } else if trimmed == "pass" {
                let indent_s = " ".repeat(indent);
                out.push_str(&format!("{} {{\n{}}}\n", indent_s, indent_s));
            } else if trimmed == "self" || trimmed.starts_with("self.") {
                let rest = trimmed.trim_start_matches("self").trim_start_matches('.');
                out.push_str(&format!("{:indent$}self.{}", "", rest, indent = indent));
                out.push('\n');
            } else if trimmed.contains("println!(") {
                out.push_str(&format!("{:indent$}{}\n", "", trimmed, indent = indent));
            } else if trimmed.contains("print(") || trimmed.contains("print(") {
                let rest = trimmed.trim();
                if let Some(content) = rest.strip_prefix("print(").and_then(|s| s.strip_suffix(')')) {
                    out.push_str(&format!("{:indent$}println!(\"{{}}\", {});\n", "", content.trim(), indent = indent));
                } else {
                    out.push_str(&format!("{:indent$}{}\n", "", trimmed, indent = indent));
                }
            } else if trimmed.ends_with(':') && !trimmed.starts_with("def ") && !trimmed.starts_with("class ") && !trimmed.starts_with("if ") && !trimmed.starts_with("elif ") && !trimmed.starts_with("else") && !trimmed.starts_with("for ") && !trimmed.starts_with("while ") && !trimmed.starts_with("try") && !trimmed.starts_with("except") && !trimmed.starts_with("with ") && !trimmed.starts_with("async ") && !trimmed.starts_with("match ") {
                out.push_str(&format!("{:indent$}{}\n", "", line, indent = indent));
            } else {
                out.push_str(&format!("{:indent$}{}\n", "", line, indent = indent));
            }

            if trimmed.ends_with(':') && !trimmed.starts_with('#') {
                indent += 4;
            }
        }

        out
    }

    fn js_to_rust(&self, code: &str) -> String {
        let mut out = String::new();
        let mut indent = 0usize;

        for line in code.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() { out.push('\n'); continue; }

            if trimmed.starts_with("function ") {
                let rest = &trimmed[9..];
                if let Some(paren) = rest.find('(') {
                    let name = to_snake_case(&rest[..paren]);
                    out.push_str(&format!("{:indent$}pub fn {}(", "", name, indent = indent));
                    let args = &rest[paren + 1..];
                    if let Some(end) = args.rfind(')') {
                        let params = args[..end].trim();
                        if !params.is_empty() {
                            for (i, p) in params.split(',').enumerate() {
                                if i > 0 { out.push_str(", "); }
                                out.push_str(&format!("{}: String", to_snake_case(p.trim())));
                            }
                        }
                    }
                    out.push_str(") -> Result<String, String> {\n");
                    indent += 4;
                }
            } else if trimmed.starts_with("async function ") {
                let rest = &trimmed[15..];
                if let Some(paren) = rest.find('(') {
                    let name = to_snake_case(&rest[..paren]);
                    out.push_str(&format!("{:indent$}pub async fn {}(", "", name, indent = indent));
                    let args = &rest[paren + 1..];
                    if let Some(end) = args.rfind(')') {
                        let params = args[..end].trim();
                        if !params.is_empty() {
                            for (i, p) in params.split(',').enumerate() {
                                if i > 0 { out.push_str(", "); }
                                out.push_str(&format!("{}: String", to_snake_case(p.trim())));
                            }
                        }
                    }
                    out.push_str(") -> Result<String, String> {\n");
                    indent += 4;
                }
            } else if trimmed.starts_with("const ") || trimmed.starts_with("let ") || trimmed.starts_with("var ") {
                let rest = trimmed.trim_start_matches("const ").trim_start_matches("let ").trim_start_matches("var ");
                if let Some(eq) = rest.find('=') {
                    let var_name = to_snake_case(rest[..eq].trim());
                    let value = rest[eq + 1..].trim().trim_end_matches(';');
                    if value.starts_with('[') {
                        out.push_str(&format!("{:indent$}let {} = vec!{};\n", "", var_name, value, indent = indent));
                    } else if value.starts_with('{') {
                        out.push_str(&format!("{:indent$}let {} = std::collections::HashMap::from({});\n", "", var_name, value, indent = indent));
                    } else {
                        out.push_str(&format!("{:indent$}let {} = {};\n", "", var_name, value, indent = indent));
                    }
                }
            } else if trimmed.starts_with("return ") {
                let val = trimmed[7..].trim().trim_end_matches(';');
                out.push_str(&format!("{:indent$}Ok({}.into())\n", "", val, indent = indent));
            } else if trimmed.starts_with("if ") || trimmed.starts_with("else ") || trimmed.starts_with("for ") || trimmed.starts_with("while ") {
                out.push_str(&format!("{:indent$}{}\n", "", trimmed, indent = indent));
                if trimmed.ends_with('{') || trimmed.ends_with(") {") { indent += 4; }
            } else if trimmed == "}" {
                indent = indent.saturating_sub(4);
                out.push_str(&format!("{:indent$}{}\n", "", trimmed, indent = indent));
            } else if !trimmed.is_empty() {
                out.push_str(&format!("{:indent$}{}\n", "", line.trim_end(), indent = indent));
            }
        }

        out
    }

    fn go_to_rust(&self, code: &str) -> String {
        let mut out = String::new();
        let mut indent = 0usize;

        for line in code.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() { out.push('\n'); continue; }

            if trimmed.starts_with("func ") {
                let rest = &trimmed[5..];
                if let Some(paren) = rest.find('(') {
                    let name = to_snake_case(rest[..paren].trim());
                    out.push_str(&format!("{:indent$}pub fn {}(", "", name, indent = indent));
                    let args = &rest[paren + 1..];
                    if let Some(end) = args.rfind(')') {
                        let params = args[..end].trim();
                        if !params.is_empty() {
                            for (i, p) in params.split(',').enumerate() {
                                let p = p.trim();
                                if p.is_empty() { continue; }
                                let parts: Vec<&str> = p.split_whitespace().collect();
                                if i > 0 { out.push_str(", "); }
                                if parts.len() >= 2 {
                                    out.push_str(&format!("{}: {}", to_snake_case(parts[0]), go_type_to_rust(parts[1])));
                                } else {
                                    out.push_str(&format!("{}: String", to_snake_case(p)));
                                }
                            }
                        }
                    }
                    out.push_str(") -> Result<String, String> {\n");
                    indent += 4;
                }
            } else if trimmed.starts_with("type ") && trimmed.contains("struct") {
                let name = trimmed[5..].split_whitespace().next().unwrap_or("").trim().to_string();
                out.push_str(&format!("{:indent$}pub struct {} {{\n", "", name, indent = indent));
                indent += 4;
            } else if trimmed.starts_with("type ") && trimmed.contains("interface") {
                let name = trimmed[5..].split_whitespace().next().unwrap_or("").trim().to_string();
                out.push_str(&format!("{:indent$}pub trait {} {{\n", "", name, indent = indent));
                indent += 4;
            } else if trimmed.starts_with("return ") {
                let val = trimmed[7..].trim().trim_end_matches(';');
                out.push_str(&format!("{:indent$}Ok({}.into())\n", "", val, indent = indent));
            } else if trimmed.starts_with("import (") || trimmed.starts_with("import \"") {
                // skip
            } else if trimmed.starts_with("var ") || trimmed.starts_with(":=") {
                let rest = trimmed.trim_start_matches("var ").trim_start_matches(":=").trim();
                if let Some(eq) = rest.find('=') {
                    let name = to_snake_case(rest[..eq].trim().split_whitespace().next().unwrap_or(""));
                    let val = rest[eq + 1..].trim().trim_end_matches(';');
                    out.push_str(&format!("{:indent$}let {} = {};\n", "", name, val, indent = indent));
                }
            } else if trimmed.starts_with("if ") || trimmed.starts_with("for ") || trimmed.starts_with("switch ") {
                out.push_str(&format!("{:indent$}{}\n", "", trimmed.replace("err != nil", "e.is_err()"), indent = indent));
                if trimmed.ends_with('{') { indent += 4; }
            } else if trimmed == "}" {
                indent = indent.saturating_sub(4);
                out.push_str(&format!("{:indent$}{}\n", "", trimmed, indent = indent));
            } else if !trimmed.is_empty() {
                out.push_str(&format!("{:indent$}{}\n", "", trimmed.trim_end_matches(';'), indent = indent));
            }
        }

        out
    }

    fn ruby_to_rust(&self, code: &str) -> String {
        let mut out = String::new();
        let mut indent = 0usize;
        for line in code.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() { out.push('\n'); continue; }

            if trimmed.starts_with("class ") {
                let class_name = trimmed[6..].split('<').next().unwrap_or("").split_whitespace().next().unwrap_or("").to_string();
                out.push_str(&format!("{:indent$}pub struct {} {{\n", "", class_name, indent = indent));
                indent += 4;
            } else if trimmed.starts_with("def ") {
                let rest = &trimmed[4..];
                if let Some(paren) = rest.find('(') {
                    let name = to_snake_case(rest[..paren].trim());
                    out.push_str(&format!("{:indent$}pub fn {}(", "", name, indent = indent));
                    let args = &rest[paren + 1..];
                    if let Some(end) = args.rfind(')') {
                        for (i, p) in args[..end].split(',').enumerate() {
                            let p = p.trim();
                            if p == "self" { continue; }
                            if i > 0 { out.push_str(", "); }
                            out.push_str(&format!("{}: String", to_snake_case(p)));
                        }
                    }
                    out.push_str(") -> Result<String, String> {\n");
                    indent += 4;
                } else {
                    let name = to_snake_case(rest.trim());
                    out.push_str(&format!("{:indent$}pub fn {}() -> Result<String, String> {{\n", "", name, indent = indent));
                    indent += 4;
                }
            } else if trimmed.starts_with("puts ") || trimmed.starts_with("print ") {
                let val = trimmed.trim_start_matches("puts ").trim_start_matches("print ");
                out.push_str(&format!("{:indent$}println!(\"{{}}\", {});\n", "", val, indent = indent));
            } else if trimmed.starts_with("return ") {
                let val = trimmed[7..].trim();
                out.push_str(&format!("{:indent$}Ok({}.into())\n", "", val, indent = indent));
            } else if trimmed.starts_with("@") {
                out.push_str(&format!("{:indent$}self.{}", "", to_snake_case(&trimmed[1..]), indent = indent));
                out.push('\n');
            } else if trimmed.starts_with("if ") || trimmed.starts_with("unless ") || trimmed.starts_with("while ") || trimmed.starts_with("for ") {
                let c = if trimmed.starts_with("unless ") {
                    let cond = &trimmed[7..];
                    format!("if !({})", cond)
                } else {
                    trimmed.to_string()
                };
                out.push_str(&format!("{:indent$}{} {{\n", "", c, indent = indent));
                indent += 4;
            } else if trimmed.starts_with("end") {
                indent = indent.saturating_sub(4);
            } else if trimmed.starts_with("attr_") || trimmed.starts_with("require ") || trimmed.starts_with("include ") {
                continue;
            } else if !trimmed.is_empty() {
                out.push_str(&format!("{:indent$}{}\n", "", trimmed, indent = indent));
            }
        }

        out
    }

    fn csharp_to_rust(&self, code: &str) -> String {
        let mut out = String::new();
        let mut indent = 0usize;

        for line in code.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() { out.push('\n'); continue; }

            if trimmed.starts_with("class ") {
                let name = trimmed[6..].split(':').next().unwrap_or("").split_whitespace().next().unwrap_or("").trim().to_string();
                out.push_str(&format!("{:indent$}pub struct {} {{\n", "", name, indent = indent));
                indent += 4;
            } else if trimmed.starts_with("public ") || trimmed.starts_with("private ") || trimmed.starts_with("protected ") || trimmed.starts_with("internal ") {
                let rest = trimmed.trim_start_matches("public ").trim_start_matches("private ").trim_start_matches("protected ").trim_start_matches("internal ");
                if let Some(paren) = rest.find('(') {
                    let before_paren = &rest[..paren];
                    let name_start = before_paren.rfind(|c: char| c == ' ' || c == '\t').map(|i| i + 1).unwrap_or(0);
                    let name = to_snake_case(&before_paren[name_start..].trim());
                    out.push_str(&format!("{:indent$}pub fn {}(", "", name, indent = indent));
                    let args = &rest[paren + 1..];
                    if let Some(end) = args.rfind(')') {
                        for (i, p) in args[..end].split(',').enumerate() {
                            let p = p.trim();
                            if p.is_empty() || p == "void" { continue; }
                            let parts: Vec<&str> = p.split_whitespace().collect();
                            if i > 0 { out.push_str(", "); }
                            if parts.len() >= 2 {
                                out.push_str(&format!("{}: {}", to_snake_case(parts.last().unwrap_or(&"")), cs_type_to_rust(parts[0])));
                            } else {
                                out.push_str(&format!("{}: String", to_snake_case(p)));
                            }
                        }
                    }
                    out.push_str(") -> Result<String, String> {\n");
                    indent += 4;
                } else if rest.starts_with("string ") || rest.starts_with("int ") || rest.starts_with("bool ") || rest.starts_with("void ") {
                    // field declaration
                    let parts: Vec<&str> = rest.split_whitespace().collect();
                    if parts.len() >= 2 {
                        out.push_str(&format!("{:indent$}{}: {},\n", "", to_snake_case(parts[1]), cs_type_to_rust(parts[0]), indent = indent));
                    }
                }
            } else if trimmed.starts_with("return ") {
                let val = trimmed[7..].trim().trim_end_matches(';');
                out.push_str(&format!("{:indent$}Ok({}.into())\n", "", val, indent = indent));
            } else if trimmed.starts_with("using ") || trimmed.starts_with("namespace ") {
                continue;
            } else if trimmed.starts_with("if ") || trimmed.starts_with("else ") || trimmed.starts_with("for ") || trimmed.starts_with("foreach ") || trimmed.starts_with("while ") || trimmed.starts_with("switch ") {
                let c = if trimmed.starts_with("foreach ") {
                    let rest_inner = &trimmed[8..].replace(" in ", " in ");
                    rest_inner.to_string()
                } else {
                    trimmed.to_string()
                };
                out.push_str(&format!("{:indent$}{}\n", "", c, indent = indent));
                if trimmed.ends_with('{') { indent += 4; }
            } else if trimmed == "}" {
                indent = indent.saturating_sub(4);
                out.push_str(&format!("{:indent$}{}\n", "", trimmed, indent = indent));
            } else if !trimmed.is_empty() {
                out.push_str(&format!("{:indent$}{}\n", "", trimmed.trim_end_matches(';'), indent = indent));
            }
        }

        out
    }
}

impl CodeAdapter {
    fn adapt_rust_to_rust(&self, code: &str, path: &std::path::Path) -> String {
        let file_name = path.file_stem().and_then(|s| s.to_str()).unwrap_or("unknown");
        if code.trim().starts_with("fn ") || code.trim().starts_with("pub fn ") {
            let mut out = String::new();
            out.push_str("use crate::*;\n\n");
            out.push_str(code);
            out
        } else if code.contains("mod ") || code.contains("use ") {
            code.to_string()
        } else {
            let mut out = String::new();
            out.push_str(&format!("pub mod {};\n\n", file_name));
            out.push_str(code);
            out
        }
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

fn go_type_to_rust(t: &str) -> &str {
    match t.trim() {
        "string" => "String",
        "int" | "int64" => "i64",
        "int32" => "i32",
        "float64" => "f64",
        "float32" => "f32",
        "bool" => "bool",
        "error" => "String",
        "[]byte" => "Vec<u8>",
        "interface{}" => "dyn std::any::Any",
        s => s,
    }
}

fn cs_type_to_rust(t: &str) -> &str {
    match t.trim() {
        "string" => "String",
        "int" => "i32",
        "long" => "i64",
        "float" => "f32",
        "double" => "f64",
        "bool" => "bool",
        "void" => "()",
        "Task" | "async Task" => "()",
        "List" => "Vec",
        "Dictionary" => "HashMap",
        "object" => "Box<dyn std::any::Any>",
        s => s,
    }
}
