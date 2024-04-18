use diesel::{dsl::now, prelude::*};
use domain::models::Project;
use infrastructure::establish_connection_pool;
use rocket::response::status::NotFound;
use shared::response_models::{Response, ResponseBody};

pub fn publish_project(project_id: i64) -> Result<Project, NotFound<String>> {
    use infrastructure::schema::projects;

    let mut connection = establish_connection_pool()
        .get()
        .expect("Failed to get connection from pool");

    match diesel::update(projects::table.filter(projects::id.eq(project_id)))
        .set(projects::created_at.eq(now))
        .get_result::<Project>(&mut connection)
    {
        Ok(project) => Ok(project),
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response {
                    status: false,
                    message: "Error publishing project".to_string(),
                    body: ResponseBody::Message(format!(
                        "Error publishing project with id {} - {}",
                        project_id, err
                    )),
                };
                Err(NotFound(serde_json::to_string(&response).unwrap()))
            }
            _ => panic!("Database error - {}", err),
        },
    }
}
