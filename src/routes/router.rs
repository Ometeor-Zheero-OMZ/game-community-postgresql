use actix_web::web::{self, get, post, put, delete};

use crate::controllers::{
    game::{
        create_game,
        delete_game,
        get_game_by_id,
        get_games,
        update_game
    }, 
    user::{
        create_user,
        delete_user,
        get_user_by_id,
        get_users,
        update_user
    },
    jwt::{
        public_view_handler,
        get_token_handler,
        secret_view_handler
    }
};

pub fn config(conf: &mut web::ServiceConfig) {
    let game_scope = web::scope("/api/games")
        .service(get_games)
        .service(get_game_by_id)
        .service(create_game)
        .service(update_game)
        .service(delete_game);

    let user_scope = web::scope("/api/users")
        .service(get_users)
        .service(get_user_by_id)
        .service(create_user)
        .service(update_user)
        .service(delete_user);

    let auth_scope = web::scope("/auth")
        .service(public_view_handler)
        .service(get_token_handler)
        .service(secret_view_handler);

    conf.service(game_scope);
    conf.service(user_scope);
    conf.service(auth_scope);

}