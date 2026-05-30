use super::log_analyzer;

pub fn generate_fix(code: &str, error: &str) -> Result<String, String> {
    let analysis = log_analyzer::analyze(error);

    match analysis.as_str() {
        "syntax_error" => fix_syntax(code),
        "module_not_found" => Err("Module bulunamadi hatasi icin otomatik duzeltme desteklenmiyor".into()),
        "type_mismatch" => Err("Tip uyusmazligi icin otomatik duzeltme desteklenmiyor".into()),
        _ => Err(format!("'{}' icin otomatik duzeltme yok", analysis)),
    }
}

fn fix_syntax(code: &str) -> Result<String, String> {
    let mut result = code.to_string();

    if let Some(pos) = result.rfind(|c| c == ';' || c == '}') {
        let after = &result[pos + 1..].trim();
        if !after.is_empty() && !after.starts_with('\n') {
            result.insert(pos + 1, '\n');
            return Ok(result);
        }
    }

    if result.contains("fn main") && !result.contains("main(") {
        result = result.replace("fn main", "fn main()");
        return Ok(result);
    }

    Err("Syntax hatasi otomatik duzeltilemedi".into())
}

pub fn dry_run_and_fix(code: &str, error: &str, max_attempts: u32) -> Result<String, String> {
    let current_code = code.to_string();
    let mut last_error = error.to_string();

    for attempt in 0..max_attempts {
        log::info!("Patch denemesi {}/{}", attempt + 1, max_attempts);

        match generate_fix(&current_code, &last_error) {
            Ok(fixed) => {
                log::info!("Patch basarili (deneme {})", attempt + 1);
                return Ok(fixed);
            }
            Err(e) => {
                last_error = e;
            }
        }
    }

    Err(format!("{} denemeden sonra duzeltilemedi. Son hata: {}", max_attempts, last_error))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fix_syntax_newline() {
        let code = "fn main() { let x = 1 }let y = 2";
        let result = fix_syntax(code);
        assert!(result.is_ok());
        assert!(result.unwrap().contains('\n'));
    }

    #[test]
    fn test_fix_syntax_fn_main() {
        let code = "fn main { }";
        let result = fix_syntax(code);
        assert!(result.is_ok());
        assert!(result.unwrap().contains("main()"));
    }

    #[test]
    fn test_dry_run_and_fix_max_attempts() {
        let code = "fn main { }";
        let result = dry_run_and_fix(code, "syntax error", 3);
        assert!(result.is_ok());
    }

    #[test]
    fn test_dry_run_and_fix_unfixable() {
        let code = "valid code";
        let result = dry_run_and_fix(code, "cannot find module `xyz`", 2);
        assert!(result.is_err());
    }
}
