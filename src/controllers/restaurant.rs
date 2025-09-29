use actix::{web, HttpResponse, Result};
use sqlx::PgPool;
use serde_json::json;
use models::{Customer, Address, RestaurantInput};
 
pub async fn create_restaurant(pool: web::Data<PgPool>, restaurant: web::Json<RestaurantInput>) -> Result<HttpResponse> {
    let restaurant = restaurant.into_inner();
    let id = uuid::Uuid::new_v4();
    let query = "INSERT INTO restaurants (id, name, email, password, phone) VALUES ($1, $2, $3, $4, $5) RETURNING id";
    let id = sqlx::query::<_,Customer>(query)
        .bind(id)
        .bind(customer.name)
        .bind(customer.email)
        .bind(customer.password)
        .bind(customer.phone)
        .fetch_one(&pool)
        .await;
 
    match id {
        Ok(id)=>{
            HttpResponse::Ok().json(json!({
                "Success":true,
                "message":"Customer created successfully",
                "customer":{
                    "id": id,
                    "name": customer.name,
                    "email": customer.email,
                    "phone": customer.phone,
                }
            }))
        }
    }
 
    Ok(HttpResponse::Ok().json(json!({"id": id})))
}