pub enum StreamEvent {
    Token(String),
    Done,
    Error(String),
}

pub struct StreamParser;

impl StreamParser {
    pub fn parse_line(line: &str) -> Option<StreamEvent> {
        if line.is_empty() || line == "data: [DONE]" {
            return Some(StreamEvent::Done);
        }
        if let Some(data) = line.strip_prefix("data: ") {
            match serde_json::from_str::<serde_json::Value>(data) {
                Ok(json) => {
                    if let Some(response) = json["response"].as_str() {
                        return Some(StreamEvent::Token(response.to_string()));
                    }
                    if json.get("done").and_then(|d| d.as_bool()).unwrap_or(false) {
                        return Some(StreamEvent::Done);
                    }
                    if let Some(error) = json["error"].as_str() {
                        return Some(StreamEvent::Error(error.to_string()));
                    }
                    None
                }
                Err(e) => Some(StreamEvent::Error(e.to_string())),
            }
        } else if line.starts_with("{\"") {
            match serde_json::from_str::<serde_json::Value>(line) {
                Ok(json) => {
                    if let Some(response) = json["response"].as_str() {
                        return Some(StreamEvent::Token(response.to_string()));
                    }
                    None
                }
                Err(_) => None,
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_token() {
        let line = "data: {\"response\":\"Merhaba\"}";
        let event = StreamParser::parse_line(line);
        assert!(matches!(event, Some(StreamEvent::Token(t)) if t == "Merhaba"));
    }

    #[test]
    fn test_parse_done() {
        assert!(matches!(StreamParser::parse_line("data: [DONE]"), Some(StreamEvent::Done)));
        assert!(matches!(StreamParser::parse_line(""), Some(StreamEvent::Done)));
    }

    #[test]
    fn test_parse_done_json() {
        let line = "data: {\"done\":true}";
        let event = StreamParser::parse_line(line);
        assert!(matches!(event, Some(StreamEvent::Done)));
    }

    #[test]
    fn test_parse_error() {
        let line = "data: {\"error\":\"timeout\"}";
        let event = StreamParser::parse_line(line);
        assert!(matches!(event, Some(StreamEvent::Error(e)) if e == "timeout"));
    }

    #[test]
    fn test_parse_raw_json() {
        let line = "{\"response\":\"test\"}";
        let event = StreamParser::parse_line(line);
        assert!(matches!(event, Some(StreamEvent::Token(t)) if t == "test"));
    }
}
