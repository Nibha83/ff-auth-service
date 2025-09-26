use actix_web::{HttpResponse, Result, web};
use crate::models::CreateCustomerRequest;
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn create_customer(
    pool: web::Data<PgPool>,
    customer: web::Json<CreateCustomerRequest>,
) -> Result<HttpResponse> {
    let customer = customer.into_inner();
    let id = Uuid::new_v4();
    let query = "INSERT INTO customers (id, name, email, password, phone) VALUES ($1, $2, $3, $4, $5) RETURNING id";

    let result = sqlx::query_scalar::<_, Uuid>(query)
        .bind(id)
        .bind(&customer.name)
        .bind(&customer.email)
        .bind(&customer.password)
        .bind(&customer.phone)
        .fetch_one(&**pool)
        .await;

    match result {
        Ok(customer_id) => Ok(HttpResponse::Ok().json(json!({
            "Success": true,
            "message": "Customer created successfully",
            "customer": {
                "id": customer_id,
                "name": customer.name,
                "email": customer.email,
                "phone": customer.phone,
            }
        }))),

        Err(err) => {
            eprintln!("Database error: {:?}", err);
            Ok(HttpResponse::InternalServerError().json(json!({
                "Success": false,
                "message": "Error creating customer"
            })))
        }
    }
}
