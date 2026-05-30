use std::collections::HashMap;

pub struct CrossReferenceResolver;

impl CrossReferenceResolver {
    pub fn new() -> Self {
        Self
    }

    pub fn resolve_links(&self, content: &str) -> Vec<ResolvedLink> {
        let mut links = Vec::new();

        for cap in LINK_REGEX.captures_iter(content) {
            if let Some(target) = cap.get(1) {
                let display = cap.get(2).map_or(target.as_str(), |m| m.as_str());
                links.push(ResolvedLink {
                    title: display.to_string(),
                    target: target.as_str().to_string(),
                    link_type: LinkType::Wiki,
                    position: target.start(),
                });
            }
        }

        for cap in MD_LINK_REGEX.captures_iter(content) {
            if let (Some(text), Some(url)) = (cap.get(1), cap.get(2)) {
                if !url.as_str().starts_with("http") {
                    links.push(ResolvedLink {
                        title: text.as_str().to_string(),
                        target: url.as_str().to_string(),
                        link_type: LinkType::Markdown,
                        position: text.start(),
                    });
                }
            }
        }

        links
    }

    pub fn build_backlink_index(&self, documents: &[(String, String)]) -> HashMap<String, Vec<Backlink>> {
        let mut index: HashMap<String, Vec<Backlink>> = HashMap::new();

        for (source, content) in documents {
            let links = self.resolve_links(content);
            for link in &links {
                index.entry(link.target.clone())
                    .or_default()
                    .push(Backlink {
                        source: source.clone(),
                        title: link.title.clone(),
                    });
            }
        }

        index
    }
}

use regex::Regex;
use std::sync::LazyLock;

static LINK_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\[\[([^\]|]+)(?:\|([^\]|]+))?\]\]").unwrap()
});

static MD_LINK_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\[([^\]]+)\]\(([^)]+)\)").unwrap()
});

#[derive(Debug, Clone)]
pub struct ResolvedLink {
    pub title: String,
    pub target: String,
    pub link_type: LinkType,
    pub position: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LinkType {
    Wiki,
    Markdown,
}

#[derive(Debug, Clone)]
pub struct Backlink {
    pub source: String,
    pub title: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_wiki_links() {
        let resolver = CrossReferenceResolver::new();
        let content = "Bilgi için [[dokuman]] sayfasına bak.";
        let links = resolver.resolve_links(content);
        assert_eq!(links.len(), 1);
        assert_eq!(links[0].target, "dokuman");
        assert_eq!(links[0].title, "dokuman");
        assert_eq!(links[0].link_type, LinkType::Wiki);
    }

    #[test]
    fn resolves_wiki_links_with_alias() {
        let resolver = CrossReferenceResolver::new();
        let content = "[[gercek-adi|Görünen Ad]]";
        let links = resolver.resolve_links(content);
        assert_eq!(links.len(), 1);
        assert_eq!(links[0].target, "gercek-adi");
        assert_eq!(links[0].title, "Görünen Ad");
    }

    #[test]
    fn resolves_markdown_links() {
        let resolver = CrossReferenceResolver::new();
        let content = "Bilgi için [dokuman](docs/rehber.md) sayfasına bak.";
        let links = resolver.resolve_links(content);
        assert_eq!(links.len(), 1);
        assert_eq!(links[0].target, "docs/rehber.md");
    }

    #[test]
    fn ignores_external_urls() {
        let resolver = CrossReferenceResolver::new();
        let content = "[Google](https://google.com)";
        let links = resolver.resolve_links(content);
        assert!(links.is_empty());
    }

    #[test]
    fn resolves_multiple_links() {
        let resolver = CrossReferenceResolver::new();
        let content = "[[a]] ve [[b]] ve [c](c.md)";
        let links = resolver.resolve_links(content);
        assert_eq!(links.len(), 3);
    }

    #[test]
    fn empty_content_returns_empty() {
        let resolver = CrossReferenceResolver::new();
        let links = resolver.resolve_links("");
        assert!(links.is_empty());
    }

    #[test]
    fn build_backlink_index() {
        let resolver = CrossReferenceResolver::new();
        let docs = vec![
            ("a.md".into(), "[[hedef]]".into()),
            ("b.md".into(), "[[hedef]] ve [[diger]]".into()),
        ];
        let index = resolver.build_backlink_index(&docs);
        assert!(index.contains_key("hedef"));
        assert_eq!(index["hedef"].len(), 2);
        assert!(index.contains_key("diger"));
        assert_eq!(index["diger"].len(), 1);
    }
}
