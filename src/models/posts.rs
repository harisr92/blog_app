use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Clone,
    Queryable,
    Selectable,
    Serialize,
    Deserialize,
    Identifiable,
    Associations,
    PartialEq,
)]
#[serde(crate = "rocket::serde")]
#[diesel(belongs_to(crate::models::users::User))]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Post {
    pub id: u64,
    pub title: String,
    pub body: String,
    pub status: String,
    pub user_id: u64,
}
