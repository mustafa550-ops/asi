use std::path::Path;

pub fn auto_commit(message: &str) -> Result<(), String> {
    let repo_path = Path::new(".");
    let repo = git2::Repository::open(repo_path)
        .map_err(|e| format!("Git repo acilamadi: {}", e))?;

    let mut index = repo.index()
        .map_err(|e| format!("Git index alinamadi: {}", e))?;

    index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)
        .map_err(|e| format!("Dosyalar eklenemedi: {}", e))?;

    index.write()
        .map_err(|e| format!("Index yazilamadi: {}", e))?;

    let tree_id = index.write_tree()
        .map_err(|e| format!("Tree yazilamadi: {}", e))?;

    let tree = repo.find_tree(tree_id)
        .map_err(|e| format!("Tree bulunamadi: {}", e))?;

    let signature = git2::Signature::now("ADLER ASI", "adler@asi.local")
        .map_err(|e| format!("Imza olusturulamadi: {}", e))?;

    let parent = repo.head()
        .ok()
        .and_then(|h| h.target())
        .and_then(|oid| repo.find_commit(oid).ok());

    let commit_oid = match parent {
        Some(ref parent_commit) => {
            repo.commit(
                Some("HEAD"),
                &signature,
                &signature,
                message,
                &tree,
                &[parent_commit],
            ).map_err(|e| format!("Commit basarisiz: {}", e))?
        }
        None => {
            repo.commit(
                Some("HEAD"),
                &signature,
                &signature,
                message,
                &tree,
                &[],
            ).map_err(|e| format!("Ilk commit basarisiz: {}", e))?
        }
    };

    log::info!("Git commit: {} — {}", commit_oid, message);
    Ok(())
}

pub fn repo_status() -> Result<String, String> {
    let repo = git2::Repository::open(".")
        .map_err(|e| format!("Repo acilamadi: {}", e))?;

    let mut statuses = Vec::new();
    let status = repo.statuses(None)
        .map_err(|e| format!("Status alinamadi: {}", e))?;

    for entry in status.iter() {
        let file = entry.path().unwrap_or("?");
        let flags = entry.status();

        if flags.contains(git2::Status::CURRENT) { continue; }

        let label = if flags.contains(git2::Status::INDEX_NEW) { "YENI" }
            else if flags.contains(git2::Status::INDEX_MODIFIED) { "DEGISTI" }
            else if flags.contains(git2::Status::WT_NEW) { "YENI(WD)" }
            else if flags.contains(git2::Status::WT_MODIFIED) { "DEGISTI(WD)" }
            else { "?" };

        statuses.push(format!("{} {}", label, file));
    }

    if statuses.is_empty() {
        Ok("Temiz calisma dizini".into())
    } else {
        Ok(statuses.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repo_status() {
        let tmp = std::env::temp_dir().join("adler_git_test");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).unwrap();
        let repo = git2::Repository::init(&tmp).unwrap();
        let original = std::env::current_dir().unwrap();
        std::env::set_current_dir(&tmp).unwrap();
        let status = repo_status();
        std::env::set_current_dir(&original).unwrap();
        let _ = std::fs::remove_dir_all(&tmp);
        assert!(status.is_ok());
    }
}
