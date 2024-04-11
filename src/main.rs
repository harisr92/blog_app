use blog_web::config;
use blog_web::routes;
use rocket_db_pools::Database;
use rocket_dyn_templates::Template;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .attach(Template::fairing())
        .attach(config::Db::init())
        .mount("/", routes::build())
        .launch()
        .await?;

    Ok(())
}
