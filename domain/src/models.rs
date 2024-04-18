use chrono::{NaiveDate, NaiveDateTime};
use diesel::{deserialize::Queryable, prelude::Insertable, Selectable};
use infrastructure::schema::{projects, users};
use serde::{Deserialize, Serialize};
use std::cmp::{Eq, Ord, PartialEq, PartialOrd};

#[derive(Queryable, Serialize, Ord, Eq, PartialEq, PartialOrd, Debug, Selectable)]
#[diesel(table_name = projects)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Project {
    pub id: i64,
    pub project_code: String,
    pub project_name: String,
    pub company_id: i64,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Deserialize)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = projects)]
pub struct NewProject {
    pub project_code: String,
    pub project_name: String,
    pub company_id: i64,
}

#[derive(Queryable, Serialize, Ord, Eq, PartialEq, PartialOrd, Debug, Selectable)]
#[diesel(table_name = users )]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserLogin {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: Option<String>,
    pub company_id: Option<i64>,
}

#[derive(Queryable, Serialize, Deserialize, Debug, Selectable)]
#[diesel(table_name = users )]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Queryable, Serialize, Ord, Eq, PartialEq, PartialOrd, Debug, Selectable)]
#[diesel(table_name = users )]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub email_verified_at: Option<NaiveDateTime>,
    pub password: String,
    pub role: Option<String>,
    pub nik: Option<String>,
    pub company_id: Option<i64>,
    pub department_id: Option<i64>,
    pub position: Option<String>,
    pub join_date: Option<NaiveDate>,
    pub exit_date: Option<NaiveDate>,
    pub photo: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub gender: Option<String>,
    pub religion: Option<String>,
    pub blood_type: Option<String>,
    pub birth_date: Option<NaiveDate>,
    pub birth_place: Option<String>,
    pub marital_status: Option<String>,
    pub emergency_contact_name: Option<String>,
    pub emergency_contact_relationship: Option<String>,
    pub emergency_contact_phone: Option<String>,
    pub emergency_contact_address: Option<String>,
    pub status: Option<String>,
    pub remember_token: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
