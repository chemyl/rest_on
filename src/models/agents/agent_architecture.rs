use crate::ai_functions::ai_func_architect::{print_project_scope, print_site_urls};
use crate::helpers::command_lines::PrintCommand;
use crate::helpers::general::{ai_task_request_decoded, check_status_code};
use crate::models::agent_basic::basic_agent::{AgentState, BasicAgent};
use crate::models::agent_basic::basic_traits::BasicTrait;
use crate::models::agents::agent_traits::{FactSheet, ProjectScope, SpecialFunctions};

// Solution Architect

#[derive(Debug)]
pub struct AgentSolutionArchitect {
    attributes: BasicAgent,
}

impl AgentSolutionArchitect {
    fn new() -> AgentSolutionArchitect {
        let attributes = BasicAgent {
            objective: "Gathers information and design solution for development".to_string(),
            position: "Solution Architect".to_string(),
            state: AgentState::Discovery,
            memory: vec![],
        };
        Self { attributes }
    }
    // Retrieve Project Scope
    async fn call_project_scope(&mut self, fact_sheet: &mut FactSheet) -> ProjectScope {
        let msg_context: String = format!("{:?}", fact_sheet.project_description);

        let ai_response: ProjectScope = ai_task_request_decoded::<ProjectScope>(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_project_scope),
            print_project_scope,
        )
        .await;

        fact_sheet.project_scope = Some(ai_response.clone());
        self.attributes.update_state(AgentState::Finished);
        ai_response
    }
}
