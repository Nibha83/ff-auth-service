use crate::controllers::restaurant::{create_restaurant, get_restaurant_by_id, login_restaurant, get_restaurants};
use actix_web::web;

pub fn restaurant_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/restaurant")
            .route(web::post().to(create_restaurant))
            .route(web::get().to(get_restaurants))
    );
    cfg.service(
        web::resource("/restaurant/login")
            .route(web::post().to(login_restaurant))
    );
    cfg.service(
        web::resource("/restaurant/{id}")
            .route(web::get().to(get_restaurant_by_id))
    );
}
