#[path = "./handlers/mod.rs"]
mod handlers;
#[path = "./model/mod.rs"]
mod model;
#[path = "./dal/mod.rs"]
mod dal;
mod errors;
mod routes;
mod state;

use actix_web::{web, App, HttpServer, http};
use actix_cors::Cors;
use std::io;
use routes::*;
use state::AppState;
use std::env;
use dotenv::dotenv;
use sqlx::postgres::{PgPoolOptions};

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL is not set in .env file");

    let db_pool = match PgPoolOptions::new()
        .idle_timeout(std::time::Duration::from_secs(30))
        .max_connections(20)
        .connect(&database_url)
        .await
    {
        Ok(db_pool) => {
            println!("✅Connection to the database is successful!");
            db_pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    let shared_data = web::Data::new(AppState {
        probe: "Probe test ok....".to_string(),
        db: db_pool,
    });

    let app = move || {
//The following section applies only to the development mode of React. Error handling in production mode is done with regular try/catch statements.
//https://reactjs.org/docs/cross-origin-errors.html
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "PUT", "PATCH", "DELETE"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);
        App::new()
            .wrap(cors)
            .app_data(shared_data.clone())
            .configure(general_routes)
            .configure(book_routes)
    };

    let hostname_port = env::var("SERVER_HOSTNAME_PORT").expect("SERVER_HOSTNAME_PORT is not set in .env file");
    println!("Http Server running on host:port = {:?}", hostname_port);
    HttpServer::new(app)
        .bind(hostname_port)?
        .run()
        .await
}