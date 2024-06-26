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
pub async fn index(
    flash: Option<FlashMessage<'_>>,
    user: Option<crate::models::users::User>,
) -> Result<Template, Flash<Redirect>> {
    if let Some(_) = user {
        return Err(Flash::error(Redirect::to("/"), "You are already logged in"));
    };

    Ok(Template::render(
        "auth/login",
        context! {
            title: "Login",
            flash: crate::helpers::flash_label(&flash),
            is_signedin: false
        },
    ))
}

#[rocket::post("/auth/login", data = "<login>")]
pub async fn create<'r>(
    login: Form<LoginInput>,
    mut db: Connection<Db>,
    user_session: crate::config::UserSession<'_>,
) -> Flash<Redirect> {
    let inner_form = login.into_inner();
    let password = inner_form.password;

    if let Ok(loaded_user) = crate::schema::users::table
        .filter(crate::schema::users::email.eq(inner_form.user_name))
        .select(crate::models::users::User::as_select())
        .first(&mut db)
        .await
    {
        if loaded_user.compare_password(password) {
            user_session.signin(&loaded_user);
            return Flash::success(Redirect::to("/"), "You are signed in");
        }
    };

    Flash::error(Redirect::to("/login"), "Invlaid user name or password")
}

#[rocket::get("/auth/logout")]
pub async fn delete(
    user_session: crate::config::UserSession<'_>,
) -> Result<Redirect, &'static str> {
    let _ = user_session.signout();

    Ok(Redirect::to("/"))
}

pub fn stage() -> Vec<rocket::Route> {
    rocket::routes![index, create, delete]
}
