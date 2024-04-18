use diesel::prelude::*;
use domain::models::Project;
use infrastructure::establish_connection_pool;
use infrastructure::schema::projects::dsl::*;
use rocket::response::status::NotFound;
use shared::response_models::{Response, ResponseBody};

pub async fn list_project(project_id: i64) -> Result<Project, NotFound<String>> {
    let mut connection = establish_connection_pool()
        .get()
        .expect("Failed to get connection from pool");

    match projects
        .filter(id.eq(project_id))
        .first::<Project>(&mut connection)
    {
        Ok(project) => Ok(project),
        Err(diesel::result::Error::NotFound) => {
            let response = Response {
                status: false,
                message: "Error selecting project".to_string(),
                body: ResponseBody::Message(format!(
                    "Error selecting project with id {} - not found",
                    project_id
                )),
            };
            Err(NotFound(serde_json::to_string(&response).unwrap()))
        }
        Err(err) => panic!("Database error - {}", err),
    }
}

pub fn list_projects() -> Vec<Project> {
    let mut connection = establish_connection_pool()
        .get()
        .expect("Failed to get connection from pool");

    match infrastructure::schema::projects::table
        .select((
            infrastructure::schema::projects::id,
            infrastructure::schema::projects::project_code,
            infrastructure::schema::projects::project_name,
            infrastructure::schema::projects::company_id,
            infrastructure::schema::projects::created_at,
            infrastructure::schema::projects::updated_at,
        ))
        .load::<Project>(&mut connection)
    {
        Ok(mut fetched_projects) => {
            fetched_projects.sort();
            fetched_projects
        }
        Err(err) => panic!("Database error - {}", err),
    }
}
