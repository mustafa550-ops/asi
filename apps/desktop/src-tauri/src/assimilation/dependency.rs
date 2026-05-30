use std::path::Path;

#[derive(Debug, Clone)]
pub struct DependencyInfo {
    pub language: String,
    pub files: Vec<String>,
    pub packages: Vec<String>,
    pub has_lockfile: bool,
}

pub struct DependencyAnalyzer;

impl DependencyAnalyzer {
    pub fn new() -> Self {
        Self
    }

    pub fn analyze(&self, repo_path: &str, language: &str) -> DependencyInfo {
        let path = Path::new(repo_path);
        let mut packages = Vec::new();
        let mut files = Vec::new();
        let mut has_lockfile = false;

        if !path.exists() {
            return DependencyInfo {
                language: language.to_string(),
                files,
                packages,
                has_lockfile,
            };
        }

        match language {
            "Rust" => self.analyze_cargo(repo_path, &mut packages, &mut files, &mut has_lockfile),
            "TypeScript/JavaScript" | "JS/TS" => self.analyze_npm(repo_path, &mut packages, &mut files, &mut has_lockfile),
            "Python" => self.analyze_python(repo_path, &mut packages, &mut files, &mut has_lockfile),
            "Go" => self.analyze_go(repo_path, &mut packages, &mut files, &mut has_lockfile),
            "Ruby" => self.analyze_ruby(repo_path, &mut packages, &mut files, &mut has_lockfile),
            "C#" => self.analyze_csharp(repo_path, &mut packages, &mut files, &mut has_lockfile),
            _ => {}
        }

        DependencyInfo {
            language: language.to_string(),
            files,
            packages,
            has_lockfile,
        }
    }

    fn analyze_cargo(&self, repo_path: &str, packages: &mut Vec<String>, files: &mut Vec<String>, has_lockfile: &mut bool) {
        let cargo_path = Path::new(repo_path).join("Cargo.toml");
        let lock_path = Path::new(repo_path).join("Cargo.lock");
        if lock_path.exists() { *has_lockfile = true; }

        if let Ok(content) = std::fs::read_to_string(&cargo_path) {
            files.push("Cargo.toml".into());
            let mut in_deps = false;
            for line in content.lines() {
                let t = line.trim();
                if t.starts_with("[dependencies]") { in_deps = true; continue; }
                if t.starts_with('[') && t.ends_with(']') && in_deps { in_deps = false; continue; }
                if in_deps {
                    if let Some(eq) = t.find('=') {
                        let name = t[..eq].trim();
                        if !name.is_empty() && !name.starts_with('#') {
                            packages.push(name.to_string());
                        }
                    }
                }
            }
        }
    }

