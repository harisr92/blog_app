pub mod auth;
pub mod posts;
pub mod users;

pub fn build() -> Vec<rocket::Route> {
    rocket::routes![
        posts::index,
        posts::new,
        posts::create,
        users::new,
        users::create,
        users::profile,
        users::update_password,
        users::users_posts,
        auth::index,
        auth::create,
        auth::delete,
    ]
}
