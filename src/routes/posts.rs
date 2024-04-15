use rocket::form::Form;
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
use rocket_db_pools::{diesel::prelude::*, Connection};
use rocket_dyn_templates::{context, Template};

use crate::config::Db;
use crate::models;
use crate::schema::posts::{self, dsl::*};

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

#[rocket::get("/posts/new")]
pub async fn new(user: crate::models::users::User) -> Result<Template, &'static str> {
    println!("{:?}", user);

    Ok(Template::render(
        "posts/new",
        context! {
            title: "Post"
        },
    ))
}

#[rocket::post("/posts", data = "<post_input>")]
pub async fn create(
    user: models::users::User,
    post_input: Form<models::posts::PostInputForm>,
    mut db: Connection<Db>,
) -> Result<Flash<Redirect>, Flash<Template>> {
    let post = models::posts::Post::build_from(post_input.into_inner(), user.id);

    let values = diesel::insert_into(posts::table)
        .values(&post)
        .execute(&mut db)
        .await;

    match values {
        Ok(vals) => {
            println!("{:?}", vals);
            Ok(Flash::success(
                Redirect::to("/"),
                "Post created successfully",
            ))
        }
        Err(_) => Err(Flash::error(
            Template::render(
                "/posts/new",
                context! {
                    title: "New Post",
                    post: post,
                },
            ),
            "Failed to create post",
        )),
    }
}
