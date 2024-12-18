mod ai_functions;
mod api_handler;
mod helpers;
mod models;

use helpers::command_lines::get_user_response;

fn main() {
    let user_req: String = get_user_response("What are we building today?");
    dbg!(user_req);
}
