use rocket::form::Form;
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
use rocket_db_pools::{diesel::prelude::*, Connection};
use rocket_dyn_templates::{context, Template};

use crate::config::Db;
use crate::models::users::User;
use crate::schema::{posts, users};

#[derive(Debug, rocket::FromForm)]
pub struct UserInputable {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: String,
    pub password: String,
    pub confirm_password: String,
}

#[derive(Debug, rocket::FromForm)]
pub struct PasswordInput {
    pub password: String,
    pub confirm_password: String,
}

#[rocket::get("/users/new")]
pub async fn new(flash: Option<FlashMessage<'_>>) -> Result<Template, String> {
    let flash_messages = flash
        .map(|msg| format!("{}", msg.message()))
        .unwrap_or_else(|| String::from("Welcome to blog app"));

    Ok(Template::render(
        "users/new",
        context! {
            title: "New user",
            flash_message: flash_messages
        },
    ))
}

#[rocket::post("/users", data = "<user_input>")]
pub async fn create<'r>(
    mut db: Connection<Db>,
    user_input: Form<UserInputable>,
    user_session: crate::config::UserSession<'_>,
) -> Result<Flash<Redirect>, Template> {
    if !validate_email(user_input.email.clone(), &mut db).await {
        return Ok(Flash::error(
            Redirect::to("/users/new"),
            "Email already taken",
        ));
    }

    let inner = user_input.into_inner();
    if inner.password != inner.confirm_password {
        return Ok(Flash::error(
            Redirect::to("/users/new"),
            "Password does not match",
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
            flash: crate::helpers::flash_label(flash),
        },
    ))
}

#[rocket::put("/users/update-password", data = "<password>")]
pub async fn update_password(
    password: Form<PasswordInput>,
    mut user: User,
    mut db: Connection<Db>,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    user.set_password(password.into_inner().password);
    let res = diesel::update(users::table)
        .filter(users::id.eq(user.id))
        .set(user)
        .execute(&mut db)
        .await;

    match res {
        Ok(_) => Ok(Flash::success(
            Redirect::to(rocket::uri!(profile())),
            "Password updated",
        )),
        Err(e) => Err(Flash::error(
            Redirect::to(rocket::uri!(profile())),
            e.to_string(),
        )),
    }
}

#[rocket::get("/users/posts")]
pub async fn users_posts(mut db: Connection<Db>) -> Result<Template, Template> {
    let all_users = users::table
        .left_join(posts::table.on(posts::user_id.eq(users::id)))
        .select((
            crate::models::users::User::as_select(),
            Option::<crate::models::posts::Post>::as_select(),
        ))
        .distinct()
        .load::<(
            crate::models::users::User,
            Option<crate::models::posts::Post>,
        )>(&mut db)
        .await;

    match all_users {
        Ok(data) => {
            let mut grouped_data: Vec<(
                crate::models::users::User,
                Vec<Option<crate::models::posts::Post>>,
            )> = Vec::new();

            for user in data {
                if let Some(with_user) = grouped_data.iter_mut().find(|u| u.0 == user.0) {
                    with_user.1.push(user.1)
                } else {
                    grouped_data.push((user.0, vec![user.1]));
                };
            }

            Ok(Template::render(
                "users/index",
                context! {
                    title: "Blogs",
                    users: grouped_data,
                },
            ))
        }
        Err(e) => Err(Template::render(
            "error",
            context! {
                title: "error",
                error: e.to_string(),
            },
        )),
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
    rocket::routes![new, create, profile, update_password, users_posts]
}
