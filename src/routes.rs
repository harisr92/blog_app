pub mod posts;
pub mod users;

pub fn build() -> Vec<rocket::Route> {
    rocket::routes![posts::index, users::new, users::create, users::users_posts]
}
