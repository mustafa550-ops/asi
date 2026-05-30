use crate::nlu::intent::Intent;

#[derive(Debug, Clone, PartialEq)]
pub enum Language {
    Turkish,
    English,
    Unknown,
}

pub struct LanguageDetector;

impl LanguageDetector {
    pub fn detect(text: &str) -> Language {
        let turkish_chars = ['ğ', 'ü', 'ş', 'ı', 'ö', 'ç', 'Ğ', 'Ü', 'Ş', 'İ', 'Ö', 'Ç'];
        let turkish_count = text.chars().filter(|c| turkish_chars.contains(c)).count();
        let turkish_words = [
            "merhaba", "nasıl", "nedir", "ne", "bu", "şu", "ve", "ile", "için",
            "bir", "ama", "veya", "değil", "çok", "daha", "hemen", "acaba",
            "yap", "et", "git", "gel", "bak", "söyle", "oku", "yaz",
        ];
        let word_count = turkish_words.iter()
            .filter(|w| text.to_lowercase().contains(*w))
            .count();

        if turkish_count > 0 || word_count > 1 {
            Language::Turkish
        } else {
            Language::English
        }
    }

    pub fn translate_intent(intent: &Intent, lang: &Language) -> &'static str {
        match lang {
            Language::Turkish => intent.as_str(),
            Language::English => match intent {
                Intent::Query => "query",
                Intent::Action => "action",
                Intent::Analysis => "analysis",
                Intent::Hardware => "hardware",
                Intent::Crypto => "crypto",
                Intent::System => "system",
                Intent::Document => "document",
                Intent::Voice => "voice",
                Intent::Chat => "chat",
                Intent::Unknown => "unknown",
            },
            Language::Unknown => intent.as_str(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nlu::intent::Intent;

    #[test]
    fn test_detect_turkish() {
        assert!(matches!(LanguageDetector::detect("Merhaba, nasılsın?"), Language::Turkish));
        assert!(matches!(LanguageDetector::detect("Röleyi aç"), Language::Turkish));
    }

    #[test]
    fn test_detect_english() {
        assert!(matches!(LanguageDetector::detect("Hello, how are you?"), Language::English));
        assert!(matches!(LanguageDetector::detect("Check SXT price"), Language::English));
    }

    #[test]
    fn test_translate_intent() {
        let intent = Intent::Query;
        assert_eq!(LanguageDetector::translate_intent(&intent, &Language::English), "query");
        assert_eq!(LanguageDetector::translate_intent(&intent, &Language::Turkish), "sorgu");
    }
}
