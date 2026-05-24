pub mod orchestrator;
pub mod memory_manager;
pub mod self_healing;
pub mod wasm_sandbox;
pub mod tool_registry;

use crate::bridge::event_bus::EventBus;

pub struct CoreContext {
    pub event_bus: Option<EventBus>,
    pub orchestrator: orchestrator::Orchestrator,
    pub memory: memory_manager::MemoryManager,
    pub self_healing: self_healing::SelfHealingEngine,
    pub wasm: wasm_sandbox::WasmSandbox,
    pub tools: tool_registry::ToolRegistry,
}
