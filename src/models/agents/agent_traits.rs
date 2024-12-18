use serde::{Deserialize, Serialize};
use std::fmt::Debug;


#[derive(Serialize, Deserialize, Debug, PartialEq, Copy, Clone)]
pub struct ProjectScope{
    pub is_crude_required: bool,
    pub is_user_login_and_logout: bool,
    pub is_external_urls_required: bool
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct RouteObject{
    pub is_route_dynamic: String,
    pub method: String,
    pub request_body: serde_json::Value,
    pub response_body: serde_json::Value,
    pub route_id: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct FactSheet {
    pub project_description: String,
    pub project_scope: Option<ProjectScope>,
    pub external_urls: Option<Vec<String>>,
    pub backend_code: Option<String>,
    pub api_endpoint_schema: Option<Vec<RouteObject>>,
}