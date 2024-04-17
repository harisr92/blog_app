use rocket::response::{Flash, Redirect};
use rocket::Request;
use rocket_dyn_templates::{context, Template};

#[rocket::catch(401)]
fn unauthorized(_req: &Request) -> Flash<Redirect> {
    Flash::error(Redirect::to("/login"), "Please login")
}

#[rocket::catch(404)]
fn not_found(_req: &Request) -> Flash<Template> {
    Flash::error(
        Template::render(
            "errors/not_found",
            context! {
                title: "Not Found",
            },
        ),
        "Route not found",
    )
}

#[rocket::catch(500)]
fn server_error(_req: &Request) -> Template {
    Template::render(
        "errors/internal_server_error",
        context! {
            title: "Server error",
        },
    )
}

pub fn stage() -> Vec<rocket::Catcher> {
    rocket::catchers![unauthorized, not_found, server_error]
}
