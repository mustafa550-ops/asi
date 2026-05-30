use async_trait::async_trait;
use serde_json::json;
use std::any::Any;

use crate::agent::Agent;
use crate::agent_lifecycle::{AgentAclLevel, AgentState};
use crate::error::Result;
use crate::protocol::AgentMessage;

pub struct IntentJudgeAgent {
    id: String,
    state: AgentState,
}

impl IntentJudgeAgent {
    pub fn new() -> Self {
        Self {
            id: "intent_judge_01".to_string(),
            state: AgentState::Idle,
        }
    }

    /// Mock classification for now. In real system, calls LLM.
    async fn classify_intent(&self, text: &str) -> String {
        let t = text.to_lowercase();
        if t.contains("sxt") || t.contains("fiyat") || t.contains("market") {
            "MarketAnalysis".to_string()
        } else if t.contains("röle") || t.contains("ışık") || t.contains("donanım") {
            "HardwareControl".to_string()
        } else if t.contains("hata") || t.contains("log") {
            "Diagnostic".to_string()
        } else {
            "Chat".to_string()
        }
    }
}

#[async_trait]
impl Agent for IntentJudgeAgent {
    fn id(&self) -> &str {
        &self.id
    }

    fn name(&self) -> &str {
        "Intent Judge"
    }

    fn status(&self) -> AgentState {
        self.state
    }

    fn acl_level(&self) -> AgentAclLevel {
        AgentAclLevel::SemiAutonomous
    }

    async fn execute(&mut self, msg: AgentMessage) -> Result<Option<AgentMessage>> {
        self.state = AgentState::Executing;
        
        // Extract text from payload
        let text = msg.payload.as_str().unwrap_or("");
        
        // Zero-Mock: We will eventually use real LLM. 
        // For now, this is a rule-based fallback representing the real pipeline.
        let intent = self.classify_intent(text).await;
        
        let response_payload = json!({
            "original_text": text,
            "intent": intent,
            "confidence": 0.85
        });

        self.state = AgentState::Idle;
        
        Ok(Some(AgentMessage::new(
            self.id.clone(),
            msg.from,
            response_payload
        )))
    }

    async fn pause(&mut self) -> Result<()> {
        self.state = AgentState::Paused;
        Ok(())
    }

    async fn resume(&mut self) -> Result<()> {
        self.state = AgentState::Idle;
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn new_creates_idle_agent() {
        let agent = IntentJudgeAgent::new();
        assert_eq!(agent.id(), "intent_judge_01");
        assert_eq!(agent.name(), "Intent Judge");
        assert_eq!(agent.status(), AgentState::Idle);
        assert_eq!(agent.acl_level(), AgentAclLevel::SemiAutonomous);
    }

    #[tokio::test]
    async fn classify_market_intent() {
        let agent = IntentJudgeAgent::new();
        assert_eq!(agent.classify_intent("SXT fiyatı nedir?").await, "MarketAnalysis");
        assert_eq!(agent.classify_intent("market analizi yap").await, "MarketAnalysis");
    }

    #[tokio::test]
    async fn classify_hardware_intent() {
        let agent = IntentJudgeAgent::new();
        assert_eq!(agent.classify_intent("röleyi aç").await, "HardwareControl");
        assert_eq!(agent.classify_intent("donanım durumu").await, "HardwareControl");
    }

    #[tokio::test]
    async fn classify_diagnostic_intent() {
        let agent = IntentJudgeAgent::new();
        assert_eq!(agent.classify_intent("hata var").await, "Diagnostic");
        assert_eq!(agent.classify_intent("logları kontrol et").await, "Diagnostic");
    }

    #[tokio::test]
    async fn classify_chat_intent() {
        let agent = IntentJudgeAgent::new();
        assert_eq!(agent.classify_intent("merhaba nasılsın").await, "Chat");
        assert_eq!(agent.classify_intent("ne yapıyorsun").await, "Chat");
    }

    #[tokio::test]
    async fn execute_returns_classification() {
        let mut agent = IntentJudgeAgent::new();
        let msg = AgentMessage::new("user", "intent_judge_01", serde_json::json!("SXT analizi yap"));
        let response = agent.execute(msg).await.unwrap().unwrap();
        assert_eq!(response.from, "intent_judge_01");
        assert_eq!(response.payload["intent"], "MarketAnalysis");
        assert_eq!(response.payload["confidence"], 0.85);
    }

    #[tokio::test]
    async fn execute_changes_state() {
        let mut agent = IntentJudgeAgent::new();
        let msg = AgentMessage::new("user", "intent_judge_01", serde_json::json!("test"));
        assert_eq!(agent.status(), AgentState::Idle);
        let _ = agent.execute(msg).await;
        assert_eq!(agent.status(), AgentState::Idle); // Back to idle after execution
    }

    #[tokio::test]
    async fn pause_resume() {
        let mut agent = IntentJudgeAgent::new();
        agent.pause().await.unwrap();
        assert_eq!(agent.status(), AgentState::Paused);
        agent.resume().await.unwrap();
        assert_eq!(agent.status(), AgentState::Idle);
    }
}