    fn analyze_npm(&self, repo_path: &str, packages: &mut Vec<String>, files: &mut Vec<String>, has_lockfile: &mut bool) {
        let pkg_path = Path::new(repo_path).join("package.json");
        let lock_path = Path::new(repo_path).join("package-lock.json");
        let yarn_lock = Path::new(repo_path).join("yarn.lock");
        if lock_path.exists() || yarn_lock.exists() { *has_lockfile = true; }

        if let Ok(content) = std::fs::read_to_string(&pkg_path) {
            files.push("package.json".into());
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                if let Some(deps) = json["dependencies"].as_object() {
                    for (name, _) in deps {
                        packages.push(name.clone());
                    }
                }
                if let Some(deps) = json["devDependencies"].as_object() {
                    for (name, _) in deps {
                        packages.push(format!("dev:{}", name));
                    }
                }
            }
        }
    }

    fn analyze_python(&self, repo_path: &str, packages: &mut Vec<String>, files: &mut Vec<String>, _has_lockfile: &mut bool) {
        let req_path = Path::new(repo_path).join("requirements.txt");
        if req_path.exists() {
            files.push("requirements.txt".into());
            if let Ok(content) = std::fs::read_to_string(&req_path) {
                for line in content.lines() {
                    let t = line.trim();
                    if !t.is_empty() && !t.starts_with('#') {
                        let pkg = t.split(&['=', '>', '<', '~', '!', '@', ';', '#'][..]).next()
                            .unwrap_or(t).trim().to_string();
                        if !pkg.is_empty() { packages.push(pkg); }
                    }
                }
            }
        }

        let pyproject = Path::new(repo_path).join("pyproject.toml");
        if pyproject.exists() {
            files.push("pyproject.toml".into());
        }
    }

    fn analyze_go(&self, repo_path: &str, packages: &mut Vec<String>, files: &mut Vec<String>, has_lockfile: &mut bool) {
        let go_mod = Path::new(repo_path).join("go.mod");
        let go_sum = Path::new(repo_path).join("go.sum");
        if go_sum.exists() { *has_lockfile = true; }

        if go_mod.exists() {
            files.push("go.mod".into());
            if let Ok(content) = std::fs::read_to_string(&go_mod) {
                for line in content.lines() {
                    let t = line.trim();
                    if t.starts_with("require") || t.starts_with('\t') {
                        let pkg = t.split_whitespace().next().unwrap_or("").to_string();
                        if !pkg.is_empty() && pkg != "require" && pkg != "(" && pkg != ")" {
                            packages.push(pkg);
                        }
                    }
                }
            }
        }
    }

    fn analyze_ruby(&self, repo_path: &str, packages: &mut Vec<String>, files: &mut Vec<String>, has_lockfile: &mut bool) {
        let gem_path = Path::new(repo_path).join("Gemfile");
        let lock_path = Path::new(repo_path).join("Gemfile.lock");
        if lock_path.exists() { *has_lockfile = true; }

        if gem_path.exists() {
            files.push("Gemfile".into());
            if let Ok(content) = std::fs::read_to_string(&gem_path) {
                for line in content.lines() {
                    let t = line.trim();
                    if t.starts_with("gem ") {
                        let gem = t.splitn(3, '\'')
                            .nth(1).or_else(|| t.splitn(3, '"').nth(1))
                            .unwrap_or("").to_string();
                        if !gem.is_empty() { packages.push(gem); }
                    }
                }
            }
        }
    }

    fn analyze_csharp(&self, repo_path: &str, packages: &mut Vec<String>, files: &mut Vec<String>, _has_lockfile: &mut bool) {
        let csproj_files = std::fs::read_dir(repo_path).ok()
            .map(|entries| entries.flatten()
                .filter(|e| e.path().extension().map_or(false, |ext| ext == "csproj"))
                .collect::<Vec<_>>())
            .unwrap_or_default();

        for entry in &csproj_files {
            let name = entry.file_name().to_string_lossy().to_string();
            files.push(name);

            if let Ok(content) = std::fs::read_to_string(entry.path()) {
                for line in content.lines() {
                    let t = line.trim();
                    if t.contains("<PackageReference Include=") {
                        if let Some(start) = t.find("Include=\"") {
                            let rest = &t[start + 9..];
                            if let Some(end) = rest.find('"') {
                                packages.push(rest[..end].to_string());
                            }
                        }
                    }
                }
            }
        }

        let sln_files = std::fs::read_dir(repo_path).ok()
            .map(|entries| entries.flatten()
                .filter(|e| e.path().extension().map_or(false, |ext| ext == "sln"))
                .collect::<Vec<_>>())
            .unwrap_or_default();
        for e in &sln_files {
            files.push(e.file_name().to_string_lossy().to_string());
        }
    }

    pub fn summary(&self, info: &DependencyInfo) -> String {
        let mut lines = Vec::new();
        lines.push(format!("  Dil: {}", info.language));
        lines.push(format!("  Build dosyaları: {}", info.files.join(", ")));
        lines.push(format!("  Bağımlılık sayısı: {}", info.packages.len()));
        if !info.packages.is_empty() {
            let max = info.packages.iter().take(10).cloned().collect::<Vec<_>>();
            lines.push(format!("  Paketler (ilk {}): {}", max.len(), max.join(", ")));
            if info.packages.len() > 10 {
                lines.push(format!("    ... ve {} daha", info.packages.len() - 10));
            }
        }
        lines.push(format!("  Lockfile: {}", if info.has_lockfile { "var" } else { "yok" }));
        lines.join("\n")
    }
}
