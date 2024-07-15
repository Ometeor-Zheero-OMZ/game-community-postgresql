use crate::models::game::{GameModel, CreateGameSchema, UpdateGameSchema};
use crate::AppState;

use actix_web::{get, post, put, web, HttpResponse, Responder, delete};
use chrono::Utc;
use serde_json::json;

#[get("")]
pub async fn get_games(data: web::Data<AppState>) -> impl Responder {
    let query_result = sqlx::query_as!(
        GameModel,
        "SELECT * FROM m_game"
    )
    .fetch_all(&data.db)
    .await;

    if query_result.is_err() {
        let message: &str = "Something bad happened while fetching the games";
        return HttpResponse::InternalServerError()
            .json(json!({"status": "error", "message": message}))
    }

    let games = query_result.unwrap();

    HttpResponse::Ok().json(json!({
        "status": "success",
        "no. games": games.len(),
        "games": games
    }))
}

#[post("/game")]
async fn create_game(body: web::Json<CreateGameSchema>, data: web::Data<AppState>) -> impl Responder {
    let query_result = sqlx::query_as!(
        GameModel,
        "INSERT INTO m_game (field_name, address, day) VALUES ($1, $2, $3) RETURNING *",
        body.field_name.to_string(),
        body.address.to_string(),
        body.day.to_string()
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(game) => {
            let game_response = serde_json::json!({"status": "success", "data": serde_json::json!({
                "game": game,
            })});
            return HttpResponse::Ok().json(game_response);
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

#[get("/game/{id}")]
async fn get_game_by_id(path: web::Path<uuid::Uuid>, data: web::Data<AppState>) -> impl Responder {
    let game_id = path.into_inner();
    let query_result = sqlx::query_as!(GameModel, "SELECT * FROM m_game WHERE id = $1", game_id)
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(game) => {
            let game_response = serde_json::json!({"status": "success", "data": serde_json::json!({
                "game": game
            })});
            return HttpResponse::Ok().json(game_response);
        },
        Err(_) => {
            let message = format!("Note with ID: {} not found", game_id);
            return HttpResponse::NotFound()
                .json(serde_json::json!({"status": "fail", "message": message}));
        }
    }
}

#[put("/game/{id}")]
async fn update_game(path: web::Path<uuid::Uuid>, data: web::Data<AppState>, body: web::Json<UpdateGameSchema>) -> impl Responder {
    let game_id = path.into_inner();
    // make sure game exists before updating
    let query_result = sqlx::query_as!(GameModel, "SELECT * FROM m_game WHERE id = $1", game_id)
        .fetch_one(&data.db)
        .await;

    if query_result.is_err() {
        let message = format!("Game with ID: {} not found", game_id);
        return HttpResponse::NotFound()
            .json(serde_json::json!({"status": "fail", "message": message}));
    }

    let now = Utc::now();
    let game = query_result.unwrap();

    let query_result = sqlx::query_as!(
        GameModel,
        "UPDATE m_game SET field_name = $1, address = $2, day = $3, updated_at = $4 WHERE id = $5 RETURNING *",
        body.field_name.to_owned().unwrap_or(game.field_name),
        body.address.to_owned().unwrap_or(game.address),
        body.day.to_owned().unwrap_or(game.day),
        now,
        game_id
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(game) => {
            let game_response = serde_json::json!({"status": "success", "data": serde_json::json!({
                "game": game
            })});
            return HttpResponse::Ok().json(game_response);
        },
        Err(_) => {
            let message = format!("Game with ID: {} not found", game_id);
            return HttpResponse::NotFound()
                .json(serde_json::json!({"status": "fail", "message": message}));
        }
    }
}

#[delete("/game/{id}")]
async fn delete_game(path: web::Path<uuid::Uuid>, data: web::Data<AppState>) -> impl Responder {
    let game_id = path.into_inner();

    let query_result = sqlx::query_as!(GameModel, "SELECT * FROM m_game WHERE id = $1", game_id)
    .fetch_one(&data.db)
    .await;

    if query_result.is_err() {
        let message = format!("Game with ID: {} not found", game_id);
        return HttpResponse::NotFound()
            .json(serde_json::json!({"status": "fail", "message": message}));
    }

    let rows_affected = sqlx::query!("DELETE FROM m_game WHERE id = $1", game_id)
        .execute(&data.db)
        .await
        .unwrap()
        .rows_affected();

        if rows_affected == 0 {
            let message = format!("Game with ID: {} not found", game_id);
            return HttpResponse::NotFound()
                .json(serde_json::json!({"status": "fail", "message": message}));
        }
        HttpResponse::NoContent().finish()
}