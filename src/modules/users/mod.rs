use crate::establish_connection;

use actix_web::{delete, get, post, put, web, HttpResponse, Responder};

use rusqlite::params;
use serde::{Deserialize, Serialize};

pub fn configure_user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_user)
        .service(get_users)
        .service(create_user)
        .service(update_user)
        .service(delete_user);
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    id: Option<u32>,
    username: String,
    email: String,
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
    let conn = establish_connection().unwrap();

    let query = "SELECT id, username, email FROM users";
    let mut stmt = conn.prepare(query).unwrap();

    let mut users = Vec::new();
    let rows = stmt
        .query_map([], |row| {
            Ok(User {
                id: row.get(0)?,
                username: row.get(1)?,
                email: row.get(2)?,
            })
        })
        .unwrap();

    for user in rows {
        users.push(user.unwrap());
    }

    HttpResponse::Ok().json(users)
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
