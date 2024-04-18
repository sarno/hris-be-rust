use diesel::prelude::*;
use domain::models::Project;
use infrastructure::establish_connection_pool;
use rocket::response::status::NotFound;
use shared::response_models::{Response, ResponseBody};

pub fn delete_project(project_id: i64) -> Result<Vec<Project>, NotFound<String>> {
    use infrastructure::schema::projects;
    use infrastructure::schema::projects::dsl::*;

    let mut connection = establish_connection_pool()
        .get()
        .expect("Failed to get connection from pool");

    let response: Response;

    let num_deleted =
        match diesel::delete(projects.filter(id.eq(project_id))).execute(&mut connection) {
            Ok(count) => count,
            Err(err) => match err {
                diesel::result::Error::NotFound => {
                    let response = Response {
                        status: false,
                        message: "Error deleting post".to_string(),
                        body: ResponseBody::Message(format!(
                            "Error deleting post with id {} - {}",
                            project_id, err
                        )),
                    };
                    return Err(NotFound(serde_json::to_string(&response).unwrap()));
                }
                _ => {
                    panic!("Database error - {}", err);
                }
            },
        };

    if num_deleted > 0 {
        match projects::table
            .select(projects::all_columns)
            .load::<Project>(&mut connection)
        {
            Ok(mut posts_) => {
                posts_.sort();
                Ok(posts_)
            }
            Err(err) => match err {
                _ => {
                    panic!("Database error - {}", err);
                }
            },
        }
    } else {
        response = Response {
            status: false,
            message: "Error deleting post".to_string(),
            body: ResponseBody::Message(format!("Error - no post with id {}", project_id)),
        };
        Err(NotFound(serde_json::to_string(&response).unwrap()))
    }
}
