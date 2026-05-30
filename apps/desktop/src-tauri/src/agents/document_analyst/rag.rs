use crate::MemoryManager;

pub fn query_with_sources(memory: &MemoryManager, query: &str, limit: usize) -> Result<String, String> {
    let semantic = memory.semantic_search(query, limit)?;
    let keyword = memory.keyword_search(query, limit)?;

    let mut report = String::new();
    let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();

    if !semantic.is_empty() {
        report.push_str("=== Semantik Eslesmeler ===\n");
        for r in &semantic {
            let label = format!("[{} | {:.0}%]", r.category, r.score * 100.0);
            report.push_str(&format!("  {} Kaynak: {}\n  {}\n\n", label, r.source, truncate(&r.content, 300)));
            seen.insert(r.source.clone());
        }
    }

    if !keyword.is_empty() {
        report.push_str("=== Anahtar Kelime Eslesmeleri ===\n");
        for r in &keyword {
            if !seen.contains(&r.source) {
                report.push_str(&format!("  [{}] Kaynak: {}\n  {}\n\n", r.category, r.source, truncate(&r.content, 300)));
            }
        }
    }

    if report.is_empty() {
        report.push_str("Eslesen dokuman bulunamadi.\n");
    }

    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncate_short_string_unchanged() {
        assert_eq!(truncate("hello", 10), "hello");
    }

    #[test]
    fn truncate_long_string() {
        let result = truncate("hello world this is a test", 10);
        assert!(result.ends_with("..."));
        assert_eq!(result.len(), 13);
    }

    #[test]
    fn truncate_exact_length() {
        assert_eq!(truncate("hello", 5), "hello");
    }

    #[test]
    fn truncate_empty_string() {
        assert_eq!(truncate("", 10), "");
    }
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}...", &s[..max])
    }
}
