use crate::controllers::customer::create_customer;
use actix_web::web;

pub fn customer_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/customer")
            .route(web::post().to(create_customer)),
    );
}
