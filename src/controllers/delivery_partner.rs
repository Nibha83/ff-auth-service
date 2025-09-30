use crate::{
    models::delivery_partner::{CreateDeliveryPartnerRequest, DeliveryPartnerLoginRequest},
    utils::create_secret_token::generate_token,
};
use actix_web::{HttpResponse, Result, web};
use bcrypt::{DEFAULT_COST, hash, verify};
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

// Helper function to validate password strength
fn validate_password(password: &str) -> Result<(), String> {
    if password.len() < 8 {
        return Err("Password must be at least 8 characters long".to_string());
    }

    if !password.chars().any(|c| c.is_ascii_uppercase()) {
        return Err("Password must contain at least one uppercase letter".to_string());
    }

    if !password.chars().any(|c| c.is_ascii_lowercase()) {
        return Err("Password must contain at least one lowercase letter".to_string());
    }

    if !password.chars().any(|c| c.is_ascii_digit()) {
        return Err("Password must contain at least one number".to_string());
    }

    Ok(())
}

// Helper function to validate email format
fn validate_email(email: &str) -> bool {
    email.contains('@') && email.contains('.') && email.len() > 5
}

// Helper function to verify password against hash
pub fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
    verify(password, hash)
}

// Register a new delivery partner

pub async fn create_delivery_partner(
    pool: web::Data<PgPool>,
    delivery_partner: web::Json<CreateDeliveryPartnerRequest>,
) -> Result<HttpResponse> {
    let delivery_partner = delivery_partner.into_inner();

    // Validate email format
    if !validate_email(&delivery_partner.email) {
        return Ok(HttpResponse::BadRequest().json(json!({
            "Success": false,
            "message": "Invalid email format"
        })));
    }

    // Validate password strength
    if let Err(password_error) = validate_password(&delivery_partner.password) {
        return Ok(HttpResponse::BadRequest().json(json!({
            "Success": false,
            "message": password_error
        })));
    }

    let id = Uuid::new_v4();

    // Hash the password using bcrypt
    let hashed_password = match hash(&delivery_partner.password, DEFAULT_COST) {
        Ok(hashed) => hashed,
        Err(err) => {
            eprintln!("Password hashing error: {:?}", err);
            return Ok(HttpResponse::InternalServerError().json(json!({
                "Success": false,
                "message": "Error processing password"
            })));
        }
    };

    let query = "INSERT INTO delivery_partners (id, name, email, password, phone_number, vehicle_number) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id";

    let result = sqlx::query_scalar::<_, Uuid>(query)
        .bind(id)
        .bind(&delivery_partner.name)
        .bind(&delivery_partner.email)
        .bind(&hashed_password)
        .bind(&delivery_partner.phone_number)
        .bind(&delivery_partner.vehicle_number)
        .fetch_one(&**pool)
        .await;

    match result {
        Ok(delivery_partner_id) => 
            Ok(HttpResponse::Ok().json(json!({
                "Success": true,
                "message": "Delivery Partner created successfully",
                "delivery_partner": {
                    "id": delivery_partner_id,
                    "name": delivery_partner.name,
                    "email": delivery_partner.email,
                    "contact number": delivery_partner.phone_number,
                    "vehicle number":delivery_partner.vehicle_number
                }
            }))),
        Err(err) => {
            eprintln!("Database error: {:?}", err);

            // Check if it's a unique constraint violation (duplicate email)
            let error_message = if err.to_string().contains("duplicate key")
                || err.to_string().contains("unique constraint")
                || err.to_string().contains("delivery_partners_email_key")
            {
                "Email address already exists"
            } else {
                "Error creating delivery partner"
            };

            if error_message == "Email address already exists" {
                Ok(HttpResponse::Conflict().json(json!({
                    "Success": false,
                    "message": error_message
                })))
            } else {
                Ok(HttpResponse::InternalServerError().json(json!({
                    "Success": false,
                    "message": error_message
                })))
            }
        }
    }
}

pub async fn login_delivery_partner(
    pool: web::Data<PgPool>,
    login_request: web::Json<DeliveryPartnerLoginRequest>,
) -> Result<HttpResponse> {
    let login_request = login_request.into_inner();

    let query = "SELECT id, password FROM delivery_partners WHERE email = $1";

    let result = sqlx::query_as::<_, (Uuid, String)>(query)
        .bind(&login_request.email)
        .fetch_one(&**pool)
        .await;

    match result {
        Ok((id, hashed_password)) => {
            match verify_password(&login_request.password, &hashed_password) {
                
                Ok(true) => {
                    let token = generate_token(&id.to_string(),&std::env::var("SECRET_KEY").unwrap());
                    Ok(HttpResponse::Ok().json(json!({
                    "Success": true,
                    "message": "Login successful",
                    "delivery_partner_id": id,
                    "token": token
                })))},
                Ok(false) => Ok(HttpResponse::Unauthorized().json(json!({
                    "Success": false,
                    "message": "Invalid credentials"
                }))),
                Err(err) => {
                    eprintln!("Password verification error: {:?}", err);
                    Ok(HttpResponse::InternalServerError().json(json!({
                        "Success": false,
                        "message": "Error processing password"
                    })))
                }
            }
        }
        Err(err) => {
            eprintln!("Database error: {:?}", err);
            Ok(HttpResponse::NotFound().json(json!({
                "Success": false,
                "message": "Delivery Partner not found"
            })))
        }
    }
}
