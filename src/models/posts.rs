use diesel::deserialize::{self, Queryable};
use diesel::prelude::*;
use scraper::element_ref::ElementRef;
use scraper::html::Html;
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
    #[diesel(column_name = body)]
    pub truncated_content: String,
    pub status: String,
    pub user_id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostQueryable {
    pub id: u64,
    pub title: String,
    pub body: String,
    pub truncated_content: String,
    pub status: String,
    pub user_id: u64,
    #[serde(serialize_with = "serialize_dt")]
    pub created_date: chrono::NaiveDateTime,
    pub created_at: chrono::NaiveDateTime,
    #[serde(serialize_with = "serialize_dt")]
    pub updated_date: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

type DB = diesel::mysql::Mysql;
impl Queryable<crate::schema::posts::SqlType, DB> for PostQueryable {
    type Row = (
        u64,
        String,
        String,
        String,
        u64,
        chrono::NaiveDateTime,
        chrono::NaiveDateTime,
    );

    fn build(row: Self::Row) -> deserialize::Result<Self> {
        Ok(PostQueryable {
            id: row.0,
            title: row.1,
            body: row.2.clone(),
            truncated_content: sanitize_body(row.2),
            status: row.3,
            user_id: row.4,
            created_at: row.5,
            updated_at: row.6,
            created_date: row.5,
            updated_date: row.6,
        })
    }
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
            status: String::from("Draft"),
            truncated_content: "".to_string(),
            user_id,
        }
    }

    pub fn current_state(&self) -> &String {
        &self.status
    }

    pub fn move_to(&mut self, new_state: String) {
        if Self::is_valid_state(&new_state) {
            self.status = new_state;
        } else {
            panic!("Invalid state");
        }
    }

    pub fn publish(&mut self) {
        self.move_to("Published".to_string());
    }

    fn is_valid_state(state: &String) -> bool {
        Self::valid_states().contains(&&state[..])
    }

    fn valid_states() -> Vec<&'static str> {
        vec!["Draft", "Review", "Published"]
    }
}

impl PostInputForm {
    pub fn new(title: String, content: String) -> Self {
        PostInputForm { title, content }
    }
}

pub fn sanitize_body(content: String) -> String {
    let doc = Html::parse_fragment(&content[..]);

    let doc_text = doc
        .tree
        .nodes()
        .filter_map(|child| ElementRef::wrap(child))
        .flat_map(|el| el.text())
        .collect::<Vec<_>>()
        .join(" ");

    doc_text[..50].to_string()
}
