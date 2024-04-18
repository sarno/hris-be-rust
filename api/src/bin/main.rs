#[macro_use]
extern crate rocket;
use api::auth_handler;
use api::project_handler;
use rocket::serde::json::{json, Value};

#[catch(404)]
fn not_found() -> Value {
    json!("Not found!")
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/api",
            routes![
                project_handler::list_projects_handler,
                project_handler::list_project_handler,
                project_handler::create_project_handler,
                project_handler::publish_project_handler,
                project_handler::delete_project_handler,
                auth_handler::login_user_handler,
            ],
        )
        .register("/", catchers![not_found])
}
