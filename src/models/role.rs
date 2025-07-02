use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize)]
pub struct Role {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Deserialize)]
pub struct CreateRolePayload {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = roles)]
pub struct NewRole<'a> {
    pub name: &'a str,
    pub description: Option<&'a str>,
}
