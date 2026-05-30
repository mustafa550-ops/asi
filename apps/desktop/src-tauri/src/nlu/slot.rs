use crate::nlu::intent::Intent;

pub struct Slot {
    pub name: String,
    pub value: Option<String>,
    pub required: bool,
}

pub struct SlotFiller;

impl SlotFiller {
    pub fn detect_missing(&self, intent: &Intent, text: &str) -> Vec<Slot> {
        let mut slots = match intent {
            Intent::Crypto => vec![
                Slot { name: "symbol".into(), value: Self::extract_symbol(text), required: true },
                Slot { name: "action".into(), value: Self::extract_action(text), required: false },
            ],
            Intent::Hardware => vec![
                Slot { name: "pin".into(), value: Self::extract_pin(text), required: false },
                Slot { name: "state".into(), value: Self::extract_state(text), required: false },
            ],
            _ => vec![],
        };
        slots.retain(|s| s.value.is_none() && s.required);
        slots
    }

    pub fn generate_question(&self, slots: &[Slot]) -> Option<String> {
        if slots.is_empty() {
            return None;
        }
        let questions: Vec<String> = slots.iter().map(|s| {
            match s.name.as_str() {
                "symbol" => "Hangi coini analiz edeyim?".to_string(),
                "action" => "Ne yapmami istersiniz? (alis/satis)".to_string(),
                "pin" => "Hangi GPIO pinini kullanayim?".to_string(),
                "state" => "Pin durumu ne olsun? (on/off)".to_string(),
                _ => format!("{} bilgisini girin:", s.name),
            }
        }).collect();
        Some(questions.join(" "))
    }

    fn extract_symbol(text: &str) -> Option<String> {
        let known = ["BTC", "ETH", "SXT", "XRP", "ADA", "SOL", "DOGE"];
        let upper = text.to_uppercase();
        known.iter().find(|c| upper.contains(*c)).map(|c| c.to_string())
    }

    fn extract_action(text: &str) -> Option<String> {
        let lower = text.to_lowercase();
        if lower.contains("al") || lower.contains("alis") || lower.contains("buy") {
            Some("buy".into())
        } else if lower.contains("sat") || lower.contains("satis") || lower.contains("sell") {
            Some("sell".into())
        } else {
            None
        }
    }

    fn extract_pin(text: &str) -> Option<String> {
        let re = regex::Regex::new(r"\b(\d+)\b").ok()?;
        re.captures(text)?.get(1).map(|m| m.as_str().to_string())
    }

    fn extract_state(text: &str) -> Option<String> {
        let lower = text.to_lowercase();
        if lower.contains("ac") || lower.contains("on") || lower.contains("yuksek") {
            Some("on".into())
        } else if lower.contains("kapat") || lower.contains("off") || lower.contains("dusuk") {
            Some("off".into())
        } else {
            None
        }
    }
}
