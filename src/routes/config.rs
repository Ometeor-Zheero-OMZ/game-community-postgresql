use actix_web::web;

use super::{
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

    conf.service(game_scope);
    conf.service(user_scope);
}