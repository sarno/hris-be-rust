use application::auth::token::login_user;
use domain::models::LoginRequest;
use rocket::post;
use rocket::serde::json::Json;
use shared::response_models::NetworkResponse;
use shared::response_models::{Response, ResponseBody};

#[post("/login", format = "application/json", data = "<user>")]
pub fn login_user_handler(user: Json<LoginRequest>) -> Result<String, NetworkResponse> {
    let token = login_user(user)?;
    // let token = "token".to_string();

    let response = Response {
        status: true,
        message: "User logged in successfully".to_string(),
        body: ResponseBody::AuthToken(token),
    };

    Ok(serde_json::to_string(&response).unwrap())
}
