use async_trait::async_trait;
use serde_json::json;
use std::any::Any;
use std::collections::HashMap;

use crate::agent::Agent;
use crate::agent_lifecycle::{AgentAclLevel, AgentState};
use crate::error::Result;
use crate::protocol::AgentMessage;
use crate::agents::intent_judge::IntentJudgeAgent;

pub struct OrchestratorAgent {
    id: String,
    state: AgentState,
    // Using Box<dyn Agent> to store heterogeneous agents
    sub_agents: HashMap<String, Box<dyn Agent>>,
}

impl OrchestratorAgent {
    pub fn new() -> Self {
        let mut sub_agents: HashMap<String, Box<dyn Agent>> = HashMap::new();
        
        let intent_judge = Box::new(IntentJudgeAgent::new());
        sub_agents.insert(intent_judge.id().to_string(), intent_judge);

        Self {
            id: "orchestrator_01".to_string(),
            state: AgentState::Idle,
            sub_agents,
        }
    }

    pub fn register_agent(&mut self, agent: Box<dyn Agent>) {
        self.sub_agents.insert(agent.id().to_string(), agent);
    }
}

#[async_trait]
impl Agent for OrchestratorAgent {
    fn id(&self) -> &str {
        &self.id
    }

    fn name(&self) -> &str {
        "Orchestrator"
    }

    fn status(&self) -> AgentState {
        self.state
    }

    fn acl_level(&self) -> AgentAclLevel {
        AgentAclLevel::FullAuthority
    }

    async fn execute(&mut self, msg: AgentMessage) -> Result<Option<AgentMessage>> {
        self.state = AgentState::Executing;
        
        // Let's assume the orchestrator receives a raw user command.
        // It first asks the intent judge.
        
        let judge_msg = AgentMessage::new(
            self.id.clone(),
            "intent_judge_01",
            msg.payload.clone()
        );

        let judge_response = if let Some(judge) = self.sub_agents.get_mut("intent_judge_01") {
            judge.execute(judge_msg).await?
        } else {
            None
        };

        let response = if let Some(resp) = judge_response {
            // Forward the judge's classification back to the caller for now.
            // In a full implementation, the orchestrator would route to the appropriate agent.
            Some(AgentMessage::new(
                self.id.clone(),
                msg.from,
                json!({
                    "status": "routed",
                    "intent_analysis": resp.payload
                })
            ))
        } else {
            None
        };

        self.state = AgentState::Idle;
        Ok(response)
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
    use crate::agents::intent_judge::IntentJudgeAgent;

    #[tokio::test]
    async fn new_creates_orchestrator_with_intent_judge() {
        let orchestrator = OrchestratorAgent::new();
        assert_eq!(orchestrator.id(), "orchestrator_01");
        assert_eq!(orchestrator.name(), "Orchestrator");
        assert_eq!(orchestrator.status(), AgentState::Idle);
        assert_eq!(orchestrator.acl_level(), AgentAclLevel::FullAuthority);
        assert!(orchestrator.sub_agents.contains_key("intent_judge_01"));
    }

    #[tokio::test]
    async fn register_agent_adds_to_sub_agents() {
        let mut orchestrator = OrchestratorAgent::new();
        let judge = IntentJudgeAgent::new();
        // Can't register intent_judge again (same id), so use a dummy
        struct DummyAgent {
            id: String,
            state: AgentState,
        }
        #[async_trait]
        impl Agent for DummyAgent {
            fn id(&self) -> &str { &self.id }
            fn name(&self) -> &str { "Dummy" }
            fn status(&self) -> AgentState { self.state }
            fn acl_level(&self) -> AgentAclLevel { AgentAclLevel::Observer }
            async fn execute(&mut self, _msg: AgentMessage) -> Result<Option<AgentMessage>> { Ok(None) }
            async fn pause(&mut self) -> Result<()> { self.state = AgentState::Paused; Ok(()) }
            async fn resume(&mut self) -> Result<()> { self.state = AgentState::Idle; Ok(()) }
            fn as_any(&self) -> &dyn Any { self }
        }
        orchestrator.register_agent(Box::new(DummyAgent {
            id: "dummy_01".into(),
            state: AgentState::Created,
        }));
        assert!(orchestrator.sub_agents.contains_key("dummy_01"));
        assert_eq!(orchestrator.sub_agents.len(), 2);
    }

    #[tokio::test]
    async fn execute_routes_through_intent_judge() {
        let mut orchestrator = OrchestratorAgent::new();
        let msg = AgentMessage::new("user", "orchestrator_01", serde_json::json!("SXT fiyatı nedir?"));
        let response = orchestrator.execute(msg).await.unwrap().unwrap();
        assert_eq!(response.from, "orchestrator_01");
        assert_eq!(response.payload["status"], "routed");
        assert_eq!(response.payload["intent_analysis"]["intent"], "MarketAnalysis");
    }

    #[tokio::test]
    async fn pause_resume() {
        let mut orchestrator = OrchestratorAgent::new();
        orchestrator.pause().await.unwrap();
        assert_eq!(orchestrator.status(), AgentState::Paused);
        orchestrator.resume().await.unwrap();
        assert_eq!(orchestrator.status(), AgentState::Idle);
    }
}
