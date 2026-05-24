use crate::agents::Agent;

/// Agent Orchestrator — Görev dağıtımı, workflow kontrolü, onay yönetimi (§4.1).
pub struct Orchestrator {
    agents: Vec<Box<dyn Agent + Send>>,
}

impl Orchestrator {
    pub fn new() -> Self {
        Self { agents: Vec::new() }
    }

    pub fn register_agent(&mut self, agent: Box<dyn Agent + Send>) {
        self.agents.push(agent);
    }

    pub fn dispatch(&self, task: &str) -> Vec<String> {
        self.agents.iter().filter_map(|agent| {
            if agent.can_handle(task) {
                Some(agent.name())
            } else {
                None
            }
        }).collect()
    }
}
