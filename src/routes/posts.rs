use rocket::form::Form;
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
use rocket_db_pools::diesel;
use rocket_db_pools::{diesel::prelude::*, Connection};
use rocket_dyn_templates::{context, Template};

use crate::config::Db;
use crate::models;
use crate::schema::posts;

#[rocket::get("/")]
pub async fn index(
    mut db: Connection<Db>,
    flash: Option<FlashMessage<'_>>,
    user: Option<models::users::User>,
) -> Result<Template, String> {
    let mut is_signedin: bool = false;
    if let Some(_) = user {
        is_signedin = true
    }

    let posts_result = posts::table
        .filter(posts::status.eq("Published"))
        .load::<models::posts::PostQueryable>(&mut db)
        .await;

    match posts_result {
        Ok(all_posts) => Ok(Template::render(
            "posts/index",
            context! {
                title: "Posts",
                posts: all_posts,
                is_signedin,
                flash: crate::helpers::flash_label(&flash),
            },
        )),
        Err(_) => Err("Something went wrong".to_string()),
    }
}

#[rocket::get("/posts/new")]
pub async fn new(_user: crate::models::users::User) -> Result<Template, &'static str> {
    Ok(Template::render(
        "posts/new",
        context! {
            title: "Post",
            is_signedin: true,
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
        Ok(_) => Ok(Flash::success(
            Redirect::to("/"),
            "Post created successfully",
        )),
        Err(_) => Err(Flash::error(
            Template::render(
                "/posts/new",
                context! {
                    title: "New Post",
                    post: post,
                    is_signedin: true,
                },
            ),
            "Failed to create post",
        )),
    }
}

#[rocket::get("/posts/mine")]
pub async fn my_posts(
    user: models::users::User,
    mut db: Connection<Db>,
) -> Result<Template, String> {
    let result = posts::table
        .filter(posts::user_id.eq(user.id))
        .load::<models::posts::PostQueryable>(&mut db)
        .await;

    match result {
        Ok(posts) => Ok(Template::render(
            "posts/index",
            context! {
                title: "Posts",
                posts: posts,
                is_signedin: true,
            },
        )),
        Err(e) => Err(e.to_string()),
    }
}

#[rocket::delete("/posts/<id>")]
async fn delete(
    id: u64,
    user: models::users::User,
    mut db: Connection<Db>,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    if let Ok(_) = diesel::delete(posts::table)
        .filter(posts::user_id.eq(user.id).and(posts::id.eq(id)))
        .execute(&mut db)
        .await
    {
        Ok(Flash::success(
            Redirect::to(rocket::uri!(my_posts())),
            "Post deleted successfully",
        ))
    } else {
        Err(Flash::error(
            Redirect::to(rocket::uri!(my_posts())),
            "Could not delete post",
        ))
    }
}

pub fn stage() -> Vec<rocket::Route> {
    rocket::routes![index, new, create, my_posts, delete]
}
