use crate::controllers::customer::create_customer;
use crate::controllers::customer::get_customer;
use crate::controllers::customer::get_customer_by_id;
use crate::controllers::customer::get_customers;
use crate::controllers::customer::login_customer;
use crate::controllers::customer::add_address;
// use crate::controllers::customer::whoami;
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
    cfg.service(
        web::resource("/customer/address")
            .route(web::post().to(add_address)),
    );
    cfg.service(
        web::resource("/customer/get")
            .route(web::get().to(get_customer)),
    );
    cfg.service(
        web::resource("/customers")
            .route(web::get().to(get_customers)),
    );
    cfg.service(
        web::resource("/customer/{id}")
            .route(web::get().to(get_customer_by_id)),
    );
    // cfg.service(
    //     web::resource("/whoami")
    //         .route(web::get().to(whoami)),
    // );
}
