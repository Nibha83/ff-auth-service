use crate::controllers::customer::create_customer;
use crate::controllers::customer::login_customer;
use actix_web::web;

pub fn customer_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/customer")
            .route(web::post().to(create_customer)),
    );
    cfg.service(
        web::resource("/customer/login")
            .route(web::post().to(login_customer)),
    );
}
