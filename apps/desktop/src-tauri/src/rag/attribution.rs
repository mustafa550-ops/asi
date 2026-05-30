use crate::rag::pipeline::RagResult;

#[derive(Debug, Clone)]
pub struct AttributedResult {
    pub content: String,
    pub source: String,
    pub score: f32,
    pub method: String,
    pub citation: String,
    pub snippet: String,
}

pub struct AttributionEngine;

impl AttributionEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn attribute_results(&self, results: Vec<RagResult>, max_snippet_len: usize) -> Vec<AttributedResult> {
        results.into_iter()
            .map(|r| {
                let citation = self.build_citation(&r.source, &r.method);
                let snippet = self.build_snippet(&r.content, max_snippet_len);
                AttributedResult {
                    content: r.content,
                    source: r.source,
                    score: r.score,
                    method: r.method,
                    citation,
                    snippet,
                }
            })
            .collect()
    }

    pub fn format_with_sources(&self, attributed: &[AttributedResult]) -> String {
        if attributed.is_empty() {
            return "Bilgi bulunamadı.".to_string();
        }

        let mut output = String::new();
        for (i, result) in attributed.iter().enumerate() {
            output.push_str(&format!("[{}] ", i + 1));
            output.push_str(&result.snippet);
            output.push('\n');
            if !result.citation.is_empty() {
                output.push_str(&format!("   → Kaynak: {}", result.citation));
                output.push('\n');
            }
        }
        output
    }

    pub fn format_context(&self, attributed: &[AttributedResult], max_tokens: usize) -> String {
        let mut context = String::new();
        let mut token_count: usize = 0;

        for result in attributed {
            let prefix = format!("[{c}]({s}): ", c = &result.source, s = &result.source);
            let entry = format!("{}{}", prefix, result.snippet);
            let tokens = (entry.len() + 3) / 4;

            if token_count + tokens > max_tokens {
                break;
            }

            context.push_str(&entry);
            context.push('\n');
            token_count += tokens;
        }

        context
    }

    fn build_citation(&self, source: &str, method: &str) -> String {
        let method_label = match method {
            "vector" => "Semantik arama",
            "fts" => "Anahtar kelime araması",
            "hybrid" => "Hibrit arama",
            _ => method,
        };
        format!("{} ({})", source, method_label)
    }

    fn build_snippet(&self, content: &str, max_len: usize) -> String {
        if content.len() <= max_len {
            content.to_string()
        } else {
            let mut snippet = content[..max_len].to_string();
            if let Some(last_space) = snippet.rfind(' ') {
                snippet.truncate(last_space);
            }
            snippet.push_str("...");
            snippet
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rag::pipeline::RagResult;

    fn make_result(content: &str, source: &str, score: f32, method: &str) -> RagResult {
        RagResult {
            content: content.to_string(),
            source: source.to_string(),
            score,
            method: method.to_string(),
        }
    }

    #[test]
    fn attribute_results_preserves_count() {
        let engine = AttributionEngine::new();
        let results = vec![
            make_result("içerik 1", "docs/a.md", 0.9, "vector"),
            make_result("içerik 2", "docs/b.md", 0.7, "fts"),
        ];
        let attributed = engine.attribute_results(results, 200);
        assert_eq!(attributed.len(), 2);
    }

    #[test]
    fn format_with_sources_empty() {
        let engine = AttributionEngine::new();
        let output = engine.format_with_sources(&[]);
        assert_eq!(output, "Bilgi bulunamadı.");
    }

    #[test]
    fn format_with_sources_shows_citations() {
        let engine = AttributionEngine::new();
        let results = vec![make_result("test içeriği", "docs/test.md", 0.8, "vector")];
        let attributed = engine.attribute_results(results, 200);
        let output = engine.format_with_sources(&attributed);
        assert!(output.contains("docs/test.md"));
        assert!(output.contains("Semantik arama"));
    }

    #[test]
    fn snippet_truncation() {
        let engine = AttributionEngine::new();
        let long = "a".repeat(300);
        let snippet = engine.build_snippet(&long, 100);
        assert!(snippet.len() <= 104);
        assert!(snippet.ends_with("..."));
    }

    #[test]
    fn snippet_no_truncation_when_short() {
        let engine = AttributionEngine::new();
        let short = "kısa metin";
        let snippet = engine.build_snippet(short, 100);
        assert_eq!(snippet, short);
    }

    #[test]
    fn format_context_respects_max_tokens() {
        let engine = AttributionEngine::new();
        let results = vec![
            make_result("kısa", "a.md", 0.9, "vector"),
            make_result("orta uzunlukta bir metin", "b.md", 0.8, "fts"),
        ];
        let attributed = engine.attribute_results(results, 200);
        // Very small max tokens should still produce at least one entry
        let context = engine.format_context(&attributed, 10);
        assert!(!context.is_empty());
    }

    #[test]
    fn build_citation_includes_method() {
        let engine = AttributionEngine::new();
        let citation = engine.build_citation("docs/report.md", "vector");
        assert!(citation.contains("docs/report.md"));
        assert!(citation.contains("Semantik"));
    }
}
