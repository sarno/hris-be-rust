use application::project::{create, delete, publish, read};
use domain::models::{NewProject, Project};
use rocket::response::status::{Created, NotFound};
use rocket::serde::json::Json;
use rocket::{delete, get, post, put};

use shared::response_models::{NetworkResponse, Response, ResponseBody, JWT};

#[get("/projects")]
pub fn list_projects_handler(key: Result<JWT, NetworkResponse>) -> Result<String, NetworkResponse> {
    let key = match key {
        Ok(key) => key,
        Err(err) => return Err(err),
    };

    // change key to string

    let projects: Vec<Project> = read::list_projects();
    let response = Response {
        status: true,
        message: key.claims.subject_id.to_string(),
        body: ResponseBody::Posts(projects),
    };

    Ok(serde_json::to_string(&response).unwrap())
}

#[get("/project/<project_id>")]
pub async fn list_project_handler(project_id: i64) -> Result<String, NotFound<String>> {
    match read::list_project(project_id).await {
        Ok(project) => {
            let response = Response {
                status: true,
                message: "Project found".to_string(),
                body: ResponseBody::Post(project),
            };
            Ok(serde_json::to_string(&response).unwrap())
        }
        Err(not_found) => Err(not_found),
    }
}

#[post("/project", format = "application/json", data = "<project>")]
pub fn create_project_handler(project: Json<NewProject>) -> Created<String> {
    create::create_project(project)
}

#[put("/project/<project_id>")]
pub fn publish_project_handler(project_id: i64) -> Result<String, NotFound<String>> {
    let post = publish::publish_project(project_id)?;
    let response = Response {
        status: true,
        message: "Project published".to_string(),
        body: ResponseBody::Post(post),
    };

    Ok(serde_json::to_string(&response).unwrap())
}

#[delete("/project/<project_id>")]
pub fn delete_project_handler(project_id: i64) -> Result<String, NotFound<String>> {
    let posts = delete::delete_project(project_id)?;
    let response = Response {
        status: true,
        message: "Project deleted".to_string(),
        body: ResponseBody::Posts(posts),
    };

    Ok(serde_json::to_string(&response).unwrap())
}
