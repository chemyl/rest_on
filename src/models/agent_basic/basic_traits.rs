use crate::models::agent_basic::basic_agent::AgentState;
use crate::models::general::llm::Message;

/// Defines the basic traits that an agent must implement for managing its state, objective, position, and memory.
pub trait BasicTraits {
    /// Creates a new instance of the implementing type with the given objective and position.
    ///
    /// # Parameters
    /// - `objective`: The main goal or purpose of the agent.
    /// - `position`: The role or title of the agent.
    ///
    /// # Returns
    /// A new instance of the implementing type.
    fn new(objective: String, position: String) -> Self;
    /// Updates the state of the agent.
    ///
    /// # Parameters
    /// - `new_state`: The new state to transition the agent to.
    fn update_state(&mut self, new_state: AgentState);
    /// Retrieves the objective of the agent.
    ///
    /// # Returns
    /// A reference to the agent's objective as a `String`.
    fn get_objective(&self) -> &String;
    /// Retrieves the position or role of the agent.
    ///
    /// # Returns
    /// A reference to the agent's position as a `String`.
    fn get_position(&self) -> &String;
    /// Retrieves the current state of the agent.
    ///
    /// # Returns
    /// A reference to the agent's current state as an `AgentState`.
    fn get_state(&self) -> &AgentState;
    /// Retrieves the memory of the agent, which contains logged messages or information.
    ///
    /// # Returns
    /// A reference to the agent's memory as a `Vec<Message>`.
    fn get_memory(&self) -> &Vec<Message>;
}
