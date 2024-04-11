use rocket::form::Form;
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
use rocket_db_pools::{diesel::prelude::*, Connection};
use rocket_dyn_templates::{context, Template};

use crate::config::Db;
use crate::schema::users;

#[derive(Debug, Insertable, rocket::FromForm)]
#[diesel(table_name = crate::schema::users)]
pub struct UserInputable {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: String,
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
) -> Result<Flash<Redirect>, Template> {
    if !validate_email(user_input.email.clone(), &mut db).await {
        return Ok(Flash::error(
            Redirect::to("/users/new"),
            "Email already taken",
        ));
    }

    let values = diesel::insert_into(users::table)
        .values(&user_input.into_inner())
        .execute(&mut db)
        .await;

    match values {
        Ok(_) => Ok(Flash::success(
            Redirect::to("/"),
            "User created successfully",
        )),
        Err(e) => Err(Template::render(
            "error",
            context! {
                title: "Error",
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
