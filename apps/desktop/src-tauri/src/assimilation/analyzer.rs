use std::fs;
use std::path::Path;

pub struct RepoAnalyzer;

#[derive(Debug, Clone)]
pub struct Analysis {
    pub language: String,
    pub framework: Option<String>,
    pub has_readme: bool,
    pub readme_summary: String,
    pub file_count: u32,
    pub build_files: Vec<String>,
    pub dependencies: Vec<String>,
    pub languages_detected: Vec<String>,
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

        let mut build_files = Vec::new();
        let mut dependencies = Vec::new();
        let mut language = "unknown".to_string();
        let mut framework = None;
        let mut has_readme = false;
        let mut readme_summary = String::new();
        let mut languages_detected = Vec::new();
        let mut file_count = 0u32;

        self.scan_directory(path, &mut file_count, &mut build_files, &mut languages_detected, &mut dependencies, &mut has_readme, &mut readme_summary)?;
        self.detect_language(&build_files, &languages_detected, &mut language, &mut framework, path)?;

        Ok(Analysis {
            language,
            framework,
            has_readme,
            readme_summary,
            file_count,
            build_files,
            dependencies,
            languages_detected,
        })
    }

    fn scan_directory(
        &self,
        dir: &Path,
        file_count: &mut u32,
        build_files: &mut Vec<String>,
        languages: &mut Vec<String>,
        dependencies: &mut Vec<String>,
        has_readme: &mut bool,
        readme_summary: &mut String,
    ) -> Result<(), String> {
        if !dir.is_dir() { return Ok(()); }

        let entries = fs::read_dir(dir).map_err(|e| format!("Cannot read dir {:?}: {}", dir, e))?;

        for entry in entries.flatten() {
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();

            if name.starts_with('.') && name != ".env" && name != ".gitignore" && !name.ends_with(".csproj") {
                continue; // skip hidden dirs
            }
            if name == "node_modules" || name == "target" || name == ".git" || name == "__pycache__" || name == "vendor" {
                continue;
            }

            if path.is_dir() {
                self.scan_directory(&path, file_count, build_files, languages, dependencies, has_readme, readme_summary)?;
            } else if path.is_file() {
                *file_count += 1;

                match name.as_str() {
                    "Cargo.toml" => {
                        build_files.push(name.clone());
                        if let Ok(content) = fs::read_to_string(&path) {
                            let mut in_deps = false;
                            for line in content.lines() {
                                let t = line.trim();
                                if t.starts_with("[dependencies]") { in_deps = true; continue; }
                                if t.starts_with('[') && t.ends_with(']') && in_deps { in_deps = false; continue; }
                                if in_deps {
                                    if let Some(eq) = t.find('=') {
                                        let dep = t[..eq].trim().to_string();
                                        if !dep.is_empty() && !dep.starts_with('#') { dependencies.push(dep); }
                                    }
                                }
                            }
                        }
                        languages.push("Rust".into());
                    }
                    "package.json" => {
                        build_files.push(name.clone());
                        if let Ok(content) = fs::read_to_string(&path) {
                            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                                if let Some(d) = json["dependencies"].as_object() {
                                    for (k, _) in d { dependencies.push(k.clone()); }
                                }
                            }
                        }
                        languages.push("JavaScript/TypeScript".into());
                    }
                    "requirements.txt" => {
                        build_files.push(name.clone());
                        if let Ok(content) = fs::read_to_string(&path) {
                            for line in content.lines() {
                                let t = line.trim();
                                if !t.is_empty() && !t.starts_with('#') {
                                    dependencies.push(t.split(&['=', '>', '<', '~', ';'][..]).next().unwrap_or(t).trim().to_string());
                                }
                            }
                        }
                        languages.push("Python".into());
                    }
                    "pyproject.toml" => {
                        build_files.push(name.clone());
                        languages.push("Python".into());
                    }
                    "go.mod" => {
                        build_files.push(name.clone());
                        languages.push("Go".into());
                    }
                    "Gemfile" => {
                        build_files.push(name.clone());
                        languages.push("Ruby".into());
                    }
                    "README.md" | "README" => {
                        *has_readme = true;
                        if let Ok(content) = fs::read_to_string(&path) {
                            let lines: Vec<&str> = content.lines().take(20).collect();
                            *readme_summary = lines.join("\n");
                        }
                    }
                    _ => {
                        if name.ends_with(".rs") { languages.push("Rust".into()); }
                        else if name.ends_with(".py") { languages.push("Python".into()); }
                        else if name.ends_with(".js") || name.ends_with(".jsx") { languages.push("JavaScript".into()); }
                        else if name.ends_with(".ts") || name.ends_with(".tsx") { languages.push("TypeScript".into()); }
                        else if name.ends_with(".go") { languages.push("Go".into()); }
                        else if name.ends_with(".rb") { languages.push("Ruby".into()); }
                        else if name.ends_with(".cs") { languages.push("C#".into()); }
                        else if name.ends_with(".csproj") || name.ends_with(".sln") {
                            build_files.push(name.clone());
                            languages.push("C#".into());
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn detect_language(
        &self,
        build_files: &[String],
        languages: &[String],
        language: &mut String,
        framework: &mut Option<String>,
        path: &Path,
    ) -> Result<(), String> {
        if build_files.contains(&"Cargo.toml".to_string()) {
            *language = "Rust".into();
        } else if build_files.contains(&"package.json".to_string()) {
            *language = "JavaScript/TypeScript".into();
            let pkg = path.join("package.json");
            if let Ok(content) = fs::read_to_string(&pkg) {
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                    let deps = json["dependencies"].as_object().or_else(|| json["devDependencies"].as_object());
                    if let Some(d) = deps {
                        for key in d.keys() {
                            match key.as_str() {
                                "react" | "next" => *framework = Some("React/Next.js".into()),
                                "vue" | "nuxt" => *framework = Some("Vue/Nuxt".into()),
                                "express" | "fastify" => *framework = Some("Express/Fastify".into()),
                                "django" | "flask" => *framework = Some("Django/Flask".into()),
                                _ => {}
                            }
                        }
                    }
                }
            }
        } else if build_files.contains(&"requirements.txt".to_string()) || build_files.contains(&"pyproject.toml".to_string()) {
            *language = "Python".into();
            if let Ok(content) = fs::read_to_string(path.join("requirements.txt")) {
                if content.contains("django") { *framework = Some("Django".into()); }
                else if content.contains("flask") { *framework = Some("Flask".into()); }
                else if content.contains("fastapi") { *framework = Some("FastAPI".into()); }
            }
        } else if build_files.contains(&"go.mod".to_string()) {
            *language = "Go".into();
        } else if build_files.contains(&"Gemfile".to_string()) {
            *language = "Ruby".into();
        } else if languages.contains(&"C#".to_string()) || build_files.iter().any(|f| f.ends_with(".csproj")) {
            *language = "C#".into();
        } else if languages.contains(&"Rust".to_string()) {
            *language = "Rust".into();
        } else if languages.contains(&"Python".to_string()) {
            *language = "Python".into();
        } else if languages.contains(&"JavaScript".to_string()) || languages.contains(&"TypeScript".to_string()) {
            *language = "JavaScript/TypeScript".into();
        } else if languages.contains(&"Go".to_string()) {
            *language = "Go".into();
        } else if languages.contains(&"Ruby".to_string()) {
            *language = "Ruby".into();
        }

        Ok(())
    }
}
