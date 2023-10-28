use actix_web::middleware::Logger;
use actix_web::{get, App, HttpResponse, HttpServer};
use env_logger::Env;
use rusqlite::Connection;
use std::fs;

mod modules;
use crate::modules::establish_connection;

#[get("/")]
async fn index() -> HttpResponse {
    let html = r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Actix API</title>
        </head>
        <body>
            <h1>Welcome to Actix API!</h1>
        </body>
        </html>
    "#;

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info,actix_web=info"))
        .format_timestamp_millis()
        .init();

    if !fs::metadata("user_database.db").is_ok() {
        let conn = Connection::open("user_database.db").expect("Erro ao criar banco de dados.");
        conn.execute(
            "CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY, username TEXT, email TEXT)",
            [],
        )
        .expect("Erro ao criar tabela users.");
    }

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .configure(modules::users::configure_user_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
