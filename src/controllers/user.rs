use crate::models::user::{User, CreateUserSchema, UpdateUserSchema};
use crate::AppState;

use actix_web::{get, post, put, web, HttpResponse, Responder, delete};
use chrono::Utc;
use serde_json::json;

#[get("")]
pub async fn get_users(data: web::Data<AppState>) -> impl Responder {
    let query_result = sqlx::query_as!(
        User,
        "SELECT * FROM m_user"
    )
    .fetch_all(&data.db)
    .await;

    if query_result.is_err() {
        let message: &str = "Something bad happened while fetching the users";
        return HttpResponse::InternalServerError()
            .json(json!({"status": "error", "message": message}))
    }

    let users = query_result.unwrap();

    HttpResponse::Ok().json(json!({
        "status": "success",
        "no. users": users.len(),
        "users": users
    }))
}

#[post("/user")]
async fn create_user(body: web::Json<CreateUserSchema>, data: web::Data<AppState>) -> impl Responder {
    let query_result = sqlx::query_as!(
        User,
        "INSERT INTO m_user (username, email, age, password) VALUES ($1, $2, $3, $4) RETURNING *",
        body.username,
        body.email,
        body.age,
        body.password
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(user) => {
            let user_response = serde_json::json!({"status": "success", "data": serde_json::json!({
                "user": user,
            })});
            return HttpResponse::Ok().json(user_response);
        },
        Err(e) => {
            if e.to_string().contains("duplicate key value violates unique constraint") {
                return HttpResponse::BadRequest()
                    .json(serde_json::json!({"status": "fail", "message": "Duplicate Key"}));
            }
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error", "message": format!("{:?}", e)}));
        }
    }
}

#[get("/user/{id}")]
async fn get_user_by_id(path: web::Path<uuid::Uuid>, data: web::Data<AppState>) -> impl Responder {
    let user_id = path.into_inner();
    let query_result = sqlx::query_as!(User, "SELECT * FROM m_user WHERE id = $1", user_id)
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(user) => {
            let user_response = serde_json::json!({"status": "success", "data": serde_json::json!({
                "user": user
            })});
            return HttpResponse::Ok().json(user_response);
        },
        Err(_) => {
            let message = format!("Note with ID: {} not found", user_id);
            return HttpResponse::NotFound()
                .json(serde_json::json!({"status": "fail", "message": message}));
        }
    }
}

#[put("/user/{id}")]
async fn update_user(path: web::Path<uuid::Uuid>, data: web::Data<AppState>, body: web::Json<UpdateUserSchema>) -> impl Responder {
    let user_id = path.into_inner();
    // make sure user exists before updating
    let query_result = sqlx::query_as!(User, "SELECT * FROM m_user WHERE id = $1", user_id)
        .fetch_one(&data.db)
        .await;

    if query_result.is_err() {
        let message = format!("User with ID: {} not found", user_id);
        return HttpResponse::NotFound()
            .json(serde_json::json!({"status": "fail", "message": message}));
    }

    let now = Utc::now();
    let user = query_result.unwrap();

    let query_result = sqlx::query_as!(
        User,
        "UPDATE m_user SET username = $1, email = $2, age = $3, password = $4, updated_at = $5 WHERE id = $6 RETURNING *",
        body.username.to_owned().unwrap_or(user.username),
        body.email.to_owned().unwrap_or(user.email),
        body.age.to_owned().unwrap_or(user.age),
        body.password.to_owned().unwrap_or(user.password),
        now,
        user_id
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(user) => {
            let user_response = serde_json::json!({"status": "success", "data": serde_json::json!({
                "user": user
            })});
            return HttpResponse::Ok().json(user_response);
        },
        Err(_) => {
            let message = format!("User with ID: {} not found", user_id);
            return HttpResponse::NotFound()
                .json(serde_json::json!({"status": "fail", "message": message}));
        }
    }
}

#[delete("/user/{id}")]
async fn delete_user(path: web::Path<uuid::Uuid>, data: web::Data<AppState>) -> impl Responder {
    let user_id = path.into_inner();
    let rows_affected = sqlx::query!("DELETE FROM m_user WHERE id = $1", user_id)
        .execute(&data.db)
        .await
        .unwrap()
        .rows_affected();

        if rows_affected == 0 {
            let message = format!("User with ID: {} not found", user_id);
            return HttpResponse::NotFound()
                .json(serde_json::json!({"status": "fail", "message": message}));
        }
        HttpResponse::NoContent().finish()
}