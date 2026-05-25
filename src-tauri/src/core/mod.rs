pub mod orchestrator;
pub mod memory_manager;
pub mod self_healing;
pub mod wasm_sandbox;
pub mod wasm_compile;
pub mod tool_registry;

use crate::agents::ApprovalLevel;
use crate::bridge::event_bus::EventBus;
use orchestrator::Orchestrator;

pub struct CoreContext {
    pub event_bus: Option<EventBus>,
    pub orchestrator: Orchestrator,
    pub self_healing: self_healing::SelfHealingEngine,
    pub wasm: wasm_sandbox::WasmSandbox,
}

impl CoreContext {
    pub fn new(approval_level: ApprovalLevel) -> Self {
        Self {
            event_bus: None,
            orchestrator: Orchestrator::new(approval_level),
            self_healing: self_healing::SelfHealingEngine::new(),
            wasm: wasm_sandbox::WasmSandbox::new(),
        }
    }
}
