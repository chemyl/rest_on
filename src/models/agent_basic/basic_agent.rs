use crate::models::agent_basic::basic_traits::BasicTrait;
use crate::models::general::llm::Message;
// Базовые состояния агента
#[derive(PartialEq, Debug)]
pub enum AgentState {
    Discovery,
    Working,
    UnitTesting,
    Finished,
}

#[derive(Debug)]
pub struct BasicAgent {
    pub objective: String,
    pub position: String,
    pub state: AgentState,
    pub memory: Vec<Message>,
}

impl BasicTrait for BasicAgent {
    fn new(objective: String, position: String) -> Self {
        Self {
            objective,
            position,
            state: AgentState::Discovery,
            memory: Vec::new(),
        }
    }
    fn update_state(&mut self, new_state: AgentState) {
        self.state = new_state;
    }
    fn get_objective(&self) -> &String {
        &self.objective
    }
    fn get_position(&self) -> &String {
        &self.position
    }
    fn get_state(&self) -> &AgentState {
        &self.state
    }
    fn get_memory(&mut self) -> &Vec<Message> {
        &self.memory
    }
}
