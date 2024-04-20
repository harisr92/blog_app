use crate::config::Db;
use rocket::config::Config;
use rocket::fairing::AdHoc;
use rocket::figment::providers::{Env, Format, Toml};
use rocket::figment::Figment;
use rocket::tokio::sync::Barrier;
use rocket::{Build, Rocket, State};
use rocket_db_pools::Database;

#[rocket::get("/barrier")]
async fn rendezvous(barrier: &State<Barrier>) -> &'static str {
    println!("Waiting for second task...");
    barrier.wait().await;
    "Rendezvous reached."
}

pub fn rocket() -> Rocket<Build> {
    rocket::custom(rocket_config_figment())
        .mount("/", rocket::routes![rendezvous])
        .attach(Db::init())
        .attach(AdHoc::on_ignite("Add Channel", |rocket| async {
            rocket.manage(Barrier::new(2))
        }))
}

fn rocket_config_figment() -> Figment {
    Figment::from(Config::debug_default())
        .merge(Toml::file(Env::var_or("ROCKET_CONFIG", "Rocket.test.toml")).nested())
}

#[cfg(test)]
mod tests {
    use super::rocket;
    use crate::config::Db;
    use crate::models::users::User;
    use rocket::local::asynchronous::Client;
    use rocket_db_pools::diesel::prelude::*;
    use rocket_db_pools::Connection;

    async fn get_rocket_client() -> Client {
        Client::tracked(rocket()).await.unwrap()
    }

    #[test]
    fn it_builds_new_user() {
        let user = User::build(
            Some("Bruce".to_string()),
            Some("Wayne".to_string()),
            "bruce@wayne.com".to_string(),
            "hello2world!".to_string(),
        );

        assert_eq!(user.id, 0);
        assert_eq!(user.first_name, Some("Bruce".to_string()));
        assert_eq!(user.last_name, Some("Wayne".to_string()));
        assert_eq!(user.email, "bruce@wayne.com".to_string());
    }

    #[rocket::async_test]
    async fn it_finds_user_by_id() {
        let rocket_client = get_rocket_client().await;
        let req = rocket_client.get("/users/profile");
        let mut db: Connection<Db> = req.guard().await.expect("Could not connect to db");

        let u = User::build(
            Some("Bruce".to_string()),
            Some("Wayne".to_string()),
            "bruce@wayne.com".to_string(),
            "hello2world!".to_string(),
        );
        let _ = diesel::insert_into(crate::schema::users::table)
            .values(&u)
            .execute(&mut db)
            .await;
        let user = User::find_by_email(u.email, db).await.unwrap();

        let db1: Connection<Db> = req.guard().await.expect("Could not connect to db");
        let user_by_id = User::find_by_id(user.id, db1).await.unwrap();

        assert_eq!(user_by_id.email, "bruce@wayne.com".to_string());
        assert_eq!(user_by_id.id, user.id);
    }
}
