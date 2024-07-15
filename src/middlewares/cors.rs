use crate::utils::SET_ENV_MSG;

use actix_cors::Cors;
use actix_web::http::header;
use dotenv::dotenv;

pub fn cors() -> Cors {
    dotenv().ok();
    let frontend_port = std::env::var("FRONTEND_PORT").expect(SET_ENV_MSG.get("NO_SET_ENV_VAR_FRONTEND_PORT").unwrap_or(&""));

    let cors = Cors::default()
        .allowed_origin(&frontend_port)
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
        .allowed_headers(vec![
            header::CONTENT_TYPE,
            header::AUTHORIZATION,
            header::ACCEPT,
        ])
        .supports_credentials();

    cors
}