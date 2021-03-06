mod models;
mod config;
mod handlers;
mod db;
mod errors;

use actix_web::{HttpServer, App, web};
use std::io;
use dotenv::dotenv;
use tokio_postgres::NoTls;
use crate::handlers::*;
use slog::{Logger, Drain, o, info};
use crate::models::AppState;


pub fn configure_log() -> Logger {
    let decorator = slog_term::TermDecorator::new().build();
    let console_drain = slog_term::FullFormat::new(decorator).build().fuse();
    let console_drain = slog_async::Async::new(console_drain).build().fuse();
    slog::Logger::root(console_drain, o!("v" => env!("CARGO_PKG_VERSION")))
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let config = crate::config::Config::from_env().unwrap();

    let pool = config.pg.create_pool(NoTls).unwrap();
    let log = configure_log();

    info!(log, "Starting server at http://{}:{}/", config.server.host, config.server.port);
    HttpServer::new(move || {
        App::new()
            .data(
                AppState{
                    pool: pool.clone(),
                    log: log.clone()
                }
            )
            .route("api/v1", web::get().to(status))
            .route("api/v1/todos{_:/?}", web::post().to(create_todo))
            .route("api/v1/todos{_:/?}", web::get().to(get_todos))
            .route("api/v1/todos/{list_id}/items{_:/?}", web::get().to(get_items))
            .route(
                "api/v1/todos/{list_id}/items/{item_id}{_:/?}",
                web::put().to(check_item),
            )
    })
        .bind(format!("{}:{}",config.server.host, config.server.port))?
        .run()
        .await
}
