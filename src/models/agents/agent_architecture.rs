use crate::ai_functions::ai_func_architect::{print_project_scope, print_site_urls};
use crate::helpers::command_lines::PrintCommand;
use crate::helpers::general::{ai_task_request_decoded, check_status_code};
use crate::models::agent_basic::basic_agent::{AgentState, BasicAgent};
use crate::models::agent_basic::basic_traits::BasicTrait;
use crate::models::agents::agent_traits::{FactSheet, ProjectScope, SpecialFunctions};
use async_trait::async_trait;
use reqwest::Client;
use std::error::Error;
use std::time::Duration;
// Solution Architect

// Структура AgentSolutionArchitect представляет собой агента, работающего с AI-сервисами для:
// Формирования области проекта (Project Scope).
// Определения внешних ссылок, относящихся к проекту.

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
        //Формирует контекст сообщения из описания проекта (fact_sheet.project_description).
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
    // Retrieve Project external links
    async fn call_determine_external_link(&mut self, fact_sheet: &mut FactSheet, msg_context: String) {
        //Использует ai_task_request_decoded для получения внешних ссылок проекта с помощью функции print_site_urls.
        let ai_response: Vec<String> = ai_task_request_decoded::<Vec<String>>(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_site_urls),
            print_site_urls,
        )
            .await;
        fact_sheet.external_urls = Some(ai_response);
        self.attributes.update_state(AgentState::UnitTesting);
    }
}

#[async_trait]
impl SpecialFunctions for AgentSolutionArchitect {
    fn get_attributes_from_agent(&self) -> &BasicAgent {
        &self.attributes
    }
    async fn execute(&mut self, fact_sheet: &mut FactSheet) -> Result<(), Box<dyn Error>> {
        // Warning - infinity loop!
        while self.attributes.state != AgentState::Finished {
            match self.attributes.state {
                AgentState::Discovery => {
                    let project_scope: ProjectScope = self.call_project_scope(fact_sheet).await;
                    // confirm is external links
                    if project_scope.is_external_urls_required {
                        self.call_determine_external_link(fact_sheet, fact_sheet.project_description.clone()).await;
                        self.attributes.state = AgentState::UnitTesting;
                    }
                }
                AgentState::UnitTesting => {
                    let mut exclude_urls: Vec<String> = vec![];
                    let client: Client = Client::builder()
                        .timeout(Duration::from_secs(5))
                        .build()?;

                    // fin broken urls
                    let urls: &Vec<String> = fact_sheet.external_urls.as_ref().expect("No URL object in fact_sheet");

                    for url in urls {
                        let endpoint_str: String = format!("Testing URL endpoint: {}", url);
                        PrintCommand::UnitTest.print_agent_message(
                            self.attributes.position.as_str(), endpoint_str.as_str());

                        // perform url Test
                        match check_status_code(&client, url).await {
                            Ok(status_code) => {
                                if status_code != 200 {
                                    exclude_urls.push(url.clone());
                                }
                            }
                            Err(_) => println!("Error while checking URL: {}", url),
                        }
                    };
                    if exclude_urls.len() > 0 {
                        let new_urls: Vec<String> = fact_sheet
                            .external_urls
                            .as_ref()
                            .unwrap()
                            .iter()
                            .filter(|url| !exclude_urls.contains(&url))
                            .cloned()
                            .collect();
                        fact_sheet.external_urls = Some(new_urls);
                    }
                    // confirm Done
                    self.attributes.update_state(AgentState::Finished);
                }
                // Default to finished state
                _ => {
                    self.attributes.state = AgentState::Finished;
                }
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

        let mut fact_sheet: FactSheet = FactSheet {
            project_description: "Build a full stack crud website with user login and logout that shows latest Forex prices".to_string(),
            project_scope: None,
            external_urls: None,
            backend_code: None,
            api_endpoint_schema: None,
        };

        agent.execute(&mut fact_sheet)
            .await
            .expect("Unable to execute Solutions Architect Agent");
        assert_ne!(fact_sheet.project_scope, None);
        assert!(fact_sheet.external_urls.is_some());
        dbg!(fact_sheet);
    }
}

