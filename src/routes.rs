pub mod auth;
pub mod posts;
pub mod users;

pub fn build() -> Vec<rocket::Route> {
    [auth::stage(), users::stage(), posts::stage()].concat()
}
