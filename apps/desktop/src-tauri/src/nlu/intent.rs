use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Intent {
    Query,
    Action,
    Analysis,
    Hardware,
    Crypto,
    System,
    Document,
    Voice,
    Chat,
    Unknown,
}

impl Intent {
    pub fn from_str(s: &str) -> Self {
        match s.trim().to_lowercase().as_str() {
            "sorgu" | "query" => Intent::Query,
            "eylem" | "action" | "komut" => Intent::Action,
            "analiz" | "analysis" => Intent::Analysis,
            "donanim" | "hardware" => Intent::Hardware,
            "kripto" | "crypto" | "borsa" => Intent::Crypto,
            "sistem" | "system" => Intent::System,
            "dokuman" | "document" | "belge" => Intent::Document,
            "ses" | "voice" => Intent::Voice,
            _ => Intent::Unknown,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Intent::Query => "sorgu",
            Intent::Action => "eylem",
            Intent::Analysis => "analiz",
            Intent::Hardware => "donanim",
            Intent::Crypto => "kripto",
            Intent::System => "sistem",
            Intent::Document => "dokuman",
            Intent::Voice => "ses",
            Intent::Chat => "chat",
            Intent::Unknown => "unknown",
        }
    }
}

pub struct IntentResult {
    pub intent: Intent,
    pub confidence: f32,
    pub raw_output: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_turkish_intents() {
        assert_eq!(Intent::from_str("sorgu"), Intent::Query);
        assert_eq!(Intent::from_str("eylem"), Intent::Action);
        assert_eq!(Intent::from_str("analiz"), Intent::Analysis);
        assert_eq!(Intent::from_str("donanim"), Intent::Hardware);
        assert_eq!(Intent::from_str("kripto"), Intent::Crypto);
        assert_eq!(Intent::from_str("sistem"), Intent::System);
        assert_eq!(Intent::from_str("dokuman"), Intent::Document);
        assert_eq!(Intent::from_str("ses"), Intent::Voice);
    }

    #[test]
    fn test_parse_english_intents() {
        assert_eq!(Intent::from_str("query"), Intent::Query);
        assert_eq!(Intent::from_str("action"), Intent::Action);
        assert_eq!(Intent::from_str("crypto"), Intent::Crypto);
        assert_eq!(Intent::from_str("voice"), Intent::Voice);
    }

    #[test]
    fn test_parse_unknown() {
        assert_eq!(Intent::from_str("blabla"), Intent::Unknown);
    }

    #[test]
    fn test_as_str_roundtrip() {
        for intent in [Intent::Query, Intent::Action, Intent::Hardware, Intent::Crypto] {
            let s = intent.as_str();
            assert!(!s.is_empty());
            assert_eq!(Intent::from_str(s), intent);
        }
    }
}
