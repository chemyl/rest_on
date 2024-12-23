use crate::ai_functions::ai_func_architect::print_project_scope;
use crate::ai_functions::ai_func_backend::{print_backend_webserver_code, print_fixed_code, print_improved_webserver_code, print_rest_api_endpoints};
use crate::helpers::general::{ai_task_request, ai_task_request_decoded, read_code_template_contents, read_exec_main_contents, save_backend_code};
use crate::models::agent_basic::basic_agent::{AgentState, BasicAgent};
use crate::models::agents::agent_traits::{FactSheet, ProjectScope};

#[derive(Debug)]
pub struct AgentBackendDeveloper {
    attributes: BasicAgent,
    bug_error: Option<String>,
    bug_count: u8,
}

impl AgentBackendDeveloper {
    pub fn new() -> Self {
        let agent_position = "Backend Developer".to_string();
        let attributes = BasicAgent {
            objective: "Develop & Test backend code for webserver".to_string(),
            position: agent_position.clone(),
            state: AgentState::Discovery,
            memory: vec![],
        };
        Self {
            attributes,
            bug_error: None,
            bug_count: 0,
        }
    }

    async fn call_initial_backend_code(&mut self, fact_sheet: &mut FactSheet) {
        let code_template_str: String = read_code_template_contents();

// concat instruction
        let mut msg_context: String = format!(
            "CODE TEMPLATE: {} \n PROJECT DESCRIPTION: {} \n",
            code_template_str, fact_sheet.project_description
        );

        let ai_response: String = ai_task_request(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_backend_webserver_code),
            print_backend_webserver_code,
        ).await;

// save backend code to rewrite
        save_backend_code(&ai_response);
        fact_sheet.backend_code = Some(ai_response);
    }


    async fn call_improved_backend_code(&mut self, fact_sheet: &mut FactSheet) {
        let mut msg_context: String = format!(
            "CODE TEMPLATE: {:?} \n PROJECT DESCRIPTION: {:?} \n",
            fact_sheet.backend_code, fact_sheet
        );

        let ai_response: String = ai_task_request(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_improved_webserver_code),
            print_improved_webserver_code,
        ).await;

        // save backend code to rewrite
        save_backend_code(&ai_response);
        fact_sheet.backend_code = Some(ai_response);
    }


    async fn call_fix_code_bugs(&mut self, fact_sheet: &mut FactSheet) {
        let mut msg_context: String = format!(
            "BROKEN CODE: {:?} \n ERROR BUGS: {:?} \n
            THIS FUNCTION ONLY OUTPUTS CODE. JUST OUTPUT THE CODE",
            fact_sheet.backend_code, self.bug_error
        );

        let ai_response: String = ai_task_request(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_fixed_code),
            print_fixed_code,
        ).await;

        // save backend code. rewrite to file
        save_backend_code(&ai_response);
        fact_sheet.backend_code = Some(ai_response);
    }

    async fn call_extract_rest_api_endpoints(&self)->String{
        let backend_code = read_exec_main_contents();

        // structure message context
        let msg_context: String = format!("CODE INPUT: {:?}", backend_code);

        let ai_response: String = ai_task_request(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_rest_api_endpoints),
            print_rest_api_endpoints,
        ).await;
        ai_response
    }

}