use blog_web::config;
use blog_web::routes;
use rocket::fs::{relative, FileServer};
use rocket_db_pools::Database;
use rocket_dyn_templates::Template;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let db = config::Db::init();

    let _rocket = rocket::build()
        .attach(Template::fairing())
        .attach(db)
        .register("/", blog_web::catchers::stage())
        .mount("/", routes::build())
        .mount("/public", FileServer::from(relative!("public")).rank(-1))
        .launch()
        .await?;

    Ok(())
}
