use rocket::response::{Flash, Redirect};
use rocket::Request;

#[rocket::catch(401)]
fn unauthorized(_req: &Request) -> Flash<Redirect> {
    Flash::error(Redirect::to("/login"), "Please login")
}

#[rocket::catch(404)]
fn not_found(_req: &Request) -> Flash<Redirect> {
    Flash::error(Redirect::to("/"), "Route not found")
}

pub fn stage() -> Vec<rocket::Catcher> {
    rocket::catchers![unauthorized, not_found]
}
