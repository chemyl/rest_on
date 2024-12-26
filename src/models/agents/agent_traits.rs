use crate::models::agent_basic::basic_agent::BasicAgent;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// Represents an object describing a route in the API, including its method,
/// whether it is dynamic, request/response body schemas, and the route itself.
///
/// # Fields
/// - `is_route_dynamic`: Indicates if the route is dynamic (e.g., contains path parameters).
/// - `method`: The HTTP method associated with the route (e.g., GET, POST).
/// - `request_body`: A JSON schema for the request body.
/// - `response`: A JSON schema for the response body.
/// - `route`: The route string, e.g., `/api/users/{id}`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RouteObject {
    pub is_route_dynamic: String,
    pub method: String,
    pub request_body: serde_json::Value,
    pub response: serde_json::Value,
    pub route: String,
}

/// Represents the scope of the project, specifying features and requirements.
///
/// # Fields
/// - `is_crud_required`: Indicates if CRUD operations are required for the project.
/// - `is_user_login_and_logout`: Indicates if user authentication is needed.
/// - `is_external_urls_required`: Indicates if external URLs are part of the project requirements.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct ProjectScope {
    pub is_crud_required: bool,
    pub is_user_login_and_logout: bool,
    pub is_external_urls_required: bool,
}

/// Contains details about the project, including its description, scope, and other metadata.
///
/// # Fields
/// - `project_description`: A textual description of the project's requirements and goals.
/// - `project_scope`: Optional information about the project's scope.
/// - `external_urls`: Optional list of external URLs related to the project.
/// - `backend_code`: Optional string containing generated backend code.
/// - `api_endpoint_schema`: Optional schema describing API endpoints as a list of `RouteObject`s.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FactSheet {
    pub project_description: String,
    pub project_scope: Option<ProjectScope>,
    pub external_urls: Option<Vec<String>>,
    pub backend_code: Option<String>,
    pub api_endpoint_schema: Option<Vec<RouteObject>>,
}

/// Defines the interface for agents to implement special functions.
///
/// # Methods
/// - `get_attributes_from_agent`: Returns a reference to the agent's attributes.
/// - `execute`: Executes the agent's workflow using a mutable reference to the `FactSheet`.
#[async_trait]
pub trait SpecialFunctions: Debug {
    fn get_attributes_from_agent(&self) -> &BasicAgent;
    async fn execute(
        &mut self,
        fact_sheet: &mut FactSheet,
    ) -> Result<(), Box<dyn std::error::Error>>;
}
