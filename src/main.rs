use actix_web::{ App, web, HttpServer};
pub mod handlers;
pub mod models;
use dotenv::dotenv;
use std::env;
use sea_orm::Database;
use crate::handlers::default_handler::hello;
use crate::handlers::default_handler::echo;
use crate::handlers::contact_handler::contacts_index;
use crate::handlers::contact_handler::contacts_show;
use crate::handlers::contact_handler::contacts_store;
use crate::handlers::contact_handler::contacts_update;
use crate::handlers::contact_handler::contacts_destroy;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = Database::connect(&database_url).await.unwrap();
    let pool = web::Data::new(conn);

    HttpServer::new( move || {
        App::new()
        .app_data(pool.clone())
            // Default routes
            .service(hello)
            .service(echo)
            // Contact routes
            .service(contacts_index)
            .service(contacts_show)
            .service(contacts_store)
            .service(contacts_update)
            .service(contacts_destroy)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
