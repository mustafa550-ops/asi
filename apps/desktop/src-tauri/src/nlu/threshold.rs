use crate::nlu::intent::{Intent, IntentResult};

pub const CONFIDENCE_THRESHOLD: f32 = 0.6;

pub struct ConfidenceGate;

impl ConfidenceGate {
    pub fn apply(result: IntentResult) -> IntentResult {
        if result.confidence < CONFIDENCE_THRESHOLD {
            IntentResult {
                intent: Intent::Chat,
                confidence: result.confidence,
                raw_output: result.raw_output,
            }
        } else {
            result
        }
    }
}
