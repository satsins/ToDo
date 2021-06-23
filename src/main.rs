mod models;
mod config;
mod handlers;

use actix_web::{HttpServer, App, web, Responder};
use std::io;
use crate::models::Status;
use dotenv::dotenv;
use tokio_postgres::NoTls;
use crate::handlers::*;


#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let config = crate::config::Config::from_env().unwrap();

    let pool = config.pg.create_pool(NoTls).unwrap();

    println!("Starting server at http://{}:{}/", config.server.host, config.server.port);
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/", web::get().to(status))
    })
        .bind(format!("{}:{}",config.server.host, config.server.port))?
        .run()
        .await
}
