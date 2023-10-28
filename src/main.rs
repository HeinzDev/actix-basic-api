use actix_web::middleware::Logger;
use actix_web::{delete, get, post, put, web, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;
use log::info;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    id: Option<u32>,
    username: String,
    email: String,
}

#[get("/users/{id}")]
async fn get_user(id: web::Path<u32>) -> impl Responder {
    let id = id.into_inner();

    if let Ok(conn) = Connection::open("user_database.db") {
        if let Ok(mut stmt) = conn.prepare("SELECT username, email FROM users WHERE id = ?1") {
            if let Ok(user) = stmt.query_row(params![id], |row| {
                let user = User {
                    id: Some(id),
                    username: row.get(0).unwrap_or_default(),
                    email: row.get(1).unwrap_or_default(),
                };
                Ok(user)
            }) {
                return HttpResponse::Ok().json(user);
            }
        }
    }

    HttpResponse::InternalServerError().finish()
}

#[post("/users")]
async fn create_user(user: web::Json<User>) -> impl Responder {
    if let Ok(conn) = Connection::open("user_database.db") {
        if let Ok(_) = conn.execute(
            "INSERT INTO users (username, email) VALUES (?1, ?2)",
            params![user.username.clone(), user.email.clone()],
        ) {
            let id = conn.last_insert_rowid() as u32;
            let created_user = User {
                id: Some(id),
                username: user.username.clone(),
                email: user.email.clone(),
            };
            return HttpResponse::Created().json(created_user);
        }
    }

    HttpResponse::InternalServerError().finish()
}

#[put("/users/{id}")]
async fn update_user(id: web::Path<u32>, user: web::Json<User>) -> impl Responder {
    let id = id.into_inner();

    if let Ok(conn) = Connection::open("user_database.db") {
        if let Ok(_) = conn.execute(
            "UPDATE users SET username = ?1, email = ?2 WHERE id = ?3",
            params![user.username.clone(), user.email.clone(), id],
        ) {
            return HttpResponse::Ok().json(user.0);
        }
    }

    HttpResponse::InternalServerError().finish()
}

#[delete("/users/{id}")]
async fn delete_user(id: web::Path<u32>) -> impl Responder {
    let id = id.into_inner();

    if let Ok(conn) = Connection::open("user_database.db") {
        if let Ok(_) = conn.execute("DELETE FROM users WHERE id = ?1", params![id]) {
            return HttpResponse::NoContent().finish();
        }
    }

    HttpResponse::InternalServerError().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let server = HttpServer::new(|| {
        App::new()
            .wrap(Logger::default()) // Adicione um logger
            .service(get_user)
            .service(create_user)
            .service(update_user)
            .service(delete_user)
    });

    let address = "127.0.0.1:8080";

    let server = server.bind(address)?;

    info!("API started on 8080 port");
    //println!("A API started on: {}", address); //use this to more simple log
    server.run().await?;

    Ok(())
}
