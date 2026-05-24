use std::fs;

pub struct CodeSplitter;

#[derive(Debug)]
pub struct SplitResult {
    pub core: Vec<String>,
    pub interface: Vec<String>,
    pub config: Vec<String>,
}

impl CodeSplitter {
    pub fn new() -> Self {
        Self
    }

    pub fn split(&self, repo_path: &str) -> SplitResult {
        let mut core = Vec::new();
        let mut interface = Vec::new();
        let mut config = Vec::new();

        if let Ok(entries) = fs::read_dir(repo_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                let name = entry.file_name().to_string_lossy().to_string();

                if path.is_dir() {
                    match name.as_str() {
                        "src" | "lib" | "core" | "backend" | "rust" => {
                            core.push(name);
                        }
                        "gui" | "ui" | "frontend" | "interface" | "web" => {
                            interface.push(name);
                        }
                        "config" | "conf" | "settings" | ".config" => {
                            config.push(name);
                        }
                        _ => {
                            if name.starts_with('.') {
                                config.push(name);
                            } else {
                                core.push(name);
                            }
                        }
                    }
                } else if path.is_file() {
                    match name.as_str() {
                        ".env" | "config.yaml" | "config.toml" | "config.json" | "settings.json" => {
                            config.push(name);
                        }
                        _ => {}
                    }
                }
            }
        }

        SplitResult { core, interface, config }
    }
}
