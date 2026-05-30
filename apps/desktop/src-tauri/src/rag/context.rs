use super::attribution::AttributedResult;
use super::chunker::Chunk;

pub struct ContextBuilder {
    max_tokens: usize,
    system_prompt: String,
}

impl Default for ContextBuilder {
    fn default() -> Self {
        Self {
            max_tokens: 4096,
            system_prompt: "Sen ADLER ASI'sin. Kullanıcıya verilen bağlam bilgilerine dayanarak yanıt ver.".into(),
        }
    }
}

impl ContextBuilder {
    pub fn new(max_tokens: usize, system_prompt: String) -> Self {
        Self { max_tokens, system_prompt }
    }

    pub fn build_context(&self, results: &[AttributedResult], user_query: &str) -> ContextOutput {
        let mut body = String::new();
        let mut used_tokens: usize = (self.system_prompt.len() + user_query.len() + 50) / 4;
        let mut source_count = 0;

        for r in results {
            let entry = format!("- [{c}]({s}): {t}\n", c = &r.source, s = &r.source, t = &r.snippet);
            let entry_tokens = (entry.len() + 3) / 4;

            if used_tokens + entry_tokens > self.max_tokens {
                break;
            }

            body.push_str(&entry);
            used_tokens += entry_tokens;
            source_count += 1;
        }

        ContextOutput {
            system_prompt: self.system_prompt.clone(),
            body,
            user_query: user_query.to_string(),
            total_sources: results.len(),
            used_sources: source_count,
            estimated_tokens: used_tokens,
        }
    }

    pub fn build_rag_prompt(&self, results: &[AttributedResult], user_query: &str) -> String {
        let ctx = self.build_context(results, user_query);
        format!(
            "{}\n\n## Bağlam Bilgisi\n{}\n\n## Kullanıcı Sorusu\n{}\n\n## Yanıt",
            ctx.system_prompt, ctx.body, ctx.user_query
        )
    }

    pub fn build_chunk_context(&self, chunks: &[Chunk], user_query: &str) -> String {
        let mut body = String::new();
        let mut used_tokens: usize = (self.system_prompt.len() + user_query.len() + 50) / 4;

        for chunk in chunks {
            let header = chunk.heading.as_deref().unwrap_or("(başlıksız)");
            let entry = format!("- [{h}]({s}): {c}\n", h = header, s = &chunk.source, c = &chunk.content);
            let entry_tokens = (entry.len() + 3) / 4;

            if used_tokens + entry_tokens > self.max_tokens {
                break;
            }

            body.push_str(&entry);
            used_tokens += entry_tokens;
        }

        format!(
            "{}\n\n## Belge Parçaları\n{}\n\n## Kullanıcı Sorusu\n{}\n\n## Yanıt",
            self.system_prompt, body, user_query
        )
    }
}

pub struct ContextOutput {
    pub system_prompt: String,
    pub body: String,
    pub user_query: String,
    pub total_sources: usize,
    pub used_sources: usize,
    pub estimated_tokens: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rag::attribution::AttributedResult;

    fn make_attributed(content: &str, source: &str) -> AttributedResult {
        AttributedResult {
            content: content.to_string(),
            source: source.to_string(),
            score: 0.9,
            method: "vector".into(),
            citation: format!("{} (Semantik)", source),
            snippet: content.to_string(),
        }
    }

    #[test]
    fn build_context_includes_results() {
        let builder = ContextBuilder::default();
        let results = vec![make_attributed("test içeriği", "docs/test.md")];
        let ctx = builder.build_context(&results, "soru");
        assert!(ctx.body.contains("test içeriği"));
        assert_eq!(ctx.used_sources, 1);
        assert_eq!(ctx.total_sources, 1);
    }

    #[test]
    fn build_rag_prompt_includes_all_sections() {
        let builder = ContextBuilder::default();
        let results = vec![make_attributed("içerik", "kaynak")];
        let prompt = builder.build_rag_prompt(&results, "soru");
        assert!(prompt.contains("Bağlam Bilgisi"));
        assert!(prompt.contains("Kullanıcı Sorusu"));
        assert!(prompt.contains("Yanıt"));
        assert!(prompt.contains("içerik"));
    }

    #[test]
    fn build_chunk_context_includes_chunks() {
        let builder = ContextBuilder::default();
        let chunks = vec![
            Chunk {
                content: "paragraf 1".into(),
                source: "doc.md".into(),
                heading: Some("Başlık".into()),
                chunk_type: crate::rag::chunker::ChunkType::Paragraph,
                token_estimate: 3,
            },
        ];
        let prompt = builder.build_chunk_context(&chunks, "soru");
        assert!(prompt.contains("Başlık"));
        assert!(prompt.contains("paragraf 1"));
        assert!(prompt.contains("Belge Parçaları"));
    }

    #[test]
    fn empty_results_still_produce_basic_prompt() {
        let builder = ContextBuilder::default();
        let prompt = builder.build_rag_prompt(&[], "selam");
        assert!(prompt.contains("Kullanıcı Sorusu"));
        assert!(prompt.contains("selam"));
    }

    #[test]
    fn context_output_tracks_used_vs_total() {
        let builder = ContextBuilder::new(50, "test".into());
        let results = vec![
            make_attributed("kısa", "a.md"),
            make_attributed("orta uzunlukta bir metin", "b.md"),
        ];
        let ctx = builder.build_context(&results, "x");
        assert!(ctx.used_sources <= ctx.total_sources);
    }
}
