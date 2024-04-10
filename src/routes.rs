pub mod posts;

pub fn build() -> Vec<rocket::Route> {
    rocket::routes![posts::index]
}
