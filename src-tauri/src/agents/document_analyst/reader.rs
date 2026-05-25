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
