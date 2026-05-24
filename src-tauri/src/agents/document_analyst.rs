use super::{Agent, AgentContext};

pub struct DocumentAnalyst;

impl Agent for DocumentAnalyst {
    fn name(&self) -> String { "Document Analyst".into() }
    fn description(&self) -> String { "Doküman analizi ve RAG sorgulama".into() }
    fn can_handle(&self, task: &str) -> bool {
        task.contains("doküman") || task.contains("not") || task.contains("belge") || task.contains("doc")
    }
    fn execute(&self, task: &str, ctx: &AgentContext) -> Result<String, String> {
        if let Some(memory) = ctx.memory {
            let query = task.trim();
            let results = memory.hybrid_search(query, 3)?;
            if results.is_empty() {
                return Ok("[Document Analyst] Bellekte eşleşen doküman bulunamadı.\n\
                           Yeni bir .md dosyası yüklemek için dosyayı chat'e ekleyebilirsin.".into());
            }
            let mut report = format!("[Document Analyst] '{}' için {} sonuç bulundu:\n\n", query, results.len());
            for (i, r) in results.iter().enumerate() {
                report.push_str(&format!("{}. [{} | {:.0}%]\n   {}\n", i + 1, r.category, r.score * 100.0, r.content));
            }
            Ok(report)
        } else {
            Ok("[Document Analyst] RAG sistemi bağlı değil. Bellek yöneticisi kullanılamıyor.".into())
        }
    }
}
