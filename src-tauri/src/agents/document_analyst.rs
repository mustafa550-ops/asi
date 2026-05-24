use super::Agent;

/// Document Analyst — RAG üzerinden .md dosyalarını analiz etme (§4.1).
pub struct DocumentAnalyst;

impl Agent for DocumentAnalyst {
    fn name(&self) -> String { "Document Analyst".into() }
    fn description(&self) -> String { "Doküman analizi ve RAG sorgulama".into() }
    fn can_handle(&self, task: &str) -> bool {
        task.contains("doküman") || task.contains("not") || task.contains("belge")
    }
    fn execute(&self, _task: &str) -> Result<String, String> {
        Ok("Doküman analizi tamamlandı".into())
    }
}
