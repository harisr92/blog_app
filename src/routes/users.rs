use rocket::form::Form;
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
use rocket_db_pools::{diesel::prelude::*, Connection};
use rocket_dyn_templates::{context, Template};

use crate::config::Db;
use crate::models::users::User;
use crate::schema::users;
use serde::Serialize;

#[derive(Debug, rocket::FromForm, Serialize)]
pub struct UserInputable {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: String,
    pub password: String,
    pub confirm_password: String,
}

#[derive(Debug, rocket::FromForm, Serialize)]
pub struct PasswordInput {
    pub password: String,
    pub confirm_password: String,
}

#[rocket::get("/users/new")]
pub async fn new(
    flash: Option<FlashMessage<'_>>,
    user: Option<User>,
) -> Result<Template, Flash<Redirect>> {
    if let Some(_) = user {
        return Err(Flash::error(
            Redirect::to("/users/profile"),
            "Please logout before signing up",
        ));
    }

    Ok(Template::render(
        "users/new",
        context! {
            title: "New user",
            flash: crate::helpers::flash_label(&flash),
            is_signedin: false,
        },
    ))
}

#[rocket::post("/users", data = "<user_input>")]
pub async fn create<'r>(
    mut db: Connection<Db>,
    user_input: Form<UserInputable>,
    user_session: crate::config::UserSession<'_>,
) -> Result<Flash<Redirect>, Template> {
    let inner = user_input.into_inner();

    if !validate_email(inner.email.clone(), &mut db).await {
        return Err(Template::render(
            "users/new",
            context! {
                title: "New user",
                user: inner,
                flash: crate::helpers::FlashLabel::error("Invalid email"),
                is_signedin: false,
            },
        ));
    }

    if inner.password != inner.confirm_password {
        return Err(Template::render(
            "users/new",
            context! {
                title: "New user",
                user: inner,
                flash: crate::helpers::FlashLabel::error("Password must match"),
                is_signedin: false,
            },
        ));
    }

    let u = crate::models::users::User::build(
        inner.first_name,
        inner.last_name,
        inner.email,
        inner.password,
    );

    let values = diesel::insert_into(users::table)
        .values(&u)
        .execute(&mut db)
        .await;

    match values {
        Ok(_) => {
            if let Some(user) = User::find_by_email(u.email, db).await {
                user_session.signin(&user);
                Ok(Flash::success(
                    Redirect::to(rocket::uri!(profile())),
                    "User created successfully",
                ))
            } else {
                Ok(Flash::success(
                    Redirect::to("/"),
                    "User created successfully",
                ))
            }
        }
        Err(e) => Err(Template::render(
            "error",
            context! {
                title: "Error",
                error: e.to_string(),
            },
        )),
    }
}

#[rocket::get("/users/profile")]
pub async fn profile<'r>(
    flash: Option<FlashMessage<'r>>,
    user: User,
) -> Result<Template, &'static str> {
    Ok(Template::render(
        "users/profile",
        context! {
            title: "Profile",
            user: user,
            is_signedin: true,
            flash: crate::helpers::flash_label(&flash),
        },
    ))
}

#[rocket::put("/users/update-password", data = "<password>")]
pub async fn update_password(
    password: Form<PasswordInput>,
    mut user: User,
    mut db: Connection<Db>,
) -> Result<Flash<Redirect>, Template> {
    if password.password != password.confirm_password {
        return Err(Template::render(
            "users/profile",
            context! {
                title: "Profile",
                user: user,
                flash: crate::helpers::FlashLabel::error("Password must match"),
                is_signedin: true,
            },
        ));
    }

    user.set_password(password.into_inner().password);
    let res = diesel::update(users::table)
        .filter(users::id.eq(user.id))
        .set(user.clone())
        .execute(&mut db)
        .await;

    match res {
        Ok(_) => Ok(Flash::success(
            Redirect::to(rocket::uri!(profile())),
            "Password updated",
        )),
        Err(e) => {
            return Err(Template::render(
                "users/profile",
                context! {
                    title: "Profile",
                    user: user,
                    flash: crate::helpers::FlashLabel::error(
                        &format!("Something went wrong : {}", e.to_string())[..]
                    ),
                    is_signedin: true,
                },
            ))
        }
    }
}

async fn validate_email(email: String, db: &mut Connection<Db>) -> bool {
    let res = users::table
        .filter(users::email.eq(email))
        .select(crate::models::users::User::as_select())
        .first(db)
        .await;

    match res {
        Ok(_) => false,
        Err(_) => true,
    }
}

pub fn stage() -> Vec<rocket::Route> {
    rocket::routes![new, create, profile, update_password]
}
