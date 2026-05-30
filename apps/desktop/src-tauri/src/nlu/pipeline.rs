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

        let intent = self.classify(&resolved);

        let missing_slots = self.slot_filler.detect_missing(&intent, &resolved);
        let follow_up_question = self.slot_filler.generate_question(&missing_slots);

        let confidence = match &intent {
            Intent::Unknown => 0.3,
            Intent::Chat => 0.5,
            _ => 0.85,
        };

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

    fn classify(&mut self, text: &str) -> Intent {
        if let Some(cached) = self.cache.get(text) {
            return cached.clone();
        }

        if let Some((_name, custom_intent)) = self.customs.match_custom(text) {
            return custom_intent.clone();
        }

        let lower = text.to_lowercase();

        if lower.contains("merhaba") || lower.contains("selam") || lower.contains("nasilsin") {
            return Intent::Chat;
        }
        if lower.contains("kontrol") || lower.contains("analiz") || lower.contains("sinyal") {
            if lower.contains("sxt") || lower.contains("btc") || lower.contains("eth")
                || lower.contains("xrp") || lower.contains("ada") || lower.contains("sol")
            {
                return Intent::Crypto;
            }
        }
        if lower.contains("ac") || lower.contains("kapat") || lower.contains("role")
            || lower.contains("gpio") || lower.contains("pin") || lower.contains("lamba")
            || lower.contains("isik") || lower.contains("zil")
        {
            return Intent::Hardware;
        }
        if lower.contains("sistem") || lower.contains("ram") || lower.contains("cpu")
            || lower.contains("bellek") || lower.contains("disk")
        {
            return Intent::System;
        }
        if lower.contains("oku") || lower.contains("tara") || lower.contains("belge")
            || lower.contains("dokuman") || lower.contains("not") || lower.contains("indeksle")
        {
            return Intent::Document;
        }
        if lower.contains("dinle") || lower.contains("konus") || lower.contains("ses")
            || lower.contains("voice") || lower.contains("speak")
        {
            return Intent::Voice;
        }
        if lower.contains("ne") || lower.contains("nedir") || lower.contains("nasil")
            || lower.contains("neden") || lower.contains("what") || lower.contains("how")
            || lower.contains("why")
        {
            return Intent::Query;
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
