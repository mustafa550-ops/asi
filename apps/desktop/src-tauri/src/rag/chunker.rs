#[derive(Debug, Clone)]
pub struct Chunk {
    pub content: String,
    pub source: String,
    pub heading: Option<String>,
    pub chunk_type: ChunkType,
    pub token_estimate: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ChunkType {
    Paragraph,
    CodeBlock,
    List,
    Heading,
}

pub struct Chunker {
    max_chunk_size: usize,
    overlap: usize,
}

impl Default for Chunker {
    fn default() -> Self {
        Self { max_chunk_size: 512, overlap: 64 }
    }
}

impl Chunker {
    pub fn new(max_chunk_size: usize, overlap: usize) -> Self {
        Self { max_chunk_size, overlap }
    }

    pub fn chunk_markdown(&self, content: &str, source: &str) -> Vec<Chunk> {
        let lines: Vec<&str> = content.lines().collect();
        let mut chunks = Vec::new();
        let mut current_chunk = String::new();
        let mut current_heading: Option<String> = None;
        let mut token_count: usize = 0;

        for line in lines {
            let trimmed = line.trim();
            let line_type = self.classify_line(trimmed);

            if let Some(ChunkType::Heading) = line_type {
                if !current_chunk.is_empty() {
                    chunks.push(self.make_chunk(&current_chunk, source, &current_heading));
                    current_chunk.clear();
                    token_count = 0;
                }
                current_heading = Some(trimmed.trim_start_matches('#').trim().to_string());
                current_chunk.push_str(line);
                current_chunk.push('\n');
                token_count += self.estimate_tokens(trimmed);
                continue;
            }

            let line_tokens = self.estimate_tokens(trimmed);
            if token_count + line_tokens > self.max_chunk_size && !current_chunk.is_empty() {
                chunks.push(self.make_chunk(&current_chunk, source, &current_heading));
                let overlap_text = self.get_overlap(&current_chunk);
                token_count = self.estimate_tokens(&overlap_text);
                current_chunk = overlap_text;
            }

            current_chunk.push_str(line);
            current_chunk.push('\n');
            token_count += line_tokens;
        }

        if !current_chunk.is_empty() {
            chunks.push(self.make_chunk(&current_chunk, source, &current_heading));
        }

        chunks
    }

    fn classify_line(&self, line: &str) -> Option<ChunkType> {
        if line.starts_with("```") {
            return Some(ChunkType::CodeBlock);
        }
        if line.starts_with('#') {
            return Some(ChunkType::Heading);
        }
        if line.starts_with('-') || line.starts_with('*') || line.starts_with(|c: char| c.is_ascii_digit()) {
            return Some(ChunkType::List);
        }
        None
    }

    fn make_chunk(&self, content: &str, source: &str, heading: &Option<String>) -> Chunk {
        Chunk {
            content: content.trim().to_string(),
            source: source.to_string(),
            heading: heading.clone(),
            chunk_type: ChunkType::Paragraph,
            token_estimate: self.estimate_tokens(content),
        }
    }

    fn get_overlap(&self, chunk: &str) -> String {
        let lines: Vec<&str> = chunk.lines().collect();
        let mut overlap_lines = Vec::new();
        let mut token_count: usize = 0;

        for line in lines.iter().rev() {
            let tokens = self.estimate_tokens(line);
            if token_count + tokens > self.overlap {
                break;
            }
            overlap_lines.push(*line);
            token_count += tokens;
        }

        overlap_lines.reverse();
        overlap_lines.join("\n") + "\n"
    }

    pub fn estimate_tokens(&self, text: &str) -> usize {
        (text.len() + 3) / 4
    }

    pub fn chunk_plain_text(&self, content: &str, source: &str, paragraph_separator: &str) -> Vec<Chunk> {
        content.split(paragraph_separator)
            .filter(|p| !p.trim().is_empty())
            .map(|p| Chunk {
                content: p.trim().to_string(),
                source: source.to_string(),
                heading: None,
                chunk_type: ChunkType::Paragraph,
                token_estimate: self.estimate_tokens(p),
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chunks_simple_text() {
        let chunker = Chunker::default();
        let chunks = chunker.chunk_markdown("Merhaba dünya", "test.md");
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0].content, "Merhaba dünya");
    }

    #[test]
    fn chunks_split_at_headings() {
        let chunker = Chunker::new(1000, 10);
        let md = "# Baslik 1\nParagraf 1\n\n## Baslik 2\nParagraf 2\n";
        let chunks = chunker.chunk_markdown(md, "test.md");
        assert_eq!(chunks.len(), 2);
        assert_eq!(chunks[0].heading, Some("Baslik 1".into()));
        assert_eq!(chunks[1].heading, Some("Baslik 2".into()));
    }

    #[test]
    fn chunks_by_size_limit() {
        let chunker = Chunker::new(20, 5);
        let md = "abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuv\nwxyzabc\ndefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwx\n";
        let chunks = chunker.chunk_markdown(md, "test.md");
        assert!(chunks.len() >= 2);
    }

    #[test]
    fn estimate_tokens() {
        let chunker = Chunker::default();
        assert!(chunker.estimate_tokens("hello world") > 0);
        assert_eq!(chunker.estimate_tokens(""), 0);
    }

    #[test]
    fn chunk_plain_text_splits_by_separator() {
        let chunker = Chunker::default();
        let text = "Paragraf 1\n\nParagraf 2\n\nParagraf 3";
        let chunks = chunker.chunk_plain_text(text, "test.md", "\n\n");
        assert_eq!(chunks.len(), 3);
    }

    #[test]
    fn empty_content_yields_no_chunks() {
        let chunker = Chunker::default();
        let chunks = chunker.chunk_markdown("", "test.md");
        assert!(chunks.is_empty());
    }

    #[test]
    fn preserves_source_in_chunks() {
        let chunker = Chunker::default();
        let chunks = chunker.chunk_markdown("icerik", "docs/rehber.md");
        assert_eq!(chunks[0].source, "docs/rehber.md");
    }
}
