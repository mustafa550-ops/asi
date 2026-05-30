use async_trait::async_trait;
use std::any::Any;

use crate::error::Result;
use crate::protocol::AgentMessage;
use crate::agent_lifecycle::{AgentAclLevel, AgentState};

#[async_trait]
pub trait Agent: Send + Sync {
    /// Unique identifier for this agent instance
    fn id(&self) -> &str;
    
    /// Human-readable name
    fn name(&self) -> &str;
    
    /// Current state of the agent
    fn status(&self) -> AgentState;
    
    /// Authority level
    fn acl_level(&self) -> AgentAclLevel;
    
    /// Main execution entrypoint for incoming messages
    async fn execute(&mut self, msg: AgentMessage) -> Result<Option<AgentMessage>>;
    
    /// Pause agent operations
    async fn pause(&mut self) -> Result<()>;
    
    /// Resume agent operations
    async fn resume(&mut self) -> Result<()>;
    
    /// Downcast support
    fn as_any(&self) -> &dyn Any;
}
