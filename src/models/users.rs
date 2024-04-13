use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, Queryable, Selectable, Insertable, Serialize, Deserialize, Identifiable, PartialEq,
)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct User {
    pub id: u64,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: String,
}
