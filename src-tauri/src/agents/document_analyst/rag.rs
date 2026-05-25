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

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}...", &s[..max])
    }
}
