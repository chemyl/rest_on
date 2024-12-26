use crate::ai_functions::ai_func_manager::convert_user_input_to_goal;
use crate::helpers::general::ai_task_request;
use crate::models::agent_basic::basic_agent::{AgentState, BasicAgent};
use crate::models::agents::agent_architecture::AgentSolutionArchitect;
use crate::models::agents::agent_backend::AgentBackendDeveloper;
use crate::models::agents::agent_traits::{FactSheet, SpecialFunctions};

//ManagingAgent — управляет группой агентов через абстрактный трейт.
//Менеджер может работать с агентами, имеющими разные реализации SpecialFunctions, не зная их внутренностей заранее.
#[derive(Debug)]
pub struct ManagingAgent {
    _attributes: BasicAgent,
    fact_sheet: FactSheet,
    //Управляет группой агентов через вектор агентов (Vec<Box<dyn SpecialFunctions>>), реализующих трейт SpecialFunctions.
    agents: Vec<Box<dyn SpecialFunctions>>,
}

// создать экземпляр ManagingAgent через асинхронную функцию new, которая принимает
// пользовательский запрос и обрабатывает его с помощью AI-функций для генерации описания проекта.
impl ManagingAgent {
    pub async fn new(user_request: String) -> Result<Self, Box<dyn std::error::Error>> {
        let agent_position = "Project Manager".to_string();
        let attributes = BasicAgent {
            objective: "Manage agents who build a website".to_string(),
            position: agent_position.clone(),
            state: AgentState::Discovery,
            memory: vec![],
        };

        //ai_task_request для преобразования пользовательского запроса в описание проекта.
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
    fn add_agent(&mut self, agent: Box<dyn SpecialFunctions>) {
        self.agents.push(agent);
    }
    fn create_agents(&mut self) {
        self.add_agent(Box::new(AgentSolutionArchitect::new()));
        self.add_agent(Box::new(AgentBackendDeveloper::new()));
    }

    pub async fn execute_project(&mut self) {
        self.create_agents();
        for agent in &mut self.agents {
            let _: Result<(), Box<dyn std::error::Error>> = agent.execute(&mut self.fact_sheet).await;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_manager_agent() {
        let user_request: &str = "I need a full stack app that fetch and tracks user fitness progress. Need to include timezone info from the web";

        let mut manager = ManagingAgent::new(user_request.to_string())
            .await
            .expect("Error during manager creation");
        manager.execute_project().await;
        dbg!(manager.fact_sheet);
    }
}
