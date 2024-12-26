use crate::ai_functions::ai_func_architect::{print_project_scope, print_site_urls};
use crate::helpers::command_lines::PrintCommand;
use crate::helpers::general::{ai_task_request_decoded, check_status_code};
use crate::models::agent_basic::basic_agent::{AgentState, BasicAgent};
use crate::models::agent_basic::basic_traits::BasicTraits;
use crate::models::agents::agent_traits::{FactSheet, ProjectScope, SpecialFunctions};
use async_trait::async_trait;
use reqwest::Client;
use std::error::Error;
use std::time::Duration;
// Solution Architect

#[derive(Debug)]
pub struct AgentSolutionArchitect {
    attributes: BasicAgent,
}

impl AgentSolutionArchitect {
    pub fn new() -> Self {
        let attributes: BasicAgent = BasicAgent {
            objective: "Gathers information and design solutions for website development"
                .to_string(),
            position: "Solutions Architect".to_string(),
            state: AgentState::Discovery,
            memory: vec![],
        };

        Self { attributes }
    }

    // Retrieve Project Scope
    async fn call_project_scope(&mut self, fact_sheet: &mut FactSheet) -> ProjectScope {
        let project_description: String = format!("{}", fact_sheet.project_description);

        let decoded_project_scope: ProjectScope = ai_task_request_decoded::<ProjectScope>(
            project_description,
            &self.attributes.position,
            get_function_string!(print_project_scope),
            print_project_scope).await;

        fact_sheet.project_scope = Some(decoded_project_scope.clone());
        self.attributes.update_state(AgentState::Finished);
        decoded_project_scope
    }

    async fn call_determine_external_urls(&mut self, fact_sheet: &mut FactSheet, msg_context: String) {
        let decoded_project_urls: Vec<String> = ai_task_request_decoded::<Vec<String>>(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_site_urls),
            print_site_urls).await;

        fact_sheet.external_urls = Some(decoded_project_urls);
        self.attributes.state = AgentState::UnitTesting;
    }
}

#[async_trait]
impl SpecialFunctions for AgentSolutionArchitect {
    fn get_attributes_from_agent(&self) -> &BasicAgent {
        &self.attributes
    }
    async fn execute(&mut self, fact_sheet: &mut FactSheet) -> Result<(), Box<dyn Error>> {
        while self.attributes.state != AgentState::Finished {
            match self.attributes.state {
                AgentState::Discovery => {
                    let project_scope: ProjectScope = self.call_project_scope(fact_sheet).await;

                    // Confirm if external urls
                    if project_scope.is_external_urls_required {
                        self.call_determine_external_urls(
                            fact_sheet,
                            fact_sheet.project_description.clone()).await;
                        self.attributes.state = AgentState::UnitTesting;
                    }
                }

                AgentState::UnitTesting => {
                    let mut excluded_external_urls: Vec<String> = vec![];

                    let client: Client = Client::builder()
                        .timeout(Duration::from_secs(5))
                        .build().unwrap();

                    // Defining urls to check
                    let urls: &Vec<String> = fact_sheet
                        .external_urls.as_ref().expect("No URL object on fact_sheet");

                    // Find faulty urls
                    for url in urls {
                        let endpoint_str: String = format!("Testing External URL Endpoint: {}", url);
                        PrintCommand::UnitTest.print_agent_message(self.attributes.position.as_str(), endpoint_str.as_str());

                        // Perform URL Test
                        match check_status_code(&client, url).await {
                            Ok(status_code) => {
                                if status_code != 200 {
                                    excluded_external_urls.push(url.clone())
                                }
                            }
                            Err(e) => println!("Error checking {}: {}", url, e),
                        }
                    }

                    // Exclude any faulty urls
                    if excluded_external_urls.len() > 0 {
                        let confirmed_external_urls: Vec<String> = fact_sheet
                            .external_urls
                            .as_ref()
                            .unwrap()
                            .iter()
                            .filter(|url| !excluded_external_urls.contains(&url))
                            .cloned()
                            .collect();
                        fact_sheet.external_urls = Some(confirmed_external_urls);
                    }
                    self.attributes.state = AgentState::Finished;
                }
                _ => self.attributes.state = AgentState::Finished
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn tests_solution_architect() {
        let mut agent: AgentSolutionArchitect = AgentSolutionArchitect::new();

        let mut factsheet: FactSheet = FactSheet {
            project_description: "Build a full stack website with user login and logout that shows latest Forex prices".to_string(),
            project_scope: None,
            external_urls: None,
            backend_code: None,
            api_endpoint_schema: None,
        };

        agent
            .execute(&mut factsheet)
            .await
            .expect("Unable to execute Solutions Architect Agent");
        assert!(factsheet.project_scope != None);
        assert!(factsheet.external_urls.is_some());
        dbg!(factsheet);
    }
}
