use crate::controllers::delivery_partner::{create_delivery_partner, login_delivery_partner};
use actix_web::web;

pub fn delivery_partner_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/delivery/partner")
            .route(web::post().to(create_delivery_partner))
    );
    cfg.service(
        web::resource("/delivery/partner/login")
            .route(web::post().to(login_delivery_partner))
    );
}
