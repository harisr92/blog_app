use rocket_dyn_templates::Template;
use blog_web::routes;
use rocket_db_pools::Database;
use blog_web::config;

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
