/// FS API — Dosya/skill yönetimi için Tauri FS köprüsü (§3.2).
pub struct FsApi;

impl FsApi {
    pub fn new() -> Self {
        Self
    }

    pub fn read_skill_manifest(&self, path: &str) -> Result<String, std::io::Error> {
        std::fs::read_to_string(path)
    }

    pub fn write_skill_manifest(&self, path: &str, content: &str) -> Result<(), std::io::Error> {
        std::fs::write(path, content)
    }
}
