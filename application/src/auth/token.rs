use bcrypt::verify;

use diesel::prelude::*;
use domain::models::{LoginRequest, User};
use infrastructure::establish_connection_pool;

use rocket::serde::json::Json;
use shared::response_models::{create_jwt, NetworkResponse, Response, ResponseBody};

pub fn login_user(user: Json<LoginRequest>) -> Result<String, NetworkResponse> {
    use infrastructure::schema::users::dsl::*;

    let user = user.into_inner();

    let mut connection = establish_connection_pool()
        .get()
        .expect("Failed to get connection from pool");

    let db_user: User = match users
        .select(users::all_columns())
        .filter(email.eq(&user.email))
        .first(&mut connection)
    {
        Ok(fetched_user) => fetched_user,
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response {
                    status: false,
                    message: "Error - Wrong username or password".to_string(),
                    body: ResponseBody::Message(format!(
                        "Error - Wrong username or password for user {}",
                        &user.email
                    )),
                };
                return Err(NetworkResponse::NotFound(
                    serde_json::to_string(&response).unwrap(),
                ));
            }
            _ => {
                panic!("Database error - {}", err);
            }
        },
    };

    // Verify the provided password against the hashed password stored in the database
    if verify(&user.password, &db_user.password).unwrap_or(false) {
        // Passwords match, create JWT token
        match create_jwt(db_user.id) {
            Ok(token) => Ok(token),
            Err(err) => Err(NetworkResponse::BadRequest(err.to_string())),
        }
    } else {
        // Passwords don't match
        let response = Response {
            status: false,
            message: "Error - Wrong username or password".to_string(),
            body: ResponseBody::Message(format!(
                "Error - Wrong username or password for user {}",
                &user.email
            )),
        };
        Err(NetworkResponse::Unauthorized(
            serde_json::to_string(&response).unwrap(),
        ))
    }

    // match create_jwt(10) {
    //     Ok(token) => Ok(token),
    //     Err(err) => Err(NetworkResponse::BadRequest(err.to_string())),
    // }
}
