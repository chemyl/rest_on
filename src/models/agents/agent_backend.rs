use crate::ai_functions::ai_func_backend::{
    print_backend_webserver_code, print_fixed_code, print_improved_webserver_code,
    print_rest_api_endpoints,
};
use crate::helpers::general::{
    check_status_code, read_code_template_contents, read_exec_main_contents, save_api_endpoints,
    save_backend_code,
};

use crate::helpers::command_lines::{confirm_safe_code, PrintCommand};
use crate::helpers::general::{ai_task_request, WEB_SERVER_PROJECT_PATH};
use crate::models::agent_basic::basic_agent::{AgentState, BasicAgent};
use crate::models::agents::agent_traits::{FactSheet, RouteObject, SpecialFunctions};

use async_trait::async_trait;
use reqwest::Client;
use std::process::{Command, Stdio};
use std::time::Duration;
use tokio::time;

#[derive(Debug)]
pub struct AgentBackendDeveloper {
    attributes: BasicAgent,
    bug_errors: Option<String>,
    bug_count: u8,
}

impl AgentBackendDeveloper {
    pub fn new() -> Self {
        let attributes: BasicAgent = BasicAgent {
            objective: "Develops backend code for webserver and json database".to_string(),
            position: "Backend Developer".to_string(),
            state: AgentState::Discovery,
            memory: vec![],
        };

        Self {
            attributes,
            bug_errors: None,
            bug_count: 0,
        }
    }

    async fn call_initial_backend_code(&mut self, fact_sheet: &mut FactSheet) {
        let code_template_str: String = read_code_template_contents();
        // println!(
        //     "* ==============FIRST Backend code TEMPLATE: {:?}",
        //     &code_template_str
        // );
        // Concatenate Instruction
        let msg_context: String = format!(
            "CODE TEMPLATE: {} \n PROJECT_DESCRIPTION: {} \n",
            code_template_str, fact_sheet.project_description
        );

        let ai_response: String = ai_task_request(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_backend_webserver_code),
            print_backend_webserver_code,
        )
            .await;
        // println!(
        //     "* ==============FIRST Backend(AI RESPONSE) code to Save: {:?}",
        //     &ai_response
        // );
        save_backend_code(&ai_response);
        fact_sheet.backend_code = Some(ai_response);
    }

    async fn call_improved_backend_code(&mut self, factsheet: &mut FactSheet) {
        // println!(
        //     "* ==============SECOND  Backend code TEMPLATE: {:?}",
        //     &factsheet.backend_code
        // );
        let msg_context: String = format!(
            "CODE TEMPLATE: {:?} \n PROJECT_DESCRIPTION: {:?} \n",
            factsheet.backend_code, factsheet
        );

        let ai_response: String = ai_task_request(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_improved_webserver_code),
            print_improved_webserver_code,
        )
            .await;
        // println!(
        //     "* ==============SECOND (AI RESPONSE) Backend code to save: {:?}",
        //     &ai_response
        // );
        save_backend_code(&ai_response);
        factsheet.backend_code = Some(ai_response);
    }

    async fn call_fix_code_bugs(&mut self, fact_sheet: &mut FactSheet) {
        let msg_context: String = format!(
            "BROKEN_CODE: {:?} \n ERROR_BUGS: {:?} \n
      THIS FUNCTION ONLY OUTPUTS CODE. JUST OUTPUT THE CODE.",
            fact_sheet.backend_code, self.bug_errors
        );

        let ai_response: String = ai_task_request(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_fixed_code),
            print_fixed_code,
        )
            .await;

        save_backend_code(&ai_response);
        fact_sheet.backend_code = Some(ai_response);
    }

    async fn call_extract_rest_api_endpoints(&self) -> String {
        let backend_code: String = read_exec_main_contents();

        // Structure message context
        let msg_context: String = format!("CODE_INPUT: {}", backend_code);

        let ai_response: String = ai_task_request(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_rest_api_endpoints),
            print_rest_api_endpoints,
        )
            .await;

        ai_response
    }
}

#[async_trait]
impl SpecialFunctions for AgentBackendDeveloper {
    fn get_attributes_from_agent(&self) -> &BasicAgent {
        &self.attributes
    }

