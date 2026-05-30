use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Command Router — Frontend'den gelen komutları ilgili ajanlara yönlendirir (§3.2).
#[derive(Default)]
pub struct CommandRouter {
    routes: HashMap<String, RouteTarget>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command {
    pub id: String,
    pub action: String,
    pub params: serde_json::Value,
}

#[derive(Debug, Clone)]
pub enum RouteTarget {
    Orchestrator,
    IntentJudge,
    Diagnostic,
    Hardware,
    MarketAnalyst,
    SystemManager,
    DocumentAnalyst,
    VoiceHandler,
}

impl CommandRouter {
    pub fn new() -> Self {
        let mut routes = HashMap::new();
        routes.insert("orchestrate".into(), RouteTarget::Orchestrator);
        routes.insert("intent".into(), RouteTarget::IntentJudge);
        routes.insert("diagnose".into(), RouteTarget::Diagnostic);
        routes.insert("hardware".into(), RouteTarget::Hardware);
        routes.insert("market".into(), RouteTarget::MarketAnalyst);
        routes.insert("system".into(), RouteTarget::SystemManager);
        routes.insert("document".into(), RouteTarget::DocumentAnalyst);
        routes.insert("voice".into(), RouteTarget::VoiceHandler);
        Self { routes }
    }

    pub fn route(&self, command: &Command) -> Option<&RouteTarget> {
        self.routes.get(&command.action)
    }
}
