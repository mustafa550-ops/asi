use std::fs;
use std::path::Path;

pub struct DocFile {
    pub path: String,
    pub content: String,
    pub size: u64,
}

pub fn scan_docs(root: &str) -> Result<Vec<DocFile>, String> {
    let mut docs = Vec::new();
    scan_dir(Path::new(root), &mut docs)?;
    Ok(docs)
}

fn scan_dir(dir: &Path, docs: &mut Vec<DocFile>) -> Result<(), String> {
    if !dir.is_dir() {
        return Ok(());
    }

    for entry in fs::read_dir(dir).map_err(|e| format!("Okuma hatasi {:?}: {}", dir, e))? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();

        if path.is_dir() {
            let name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
            if name.starts_with(".") || name == "node_modules" || name == "target" || name == ".git" {
                continue;
            }
            scan_dir(&path, docs)?;
        } else if let Some(ext) = path.extension() {
            if ext == "md" || ext == "txt" || ext == "rs" || ext == "toml" {
                let size = fs::metadata(&path)
                    .map(|m| m.len())
                    .unwrap_or(0);
                let content = fs::read_to_string(&path)
                    .unwrap_or_else(|_| String::new());
                docs.push(DocFile {
                    path: path.to_string_lossy().to_string(),
                    content,
                    size,
                });
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn scan_docs_finds_md_files() {
        let dir = TempDir::new().unwrap();
        let file_path = dir.path().join("test.md");
        std::fs::write(&file_path, "# Hello").unwrap();

        let docs = scan_docs(dir.path().to_str().unwrap()).unwrap();
        assert_eq!(docs.len(), 1);
        assert!(docs[0].path.ends_with("test.md"));
        assert_eq!(docs[0].content, "# Hello");
    }

    #[test]
    fn scan_docs_skips_node_modules() {
        let dir = TempDir::new().unwrap();
        let nm = dir.path().join("node_modules");
        std::fs::create_dir(&nm).unwrap();
        std::fs::write(nm.join("ignored.md"), "should not appear").unwrap();

        let docs = scan_docs(dir.path().to_str().unwrap()).unwrap();
        assert!(docs.is_empty());
    }

    #[test]
    fn scan_docs_skips_dotfiles() {
        let dir = TempDir::new().unwrap();
        let hidden = dir.path().join(".hidden");
        std::fs::create_dir(&hidden).unwrap();
        std::fs::write(hidden.join("secret.md"), "hidden").unwrap();

        let docs = scan_docs(dir.path().to_str().unwrap()).unwrap();
        assert!(docs.is_empty());
    }

    #[test]
    fn scan_docs_finds_rs_files() {
        let dir = TempDir::new().unwrap();
        std::fs::write(dir.path().join("main.rs"), "fn main() {}").unwrap();

        let docs = scan_docs(dir.path().to_str().unwrap()).unwrap();
        assert_eq!(docs.len(), 1);
        assert!(docs[0].path.ends_with("main.rs"));
    }

    #[test]
    fn scan_docs_reports_file_sizes() {
        let dir = TempDir::new().unwrap();
        let content = "x".repeat(1024);
        std::fs::write(dir.path().join("large.md"), &content).unwrap();

        let docs = scan_docs(dir.path().to_str().unwrap()).unwrap();
        assert_eq!(docs[0].size, 1024);
    }
}
