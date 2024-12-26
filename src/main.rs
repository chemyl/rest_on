#[macro_export]
macro_rules! get_function_string {
    ($func: ident) => {{
        stringify!($func)
    }};
}

#[macro_use]
mod ai_functions;
mod api_handler;
mod helpers;
mod models;

use crate::helpers::command_lines::get_user_response;
use models::agents_manager::managing_agents::ManagingAgent;

#[tokio::main]
async fn main() {
    let usr_req: String = get_user_response("What website are we building today?");

    let mut manage_agent: ManagingAgent = ManagingAgent::new(usr_req)
        .await
        .expect("Error creating agent");

    manage_agent.execute_project().await;
}
