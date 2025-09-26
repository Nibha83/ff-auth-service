mod config;
mod models;

use config::db::connect_cockroach;
use actix_web::{web, App, HttpServer, HttpResponse, Result};

async fn health_check() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "auth-service"
    })))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting Auth Service...");
   
    //connect cockroachDB
    let cockroach_pool = connect_cockroach().await;
    println!("Connected to cockroachDB");

    println!("Auth Service is running and ready to accept connections");

    Ok(HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(cockroach_pool.clone()))
            .route("/health", web::get().to(health_check))
    })
    .bind(("127.0.0.1", 8001))?
    .run()
    .await?)
}
