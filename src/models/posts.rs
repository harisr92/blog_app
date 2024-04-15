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

#[derive(Debug, rocket::FromForm)]
pub struct PostInputForm {
    title: String,
    content: String,
}

impl Post {
    pub fn build_from(form: PostInputForm, user_id: u64) -> Self {
        Post {
            id: 0,
            title: form.title,
            body: form.content,
            status: String::from(*Self::states().front().unwrap()),
            user_id,
        }
    }

    pub fn states() -> LinkedList<&'static str> {
        LinkedList::from(["Draft", "Review", "Published"])
    }
}
