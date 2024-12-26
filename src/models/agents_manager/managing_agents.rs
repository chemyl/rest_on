use crate::ai_functions::ai_func_manager::convert_user_input_to_goal;
use crate::helpers::general::ai_task_request;
use crate::models::agent_basic::basic_agent::{AgentState, BasicAgent};
use crate::models::agents::agent_architecture::AgentSolutionArchitect;
use crate::models::agents::agent_backend::AgentBackendDeveloper;
use crate::models::agents::agent_traits::{FactSheet, SpecialFunctions};

/// Represents a managing agent responsible for overseeing the workflow of other agents and managing the fact sheet.
#[derive(Debug)]
pub struct ManagingAgent {
    /// Attributes of the managing agent, including its objective, position, state, and memory.
    _attributes: BasicAgent,
    /// The fact sheet containing the project description and other related information.
    fact_sheet: FactSheet,
    /// A collection of agents implementing `SpecialFunctions` that perform specific tasks.
    agents: Vec<Box<dyn SpecialFunctions>>,
}

impl ManagingAgent {
    /// Creates a new `ManagingAgent` instance, initializing it with the user's request and project description.
    ///
    /// # Parameters
    /// - `user_request`: A string describing the user's project requirements.
    ///
    /// # Returns
    /// - A `ManagingAgent` instance wrapped in a `Result`.
    pub async fn new(user_request: String) -> Result<Self, Box<dyn std::error::Error>> {
        let agent_position = "Project Manager".to_string();
        let attributes = BasicAgent {
            objective: "Manage agents who build a website".to_string(),
            position: agent_position.clone(),
            state: AgentState::Discovery,
            memory: vec![],
        };

        let project_description: String = ai_task_request(
            user_request,
            &agent_position,
            get_function_string!(convert_user_input_to_goal),
            convert_user_input_to_goal,
        )
        .await;

        let agents: Vec<Box<dyn SpecialFunctions>> = vec![];

        let mut fact_sheet: FactSheet = FactSheet {
            project_description,
            project_scope: None,
            external_urls: None,
            backend_code: None,
            api_endpoint_schema: None,
        };
        Ok(Self {
            _attributes: attributes,
            agents,
            fact_sheet,
        })
    }
    /// Adds a new agent to the `ManagingAgent`.
    ///
    /// # Parameters
    /// - `agent`: A boxed instance of a struct implementing the `SpecialFunctions` trait.
    fn add_agent(&mut self, agent: Box<dyn SpecialFunctions>) {
        self.agents.push(agent);
    }

    /// Creates and initializes agents to handle specific tasks related to the project.
    fn create_agents(&mut self) {
        self.add_agent(Box::new(AgentSolutionArchitect::new()));
        self.add_agent(Box::new(AgentBackendDeveloper::new()));
    }

    /// Executes the project workflow by iterating through all agents and invoking their `execute` methods.
    pub async fn execute_project(&mut self) {
        self.create_agents();
        for agent in &mut self.agents {
            let _: Result<(), Box<dyn std::error::Error>> =
                agent.execute(&mut self.fact_sheet).await;
        }
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;
//
//     #[tokio::test]
//     async fn test_manager_agent() {
//         let user_request: &str = "I need a full stack app that fetch and tracks user fitness progress. Need to include timezone info from the web";
//
//         let mut manager = ManagingAgent::new(user_request.to_string())
//             .await
//             .expect("Error during manager creation");
//         manager.execute_project().await;
//         dbg!(manager.fact_sheet);
//     }
// }
