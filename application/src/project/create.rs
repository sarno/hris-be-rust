use diesel::prelude::*;
use domain::models::{NewProject, Project};
use infrastructure::establish_connection_pool;
use rocket::response::status::Created;
use rocket::serde::json::Json;
use shared::response_models::{Response, ResponseBody};

pub fn create_project(project: Json<NewProject>) -> Created<String> {
    use infrastructure::schema::projects;

    let mut connection = establish_connection_pool()
        .get()
        .expect("Failed to get connection from pool");

    let project = project.into_inner();

    match diesel::insert_into(projects::table)
        .values(&project)
        .get_result::<Project>(&mut connection)
    {
        Ok(post) => {
            let response = Response {
                status: true,
                message: "Project created successfully".to_string(),
                body: ResponseBody::Post(post),
            };
            Created::new("").tagged_body(serde_json::to_string(&response).unwrap())
        }
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}
