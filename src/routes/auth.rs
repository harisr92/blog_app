use rocket::form::Form;
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
use rocket_db_pools::{diesel::prelude::*, Connection};
use rocket_dyn_templates::{context, Template};

use crate::config::Db;

#[derive(rocket::FromForm, Debug)]
pub struct LoginInput {
    user_name: String,
    password: String,
}

#[rocket::get("/login")]
pub async fn index(flash: Option<FlashMessage<'_>>) -> Result<Template, Template> {
    Ok(Template::render(
        "auth/login",
        context! { flash: crate::helpers::flash_label(flash) },
    ))
}

#[rocket::post("/auth/login", data = "<login>")]
pub async fn create(login: Form<LoginInput>, mut db: Connection<Db>) -> Flash<Redirect> {
    let inner_form = login.into_inner();
    let password = inner_form.password;

    if let Ok(loaded_user) = crate::schema::users::table
        .filter(crate::schema::users::email.eq(inner_form.user_name))
        .select(crate::models::users::User::as_select())
        .first(&mut db)
        .await
    {
        if loaded_user.compare_password(password) {
            return Flash::success(Redirect::to("/"), "You are signed in");
        }
    };

    Flash::error(Redirect::to("/login"), "Invlaid user name or password")
}
