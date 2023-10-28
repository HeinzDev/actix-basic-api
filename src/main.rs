use actix_web::middleware::Logger;
use actix_web::{delete, get, post, put, web, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;
use log::info;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    id: Option<u32>,
    username: String,
    email: String,
}

fn establish_connection() -> rusqlite::Result<Connection> {
    Connection::open("user_database.db")
}

#[get("/users/{id}")]
async fn get_user(id: web::Path<u32>) -> impl Responder {
    let id = id.into_inner();

    if let Ok(conn) = establish_connection() {
        if let Ok(user) = conn.query_row(
            "SELECT id, username, email FROM users WHERE id = ?1",
            params![id],
            |row| {
                Ok(User {
                    id: Some(row.get(0)?),
                    username: row.get(1)?,
                    email: row.get(2)?,
                })
            },
        ) {
            return HttpResponse::Ok().json(user);
        }
    }

    HttpResponse::InternalServerError().finish()
}

#[get("/users")]
async fn get_users() -> impl Responder {
    if let Ok(conn) = establish_connection() {
        let mut stmt = conn
            .prepare("SELECT id, username, email FROM users")
            .expect("Failed to prepare query");

        let users: rusqlite::Result<Vec<User>> = stmt
            .query_map([], |row| {
                Ok(User {
                    id: Some(row.get(0)?),
                    username: row.get(1)?,
                    email: row.get(2)?,
                })
            })
            .map(|result| {
                result
                    .collect::<Result<Vec<User>, rusqlite::Error>>()
                    .unwrap_or_else(|_| vec![])
            });

        if let Ok(users) = users {
            return HttpResponse::Ok().json(users);
        }
    }

    HttpResponse::InternalServerError().finish()
}

#[post("/users")]
async fn create_user(user: web::Json<User>) -> impl Responder {
    if let Ok(conn) = establish_connection() {
        if let Ok(_) = conn.execute(
            "INSERT INTO users (username, email) VALUES (?, ?)",
            params![user.username, user.email],
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

    if let Ok(conn) = establish_connection() {
        if let Ok(_) = conn.execute(
            "UPDATE users SET username = ?, email = ? WHERE id = ?",
            params![user.username, user.email, id],
        ) {
            return HttpResponse::Ok().json(user.0);
        }
    }

    HttpResponse::InternalServerError().finish()
}

#[delete("/users/{id}")]
async fn delete_user(id: web::Path<u32>) -> impl Responder {
    let id = id.into_inner();

    if let Ok(conn) = establish_connection() {
        if let Ok(_) = conn.execute("DELETE FROM users WHERE id = ?", params![id]) {
            return HttpResponse::NoContent().finish();
        }
    }

    HttpResponse::InternalServerError().finish()
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
            .service(get_user)
            .service(get_users)
            .service(create_user)
            .service(update_user)
            .service(delete_user)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
