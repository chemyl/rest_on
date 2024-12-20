use crate::models::agent_basic::basic_agent::BasicAgent;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Debug, PartialEq, Copy, Clone)]
pub struct ProjectScope {
    pub is_crude_required: bool,
    pub is_user_login_and_logout: bool,
    pub is_external_urls_required: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct RouteObject {
    pub is_route_dynamic: String,
    pub method: String,
    pub request_body: serde_json::Value,
    pub response_body: serde_json::Value,
    pub route_id: String,
}

// структура данных, хранящая факты, которые обрабатываются как менеджером, так и агентами (определена в модуле agent_traits).
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct FactSheet {
    pub project_description: String,
    pub project_scope: Option<ProjectScope>,
    pub external_urls: Option<Vec<String>>,
    pub backend_code: Option<String>,
    pub api_endpoint_schema: Option<Vec<RouteObject>>,
}

//SpecialFunctions — задаёт интерфейс взаимодействия между менеджером и агентами.
pub trait SpecialFunctions: Debug {
    // Метод Возвращает ссылку на базовый агент (BasicAgent), представляющий атрибуты агента.
    fn get_attributes_from_agent(&self) -> &BasicAgent;

    // Асинхронная функция, принимающая на вход структуру FactSheet и выполняющая какую-то логику. Возвращает Result:
    async fn execute(
        &mut self,
        fact_sheet: &mut FactSheet,
    ) -> Result<(), Box<dyn std::error::Error>>;
}
