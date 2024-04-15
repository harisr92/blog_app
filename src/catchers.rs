use rocket::response::{Flash, Redirect};
use rocket::Request;

#[rocket::catch(401)]
fn unauthorized(_req: &Request) -> Flash<Redirect> {
    Flash::error(Redirect::to("/login"), "Please login")
}

pub fn stage() -> Vec<rocket::Catcher> {
    rocket::catchers![unauthorized]
}
