/// Code Splitter — Repo kodunu Core/Interface/Config olarak ayırır (§8.1).
pub struct CodeSplitter;

impl CodeSplitter {
    pub fn new() -> Self {
        Self
    }

    pub fn split(&self, _repo_path: &str) -> SplitResult {
        SplitResult {
            core: Vec::new(),
            interface: Vec::new(),
            config: Vec::new(),
        }
    }
}

pub struct SplitResult {
    pub core: Vec<String>,
    pub interface: Vec<String>,
    pub config: Vec<String>,
}
