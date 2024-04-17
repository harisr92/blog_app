use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::LinkedList;

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
    Insertable,
    AsChangeset,
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

#[derive(Debug, Clone, Queryable, Serialize, Deserialize)]
pub struct PostQueryable {
    pub id: u64,
    pub title: String,
    pub body: String,
    pub status: String,
    pub user_id: u64,
    #[serde(serialize_with = "serialize_dt", alias = "created_date")]
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, rocket::FromForm, Serialize, Deserialize)]
pub struct PostInputForm {
    title: String,
    content: String,
}

pub fn serialize_dt<S>(dt: &chrono::NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let res = dt.format("%d %B").to_string();
    res.serialize(serializer)
}

impl Post {
    pub fn build_from(form: &PostInputForm, user_id: u64) -> Self {
        Post {
            id: 0,
            title: form.title.clone(),
            body: form.content.clone(),
            status: String::from(*Self::states().front().unwrap()),
            user_id,
        }
    }

    pub fn states() -> LinkedList<&'static str> {
        LinkedList::from(["Draft", "Review", "Published"])
    }
}

impl PostInputForm {
    pub fn new(title: String, content: String) -> Self {
        PostInputForm { title, content }
    }
}
