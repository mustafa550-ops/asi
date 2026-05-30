#[derive(Debug, Clone, PartialEq)]
pub enum Sentiment {
    Urgent,
    Neutral,
    Casual,
}

pub struct SentimentAnalyzer;

impl SentimentAnalyzer {
    pub fn analyze(text: &str) -> Sentiment {
        let lower = text.to_lowercase();

        let urgent_signals = ["hemen", "acil", "çabuk", "derhal", "acilen",
                              "hata var", "calismiyor", "bozuk", "kırık",
                              "yardım et", "durum kritik"];
        for s in urgent_signals {
            if lower.contains(s) {
                return Sentiment::Urgent;
            }
        }

        let casual_signals = ["merhaba", "selam", "nasılsın", "ne haber",
                              "teşekkür", "sağol", "eyvallah", "görüşürüz"];
        for s in casual_signals {
            if lower.contains(s) {
                return Sentiment::Casual;
            }
        }

        Sentiment::Neutral
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_urgent() {
        assert_eq!(SentimentAnalyzer::analyze("Hemen yardım et!"), Sentiment::Urgent);
        assert_eq!(SentimentAnalyzer::analyze("Acil durum var"), Sentiment::Urgent);
        assert_eq!(SentimentAnalyzer::analyze("Sistem calismiyor"), Sentiment::Urgent);
    }

    #[test]
    fn test_casual() {
        assert_eq!(SentimentAnalyzer::analyze("Merhaba, nasılsın?"), Sentiment::Casual);
        assert_eq!(SentimentAnalyzer::analyze("Teşekkür ederim"), Sentiment::Casual);
    }

    #[test]
    fn test_neutral() {
        assert_eq!(SentimentAnalyzer::analyze("SXT fiyatı nedir?"), Sentiment::Neutral);
        assert_eq!(SentimentAnalyzer::analyze("Rolayi kapat"), Sentiment::Neutral);
    }
}
