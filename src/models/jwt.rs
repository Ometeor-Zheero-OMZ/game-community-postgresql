use actix_web::{
    dev::Payload, error::InternalError, http::header, web::{get, post, Json}, App, FromRequest, HttpRequest, HttpResponse, HttpServer
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::future::{ready, Ready};

use crate::middlewares::{get_jwt, decode_jwt};

#[derive(Deserialize, Serialize)]
pub struct UserEmail {
  pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub email: String,
    pub exp: i64
}

pub struct Auth(pub UserEmail);

impl FromRequest for Auth {
    type Error = InternalError<String>;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let access_token = req
            .headers()
            .get(header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .and_then(|str| str.split(" ").nth(1));

        match access_token {
            Some(token) => {
                let user = decode_jwt(token);

                match user {
                    Ok(user) => ready(Ok(Auth(user))),
                    Err(e) => ready(Err(
                        InternalError::from_response(
                            e.clone(),
                            HttpResponse::Unauthorized().json(json!({
                                "success": false,
                                "data": {
                                    "message": e
                                }
                            }))
                        )
                    ))
                }
            },
            None => ready(Err(
                InternalError::from_response(
                    String::from("No token provided"),
                    HttpResponse::Unauthorized().json(json!({
                        "success": false,
                        "data": {
                            "message": "No token provided"
                        }
                    }))
                )
            ))
        }
    }
}