use crate::nlu::analytics::IntentAnalytics;
use crate::nlu::cache::IntentCache;
use crate::nlu::context::ContextResolver;
use crate::nlu::custom::CustomIntentRegistry;
use crate::nlu::i18n::{Language, LanguageDetector};
use crate::nlu::intent::{Intent, IntentResult};
use crate::nlu::ner::NERExtractor;
use crate::nlu::prompts::PromptTemplates;
use crate::nlu::repair::IntentRepair;
use crate::nlu::sentiment::SentimentAnalyzer;
use crate::nlu::slot::SlotFiller;
use crate::nlu::threshold::ConfidenceGate;
use crate::llm::OllamaClient;

pub struct NLUPipeline {
    pub analytics: IntentAnalytics,
    pub cache: IntentCache,
    pub context: ContextResolver,
    pub customs: CustomIntentRegistry,
    pub slot_filler: SlotFiller,
    pub repair: IntentRepair,
    llm: OllamaClient,
}

pub struct NLUResult {
    pub intent: Intent,
    pub confidence: f32,
    pub sentiment: String,
    pub language: Language,
    pub entities: Vec<crate::nlu::ner::Entity>,
    pub resolved_text: String,
    pub missing_slots: Vec<crate::nlu::slot::Slot>,
    pub follow_up_question: Option<String>,
}

impl NLUPipeline {
    pub fn new(llm: OllamaClient) -> Self {
        Self {
            analytics: IntentAnalytics::new(),
            cache: IntentCache::new(100),
            context: ContextResolver::new(),
            customs: CustomIntentRegistry::new(),
            slot_filler: SlotFiller,
            repair: IntentRepair,
            llm,
        }
    }

    pub fn process(&mut self, text: &str) -> NLUResult {
        let resolved = self.context.resolve_anaphora(text);
        self.context.push(&resolved);

        let language = LanguageDetector::detect(&resolved);
        let sentiment = SentimentAnalyzer::analyze(&resolved);
        let entities = NERExtractor::extract(&resolved);

        let rule_match_details = self.rule_match(&resolved);
        let intent = match &rule_match_details {
            Some((intent, _)) => intent.clone(),
            None => self.classify_llm(&resolved),
        };

        let missing_slots = self.slot_filler.detect_missing(&intent, &resolved);
        let follow_up_question = self.slot_filler.generate_question(&missing_slots);

        let confidence = self.compute_confidence(&intent, rule_match_details.as_ref(), &resolved);

        NLUResult {
            intent,
            confidence,
            sentiment: format!("{:?}", sentiment),
            language,
            entities,
            resolved_text: resolved,
            missing_slots,
            follow_up_question,
        }
    }

    pub fn compute_confidence(&self, intent: &Intent, matched: Option<&(Intent, u32)>, text: &str) -> f32 {
        match intent {
            Intent::Unknown => 0.3,
            Intent::Chat => 0.5,
            _ => {
                if let Some((_, keyword_count)) = matched {
                    if *keyword_count >= 3 {
                        0.95
                    } else if *keyword_count == 2 {
                        0.90
                    } else {
                        0.80
                    }
                } else {
                    let word_count = text.split_whitespace().count();
                    if word_count >= 5 {
                        0.85
                    } else if word_count >= 3 {
                        0.75
                    } else {
                        0.65
                    }
                }
            }
        }
    }

    pub fn rule_match(&self, text: &str) -> Option<(Intent, u32)> {
        let lower = text.to_lowercase();
        let mut candidates: Vec<(Intent, u32)> = Vec::new();

        let crypto_symbols = ["sxt", "btc", "eth", "xrp", "ada", "sol", "doge", "bnb", "dot", "link"];
        let crypto_count = crypto_symbols.iter().filter(|&&s| lower.contains(s)).count() as u32;
        if crypto_count > 0 && (lower.contains("kontrol") || lower.contains("analiz") || lower.contains("sinyal") || lower.contains("fiyat") || lower.contains("al") || lower.contains("sat")) {
            candidates.push((Intent::Crypto, crypto_count + 2));
        }

        let hw_terms = ["ac", "kapat", "role", "gpio", "pin", "lamba", "isik", "zil", "motor", "sensor", "led", "12v"];
        let hw_count = hw_terms.iter().filter(|&&t| lower.contains(t)).count() as u32;
        if hw_count > 0 {
            candidates.push((Intent::Hardware, hw_count));
        }

        let sys_terms = ["sistem", "ram", "cpu", "bellek", "disk", "islemci", "depolama", "durum"];
        let sys_count = sys_terms.iter().filter(|&&t| lower.contains(t)).count() as u32;
        if sys_count > 0 {
            candidates.push((Intent::System, sys_count));
        }

        let doc_terms = ["oku", "tara", "belge", "dokuman", "not", "indeksle", "dosya", "klasor"];
        let doc_count = doc_terms.iter().filter(|&&t| lower.contains(t)).count() as u32;
        if doc_count > 0 {
            candidates.push((Intent::Document, doc_count));
        }

        let voice_terms = ["dinle", "konus", "ses", "voice", "speak", "mikrofon", "kaydet"];
        let voice_count = voice_terms.iter().filter(|&&t| lower.contains(t)).count() as u32;
        if voice_count > 0 {
            candidates.push((Intent::Voice, voice_count));
        }

        let query_terms = ["ne", "nedir", "nasil", "neden", "what", "how", "why", "kim", "nerede", "acikla"];
        let query_count = query_terms.iter().filter(|&&t| lower.contains(t)).count() as u32;
        if query_count >= 2 && !lower.contains("kontrol") && !lower.contains("ac") && !lower.contains("kapat") {
            candidates.push((Intent::Query, query_count));
        }

        candidates.into_iter().max_by_key(|(_, count)| *count)
    }

    fn classify_llm(&mut self, text: &str) -> Intent {
        if let Some(cached) = self.cache.get(text) {
            return cached.clone();
        }

        if let Some((_name, custom_intent)) = self.customs.match_custom(text) {
            return custom_intent.clone();
        }

        let lower = text.to_lowercase();
        if lower.contains("merhaba") || lower.contains("selam") || lower.contains("nasilsin") {
            let intent = Intent::Chat;
            self.cache.set(text, intent.clone());
            return intent;
        }
        if lower.contains("ne") || lower.contains("nedir") || lower.contains("nasil")
            || lower.contains("neden") || lower.contains("what") || lower.contains("how")
            || lower.contains("why")
        {
            let intent = Intent::Query;
            self.cache.set(text, intent.clone());
            return intent;
        }

        let prompt = PromptTemplates::intent_classification().replace("{}", text);
        match self.llm.generate_sync(&prompt) {
            Ok(raw) => {
                let intent = Intent::from_str(&raw);
                let result = ConfidenceGate::apply(IntentResult {
                    confidence: if intent == Intent::Unknown { 0.3 } else { 0.85 },
                    intent: intent.clone(),
                    raw_output: raw,
                });
                let final_intent = result.intent.clone();
                self.cache.set(text, final_intent.clone());
                final_intent
            }
            Err(_) => Intent::Chat,
        }
    }

}
