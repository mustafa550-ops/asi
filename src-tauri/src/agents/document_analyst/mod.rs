pub mod reader;
pub mod rag;

use super::{Agent, AgentContext};

pub struct DocumentAnalyst;

impl Agent for DocumentAnalyst {
    fn name(&self) -> String {
        "Document Analyst".into()
    }

    fn description(&self) -> String {
        "Dokuman analizi, .md indeksleme ve RAG sorgulama".into()
    }

    fn can_handle(&self, task: &str) -> bool {
        let task = task.to_lowercase();
        task.contains("dokuman") || task.contains("not") || task.contains("belge") || task.contains("doc") || task.contains("indeksle") || task.contains("sorgula") || task.contains("tara")
    }

    fn execute(&self, task: &str, ctx: &AgentContext) -> Result<String, String> {
        let action = task.to_lowercase();

        if action.contains("tara") || action.contains("index") || action.contains("indeksle") {
            let docs = reader::scan_docs(".")?;
            let mut report = format!("[Document Analyst] {} dosya bulundu:\n\n", docs.len());
            for d in &docs {
                report.push_str(&format!("- {} ({} byte)\n", d.path, d.size));
            }

            if !docs.is_empty() && action.contains("kaydet") {
                for d in &docs {
                    if let Some(mem) = ctx.memory {
                        mem.index_content(&d.content, &d.path, "doc")?;
                    }
                }
                report.push_str(&format!("\n{} dokuman bellege indekslendi.", docs.len()));
            }
            Ok(report)
        } else if action.contains("ara") || action.contains("sorgula") || action.contains("search") || action.contains("query") {
            let query = task
                .splitn(2, |c: char| c == ':' || c == ' ')
                .nth(1)
                .unwrap_or(task)
                .trim();
            if let Some(mem) = ctx.memory {
                let results = rag::query_with_sources(mem, query, 5)?;
                Ok(format!("[Document Analyst] Sorgu: '{}'\n\n{}", query, results))
            } else {
                Ok("[Document Analyst] RAG sistemi bagli degil. Bellek yoneticisi kullanilamiyor.".into())
            }
        } else {
            Ok("[Document Analyst] Kullanilabilir komutlar:\n  'dokuman tara' — proje dokumanlarini tara\n  'dokuman tara ve kaydet' — tara + indeksle\n  'ara: <sorgu>' — RAG ile semantik arama\n  'sorgula: <sorgu>' — anahtar kelime ile ara".into())
        }
    }
}
