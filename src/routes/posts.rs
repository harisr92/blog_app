use rocket::request::FlashMessage;
use rocket_db_pools::{diesel::prelude::*, Connection};
use rocket_dyn_templates::{context, Template};

use crate::config::Db;
use crate::schema::posts::dsl::*;

#[rocket::get("/")]
pub async fn index(
    mut db: Connection<Db>,
    flash: Option<FlashMessage<'_>>,
) -> Result<Template, String> {
    let posts_result = posts
        .select(crate::models::posts::Post::as_select())
        .load(&mut db)
        .await;

    match posts_result {
        Ok(all_posts) => Ok(Template::render(
            "index",
            context! {
                title: "Posts",
                posts: all_posts,
                flash: crate::helpers::flash_label(flash),
            },
        )),
        Err(_) => Err("Something went wrong".to_string()),
    }
}
