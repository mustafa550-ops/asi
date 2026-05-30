use std::path::Path;

#[derive(Debug)]
pub struct AnalysisResult {
    pub language: String,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub file_count: u32,
    pub has_cargo_check: bool,
}

pub struct StaticAnalyzer;

impl StaticAnalyzer {
    pub fn analyze(path: &Path, language: &str) -> AnalysisResult {
        match language {
            "Rust" => Self::analyze_rust(path),
            "TypeScript" | "JavaScript" => Self::analyze_js(path),
            _ => AnalysisResult {
                language: language.to_string(),
                errors: vec![],
                warnings: vec!["Static analysis not supported for this language".into()],
                file_count: 0,
                has_cargo_check: false,
            },
        }
    }

    fn analyze_rust(path: &Path) -> AnalysisResult {
        let mut errors = Vec::new();
        let warnings = Vec::new();

        let has_cargo_toml = path.join("Cargo.toml").exists();
        if !has_cargo_toml {
            // warnings.push("Cargo.toml bulunamadi — Rust projesi olmayabilir".into());
        }

        let file_count = Self::count_files(path, &["rs"]);
        let has_cargo_check = has_cargo_toml && Self::run_cargo_check(path, &mut errors);

        AnalysisResult {
            language: "Rust".into(),
            errors,
            warnings,
            file_count,
            has_cargo_check,
        }
    }

    fn analyze_js(path: &Path) -> AnalysisResult {
        let errors = Vec::new();
        let warnings = Vec::new();

        let has_package_json = path.join("package.json").exists();
        if !has_package_json {} // package.json kontrolu

        let file_count = Self::count_files(path, &["ts", "tsx", "js", "jsx"]);

        AnalysisResult {
            language: "TypeScript".into(),
            errors,
            warnings,
            file_count,
            has_cargo_check: false,
        }
    }

    fn count_files(path: &Path, exts: &[&str]) -> u32 {
        let mut count = 0;
        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries.flatten() {
                let p = entry.path();
                if p.is_dir() {
                    let name = p.file_name().unwrap_or_default().to_string_lossy().to_string();
                    if !name.starts_with('.') && name != "node_modules" && name != "target" {
                        count += Self::count_files(&p, exts);
                    }
                } else if let Some(ext) = p.extension() {
                    if exts.contains(&ext.to_str().unwrap_or("")) {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    fn run_cargo_check(path: &Path, errors: &mut Vec<String>) -> bool {
        let output = std::process::Command::new("cargo")
            .arg("check")
            .current_dir(path)
            .output();
        match output {
            Ok(out) => {
                if !out.status.success() {
                    let stderr = String::from_utf8_lossy(&out.stderr);
                    let error_lines: Vec<_> = stderr.lines()
                        .filter(|l| l.contains("error["))
                        .take(5)
                        .map(|l| l.to_string())
                        .collect();
                    for e in error_lines {
                        errors.push(e);
                    }
                }
                out.status.success()
            }
            Err(e) => {
                errors.push(format!("cargo check calistirilamadi: {}", e));
                false
            }
        }
    }
}
