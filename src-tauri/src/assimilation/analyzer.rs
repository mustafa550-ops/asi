use std::fs;
use std::path::Path;

pub struct RepoAnalyzer;

#[derive(Debug, Clone)]
pub struct Analysis {
    pub language: String,
    pub has_readme: bool,
    pub file_count: u32,
    pub build_files: Vec<String>,
    pub dependencies: Vec<String>,
}

impl RepoAnalyzer {
    pub fn new() -> Self {
        Self
    }

    pub fn analyze(&self, repo_path: &str) -> Result<Analysis, String> {
        let path = Path::new(repo_path);
        if !path.exists() {
            return Err(format!("Repo path '{}' does not exist", repo_path));
        }

        let mut file_count = 0u32;
        let mut build_files = Vec::new();
        let mut dependencies = Vec::new();
        let mut language = "unknown".to_string();
        let mut has_readme = false;

        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                match name.as_str() {
                    "Cargo.toml" => {
                        language = "Rust".into();
                        build_files.push(name.clone());
                        if let Ok(content) = fs::read_to_string(entry.path()) {
                            for line in content.lines() {
                                if line.trim().starts_with("name =") {
                                    dependencies.push(line.trim().trim_matches(',').to_string());
                                }
                            }
                        }
                    }
                    "package.json" => {
                        if language == "unknown" {
                            language = "TypeScript/JavaScript".into();
                        }
                        build_files.push(name.clone());
                    }
                    "requirements.txt" | "pyproject.toml" => {
                        if language == "unknown" {
                            language = "Python".into();
                        }
                        build_files.push(name.clone());
                    }
                    "README.md" | "README" => {
                        has_readme = true;
                    }
                    _ => {}
                }

                if entry.path().is_file() {
                    file_count += 1;
                }
            }
        }

        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                if entry.path().is_dir() && !entry.file_name().to_string_lossy().starts_with('.') {
                    if let Ok(sub) = fs::read_dir(entry.path()) {
                        file_count += sub.flatten().count() as u32;
                    }
                }
            }
        }

        Ok(Analysis {
            language,
            has_readme,
            file_count,
            build_files,
            dependencies,
        })
    }
}
