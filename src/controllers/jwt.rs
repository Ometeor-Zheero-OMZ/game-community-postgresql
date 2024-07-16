use actix_web::{
    get,
    post,
    web::Json,
    HttpResponse,
    Responder
};
use serde_json::json;

use crate::middlewares::{get_jwt, decode_jwt};
use crate::models::jwt::{Auth, Claims, UserEmail};

#[get("/public-view")]
pub async fn public_view_handler() -> impl Responder {
    HttpResponse::Ok()
        .json(json!({
            "success": true,
            "data": {
                "message": "This data is available to all users"
            }
        }))
}

#[post("get-token")]
pub async fn get_token_handler(Json(user): Json<UserEmail>) -> impl Responder {
    let token = get_jwt(user);

    match token {
        Ok(token) => HttpResponse::Ok()
            .json(json!({
                "success": true,
                "data": {
                    "token": token
                }
            })
        ),

        Err(e) => HttpResponse::BadRequest()
            .json(json!({
                "success": false,
                "data": {
                    "message": e
                }
            })
        )
    }
}

#[get("/secret-view")]
pub async fn secret_view_handler(Auth(user): Auth) -> impl Responder {
    HttpResponse::Ok()
        .json(json!({
                "success": true,
                "data": user
            })
        )
}