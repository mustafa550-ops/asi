/// Repo Analyzer — Repo yapısını analiz eder (§8.1).
pub struct RepoAnalyzer;

impl RepoAnalyzer {
    pub fn new() -> Self {
        Self
    }

    pub fn analyze(&self, _repo_path: &str) -> Result<Analysis, String> {
        Ok(Analysis {
            language: "unknown".into(),
            has_readme: false,
            file_count: 0,
        })
    }
}

pub struct Analysis {
    pub language: String,
    pub has_readme: bool,
    pub file_count: u32,
}
