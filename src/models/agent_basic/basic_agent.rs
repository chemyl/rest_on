use crate::models::agent_basic::basic_traits::BasicTraits;
use crate::models::general::llm::Message;

/// Represents the various states an agent can be in during its workflow.
///
/// # Variants
/// - `Discovery`: The agent is gathering initial information or requirements.
/// - `Working`: The agent is actively performing tasks related to the project.
/// - `UnitTesting`: The agent is performing tests on the results or validating its work.
/// - `Finished`: The agent has completed its tasks.
#[derive(Debug, PartialEq)]
pub enum AgentState {
    Discovery,
    Working,
    UnitTesting,
    Finished,
}

/// Represents a basic agent with attributes like its objective, position, state, and memory.
///
/// # Fields
/// - `objective`: The main goal or purpose of the agent.
/// - `position`: The agent's role or title in the workflow.
/// - `state`: The current state of the agent in its workflow.
/// - `memory`: A log of messages or information the agent retains during its operation.
#[derive(Debug)]
pub struct BasicAgent {
    pub objective: String,
    pub position: String,
    pub state: AgentState,
    pub memory: Vec<Message>,
}

impl BasicTraits for BasicAgent {
    /// Creates a new instance of `BasicAgent` with the given objective and position.
    ///
    /// # Parameters
    /// - `objective`: The goal or purpose of the agent.
    /// - `position`: The role or title of the agent.
    ///
    /// # Returns
    /// A new `BasicAgent` instance with the initial state set to `Discovery` and an empty memory.
    fn new(objective: String, position: String) -> Self {
        Self {
            objective,
            position,
            state: AgentState::Discovery,
            memory: Vec::from([]),
        }
    }
    /// Updates the state of the agent.
    ///
    /// # Parameters
    /// - `new_state`: The new state to transition the agent to.
    fn update_state(&mut self, new_state: AgentState) {
        self.state = new_state;
    }
    /// Retrieves the objective of the agent.
    ///
    /// # Returns
    /// A reference to the agent's objective.
    fn get_objective(&self) -> &String {
        &self.objective
    }
    /// Retrieves the position or role of the agent.
    ///
    /// # Returns
    /// A reference to the agent's position.
    fn get_position(&self) -> &String {
        &self.position
    }
    /// Retrieves the current state of the agent.
    ///
    /// # Returns
    /// A reference to the agent's state.
    fn get_state(&self) -> &AgentState {
        &self.state
    }
    /// Retrieves the memory of the agent.
    ///
    /// # Returns
    /// A reference to the agent's memory.
    fn get_memory(&self) -> &Vec<Message> {
        &self.memory
    }
}