    async fn execute(&mut self, fact_sheet: &mut FactSheet) -> Result<(), Box<dyn std::error::Error>> {
        while self.attributes.state != AgentState::Finished {

            match &self.attributes.state {

                AgentState::Discovery => {
                    self.call_initial_backend_code(fact_sheet).await;
                    self.attributes.state = AgentState::Working;
                    continue;
                }

                AgentState::Working => {
                    if self.bug_count == 0 {
                        self.call_improved_backend_code(fact_sheet).await;
                    } else {
                        self.call_fix_code_bugs(fact_sheet).await;
                    }
                    self.attributes.state = AgentState::UnitTesting;
                    continue;
                }

                AgentState::UnitTesting => {
                    // API SAFETY GUARD
                    PrintCommand::UnitTest.print_agent_message(
                        &self.attributes.position.as_str(),
                        "Backend Code Unit Testing: Ensuring Safe Code",
                    );

                    let user_confirmation: bool = confirm_safe_code();

                    if !user_confirmation {
                        panic!("Better go work on some AI alignment instead...");
                    }

                    // Build and testing code
                    PrintCommand::UnitTest.print_agent_message(
                        &self.attributes.position.as_str(),
                        "Backend Code Unit Testing: Building web server...",
                    );

                    let build_backend_server: std::process::Output = Command::new("cargo")
                        .arg("build")
                        .current_dir(WEB_SERVER_PROJECT_PATH)
                        .stdout(Stdio::piped())
                        .stderr(Stdio::piped())
                        .output()
                        .expect("Failed to build backend application");

                    // determine errors
                    if build_backend_server.status.success() {
                        self.bug_count = 0;
                        PrintCommand::UnitTest.print_agent_message(
                            &self.attributes.position.as_str(),
                            "Backend Code Unit Testing: Test server build successful...");
                    } else {
                        let error_arr: Vec<u8> = build_backend_server.stderr;
                        let error_str = String::from_utf8(error_arr).unwrap();
                        //update error count
                        self.bug_count += 1;
                        self.bug_errors = Some(error_str);

                        if self.bug_count > 2 {
                            PrintCommand::Issue.print_agent_message(
                                self.attributes.position.as_str(),
                                "Backend Code Unit Testing: Too many bugs found in code",
                            );
                            panic!("Error: Too many bugs")
                        }

                        // Pass back for rework
                        self.attributes.state = AgentState::Working;
                        continue;
                    }

                    /*
                        Extract and test REST API endpoints
                    */

                    // Exctract API Endpoints
                    let api_endpoints_str: String = self.call_extract_rest_api_endpoints().await;

                    // convert API endpoints into value
                    let api_endpoints: Vec<RouteObject> =
                        serde_json::from_str(api_endpoints_str.as_str())
                            .expect("Failed to parse API endpoints");

                    // define endpoints checks. Выбрать в итераторе только проверяемые ендпоинты
                    let check_endpoints: Vec<RouteObject> = api_endpoints
                        .iter()
                        .filter(|&route_object| {
                            route_object.method == "get" && route_object.is_route_dynamic == "false"
                        })
                        .cloned()
                        .collect();

                    // Store API endpoints
                    fact_sheet.api_endpoint_schema = Some(check_endpoints.clone());

                    //Run backend application
                    PrintCommand::UnitTest.print_agent_message(
                        &self.attributes.position.as_str(),
                        "Backend Code Unit Testing: Starting Web server...",
                    );

                    let mut run_backend_server: std::process::Child = Command::new("cargo")
                        .arg("run")
                        .current_dir(WEB_SERVER_PROJECT_PATH)
                        .stdout(Stdio::piped())
                        .stderr(Stdio::piped())
                        .spawn()
                        .expect("Failed to run backend application");

                    //Launching testing on the server
                    PrintCommand::UnitTest.print_agent_message(
                        &self.attributes.position.as_str(),
                        "Backend Code Unit Testing: Launching test on server in 5 sec...",
                    );

                    //Sleep for 5 sec before call webserver
                    let seconds_sleep: Duration = Duration::from_secs(5);
                    time::sleep(seconds_sleep).await;

                    // check endpoints against server
                    for endpoint in check_endpoints {
                        let test_message: String =
                            format!("Testing endpoint: '{}...'", endpoint.route);
                        PrintCommand::UnitTest.print_agent_message(
                            &self.attributes.position.as_str(),
                            test_message.as_str(),
                        );

                        //create client to call endpoints api
                        let client: Client = Client::builder()
                            .timeout(Duration::from_secs(5))
                            .build()
                            .unwrap();

                        // test url
                        let url: String = format!("http://127.0.0.1:8080{}", endpoint.route);
                        match check_status_code(&client, &url).await {
                            Ok(status_code) => {
                                if status_code != 200 {
                                    let error_msg: String = format!(
                                        "WARNING: Failed to call backend endpoint {}",
                                        endpoint.route
                                    );
                                    PrintCommand::Issue.print_agent_message(
                                        &self.attributes.position.as_str(),
                                        error_msg.as_str(),
                                    );
                                }
                            }
                            Err(error_msg) => {
                                //kill process
                                run_backend_server
                                    .kill()
                                    .expect("Unable to stop backend application");
                                let error_msg: String =
                                    format!("ERROR: While checking backend {}", error_msg);
                                PrintCommand::Issue.print_agent_message(
                                    &self.attributes.position.as_str(),
                                    error_msg.as_str(),
                                );
                            }
                        }
                    }

                    save_api_endpoints(&api_endpoints_str);
                    PrintCommand::Success.print_agent_message(
                        &self.attributes.position.as_str(),
                        "Backend Testing completed...",
                    );

                    run_backend_server
                        .kill()
                        .expect("failed to Kill server on completion");
                    self.attributes.state = AgentState::Finished;
                }
                _ => {}
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn tests_backend_developer() {
        let mut agent: AgentBackendDeveloper = AgentBackendDeveloper::new();

        let fact_sheet_str: &str = r#"
      {
        "project_description": "build a website that fetches and tracks fitness progress with timezone information",
        "project_scope": {
          "is_crud_required": true,
          "is_user_login_and_logout": true,
          "is_external_urls_required": true
        },
        "external_urls": [
          "http://worldtimeapi.org/api/timezone"
        ],
        "backend_code": null,
        "api_endpoint_schema": null
      }"#;

        let mut factsheet: FactSheet = serde_json::from_str(fact_sheet_str).unwrap();

        // agent.attributes.state = AgentState::Discovery;
        agent.attributes.state = AgentState::UnitTesting;
        agent
            .execute(&mut factsheet)
            .await
            .expect("Failed to execute Backend Developer agent");
    }
}
