use rocket::form::Form;
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
use rocket_db_pools::diesel;
use rocket_db_pools::{diesel::prelude::*, Connection};
use rocket_dyn_templates::{context, Template};

use crate::config::Db;
use crate::models;
use crate::schema::{posts, users};

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
    let post_input = models::posts::PostInputForm::new("".to_string(), "".to_string());

    Ok(Template::render(
        "posts/new",
        context! {
            title: "Post",
            is_signedin: true,
            post_input
        },
    ))
}

#[rocket::post("/posts", data = "<post_input>")]
pub async fn create(
    user: models::users::User,
    post_input: rocket::form::Result<'_, Form<models::posts::PostInputForm>>,
    mut db: Connection<Db>,
) -> Result<Flash<Redirect>, Template> {
    if let Err(e) = post_input {
        let err_messages = e
            .iter()
            .map(|v_err| v_err.to_string())
            .collect::<Vec<_>>()
            .join(" ");

        let post_empty = models::posts::PostInputForm::new("".to_string(), "".to_string());
        return Err(Template::render(
            "posts/new",
            context! {
                title: "New Post",
                is_signedin: true,
                post_input: post_empty,
                flash: crate::helpers::FlashLabel::error(
                    &format!("Failed to create post: {}", err_messages)[..]
                ),
            },
        ));
    }
    let input = post_input.ok().unwrap().into_inner();
    let post = models::posts::Post::build_from(&input, user.id);

    let values = diesel::insert_into(posts::table)
        .values(&post)
        .execute(&mut db)
        .await;

    match values {
        Ok(_) => Ok(Flash::success(
            Redirect::to("/"),
            "Post created successfully",
        )),
        Err(e) => Err(Template::render(
            "posts/new",
            context! {
                title: "New Post",
                post_input: input,
                is_signedin: true,
                flash: crate::helpers::FlashLabel::error(
                    &format!("Failed to create post: {}", e.to_string())[..]
                ),
            },
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
        .order(posts::created_at.desc())
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

#[rocket::get("/posts/<id>")]
async fn show(
    id: u64,
    user: Option<models::users::User>,
    mut db: Connection<Db>,
) -> Result<Template, &'static str> {
    let mut is_signedin = false;
    let mut can_edit = false;

    if let Ok(post) = posts::table
        .left_join(users::table.on(users::id.eq(posts::user_id)))
        .filter(posts::id.eq(id))
        .first::<(models::posts::PostQueryable, Option<models::users::User>)>(&mut db)
        .await
    {
        if let Some(u) = user {
            is_signedin = true;
            if let Some(pu) = &post.1 {
                can_edit = u.id == pu.id;
            }
        }

        Ok(Template::render(
            "posts/show",
            context! {
                title: "Post",
                post: post.0,
                post_user: post.1,
                is_signedin,
                can_edit,
            },
        ))
    } else {
        Err("Something went wrong")
    }
}

#[rocket::get("/posts/<id>/edit")]
async fn edit(
    id: u64,
    user: models::users::User,
    mut db: Connection<Db>,
) -> Result<Template, &'static str> {
    if let Ok(post) = posts::table
        .filter(posts::id.eq(id).and(posts::user_id.eq(user.id)))
        .first::<models::posts::PostQueryable>(&mut db)
        .await
    {
        Ok(Template::render(
            "posts/edit",
            context! {
                title: "Edit post",
                post: post,
                is_signedin: true,
            },
        ))
    } else {
        Err("Something went wrong")
    }
}

#[rocket::put("/posts/<id>", data = "<input>")]
pub async fn update(
    id: u64,
    input: Form<models::posts::PostInputForm>,
    user: models::users::User,
    mut db: Connection<Db>,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let post = models::posts::Post::build_from(&input, user.id);
    let res = diesel::update(posts::table)
        .filter(posts::id.eq(id).and(posts::user_id.eq(user.id)))
        .set(post)
        .execute(&mut db)
        .await;

    match res {
        Ok(_) => Ok(Flash::success(
            Redirect::to(rocket::uri!(show(id))),
            "Article updated successfully",
        )),
        Err(e) => Err(Flash::error(
            Redirect::to(rocket::uri!(show(id))),
            e.to_string(),
        )),
    }
}

#[rocket::put("/posts/<id>/publish")]
pub async fn publish(
    id: u64,
    user: models::users::User,
    mut db: Connection<Db>,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let mut post: models::posts::Post = posts::table
        .filter(posts::id.eq(id).and(posts::user_id.eq(user.id)))
        .select(models::posts::Post::as_select())
        .first(&mut db)
        .await
        .ok()
        .expect("Could not find post");

    post.publish();
    let res = diesel::update(posts::table)
        .filter(posts::id.eq(post.id))
        .set(posts::status.eq(post.status))
        .execute(&mut db)
        .await;

    match res {
        Ok(_) => Ok(Flash::success(
            Redirect::to(rocket::uri!(show(id))),
            "Article updated successfully",
        )),
        Err(e) => Err(Flash::error(
            Redirect::to(rocket::uri!(show(id))),
            e.to_string(),
        )),
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
    rocket::routes![index, new, create, my_posts, show, edit, update, publish, delete]
}
