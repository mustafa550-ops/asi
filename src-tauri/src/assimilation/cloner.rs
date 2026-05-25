use std::path::Path;

pub struct RepoCloner;

impl RepoCloner {
    pub fn new() -> Self {
        Self
    }

    pub fn clone(&self, url: &str, target_dir: &str) -> Result<String, String> {
        let path = Path::new(target_dir);
        if path.exists() {
            std::fs::remove_dir_all(path).map_err(|e| format!("Cannot clean target dir: {}", e))?;
        }
        std::fs::create_dir_all(path).map_err(|e| format!("Cannot create target dir: {}", e))?;

        log::info!("Cloning {} into {}", url, target_dir);
        match git2::Repository::clone(url, target_dir) {
            Ok(_) => {
                log::info!("Clone successful: {}", url);
                Ok(format!("Repo cloned to {}", target_dir))
            }
            Err(e) => Err(format!("Git clone failed for '{}': {}", url, e)),
        }
    }

    pub fn clone_with_token(&self, url: &str, target_dir: &str, token: &str) -> Result<String, String> {
        let authenticated_url = url
            .replace("https://", &format!("https://x-access-token:{}@", token))
            .replace("http://", &format!("http://x-access-token:{}@", token));
        self.clone(&authenticated_url, target_dir)
    }
}
