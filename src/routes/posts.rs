// use diesel::{SelectableHelper, QueryDsl};
// use diesel::prelude::*;
use rocket_dyn_templates::{context, Template};
use rocket_db_pools::{Connection, diesel::prelude::*};

use crate::schema::posts::dsl::*;
use crate::config::Db;

#[rocket::get("/")]
pub async fn index(mut db: Connection<Db>) -> Result<Template, String> {
    let posts_result = posts.select(crate::models::posts::Post::as_select())
                         .load(&mut db).await;

    match posts_result {
        Ok(all_posts) => {
            Ok(Template::render("index", context! {
                title: "Posts",
                posts: all_posts,
            }))
        }
        Err(e) => {
            Ok(Template::render("error", context! {
                title: "Error",
                error: e.to_string(),
            }))
        }
    }

}
