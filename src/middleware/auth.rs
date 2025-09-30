use actix_web::{HttpRequest, HttpResponse, web};
use serde_json::json;
use crate::utils::create_secret_token::verify_token;
use std::env;

/// Extract and validate JWT token from Authorization header
/// Returns the customer email if token is valid
pub fn extract_customer_from_token(req: &HttpRequest) -> Result<String, HttpResponse> {
    // Get Authorization header
    let auth_header = match req.headers().get("Authorization") {
        Some(header) => header,
        None => {
            return Err(HttpResponse::Unauthorized().json(json!({
                "Success": false,
                "message": "Authorization header missing"
            })));
        }
    };

    // Convert header to string
    let auth_str = match auth_header.to_str() {
        Ok(s) => s,
        Err(_) => {
            return Err(HttpResponse::Unauthorized().json(json!({
                "Success": false,
                "message": "Invalid authorization header format"
            })));
        }
    };

    // Extract token from "Bearer <token>" format
    let token = if auth_str.starts_with("Bearer ") {
        &auth_str[7..]
    } else {
        return Err(HttpResponse::Unauthorized().json(json!({
            "Success": false,
            "message": "Authorization header must start with 'Bearer '"
        })));
    };

    // Get secret key from environment
    let secret = match env::var("SECRET_KEY") {
        Ok(secret) => secret,
        Err(_) => {
            eprintln!("SECRET_KEY environment variable not set");
            return Err(HttpResponse::InternalServerError().json(json!({
                "Success": false,
                "message": "Server configuration error"
            })));
        }
    };

    // Verify token
    match verify_token(token, &secret) {
        Ok(claims) => Ok(claims.sub), // Return customer email
        Err(err) => {
            eprintln!("Token verification failed: {:?}", err);
            Err(HttpResponse::Unauthorized().json(json!({
                "Success": false,
                "message": "Invalid or expired token"
            })))
        }
    }
}

/// Get customer ID from email
pub async fn get_customer_id_by_email(
    email: &str, 
    pool: &web::Data<sqlx::PgPool>
) -> Result<uuid::Uuid, HttpResponse> {
    let query = "SELECT id FROM customers WHERE email = $1";
    
    match sqlx::query_scalar::<_, uuid::Uuid>(query)
        .bind(email)
        .fetch_optional(&***pool)
        .await
    {
        Ok(Some(customer_id)) => Ok(customer_id),
        Ok(None) => Err(HttpResponse::Unauthorized().json(json!({
            "Success": false,
            "message": "Customer not found"
        }))),
        Err(err) => {
            eprintln!("Database error: {:?}", err);
            Err(HttpResponse::InternalServerError().json(json!({
                "Success": false,
                "message": "Database error"
            })))
        }
    }
}
