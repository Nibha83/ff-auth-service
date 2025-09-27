use actix_web::{HttpResponse, Result, web};
use crate::models::CreateCustomerRequest;
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;
use bcrypt::{hash, verify, DEFAULT_COST};

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

// Register a new customer

pub async fn create_customer(
    pool: web::Data<PgPool>,
    customer: web::Json<CreateCustomerRequest>,
) -> Result<HttpResponse> {
    let customer = customer.into_inner();

    // Validate email format
    if !validate_email(&customer.email) {
        return Ok(HttpResponse::BadRequest().json(json!({
            "Success": false,
            "message": "Invalid email format"
        })));
    }

    // Validate password strength
    if let Err(password_error) = validate_password(&customer.password) {
        return Ok(HttpResponse::BadRequest().json(json!({
            "Success": false,
            "message": password_error
        })));
    }

    let id = Uuid::new_v4();

    // Hash the password using bcrypt
    let hashed_password = match hash(&customer.password, DEFAULT_COST) {
        Ok(hashed) => hashed,
        Err(err) => {
            eprintln!("Password hashing error: {:?}", err);
            return Ok(HttpResponse::InternalServerError().json(json!({
                "Success": false,
                "message": "Error processing password"
            })));
        }
    };

    let query = "INSERT INTO customers (id, name, email, password, phone) VALUES ($1, $2, $3, $4, $5) RETURNING id";

    let result = sqlx::query_scalar::<_, Uuid>(query)
        .bind(id)
        .bind(&customer.name)
        .bind(&customer.email)
        .bind(&hashed_password)
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

            // Check if it's a unique constraint violation (duplicate email)
            let error_message = if err.to_string().contains("duplicate key") ||
                                  err.to_string().contains("unique constraint") ||
                                  err.to_string().contains("customers_email_key") {
                "Email address already exists"
            } else {
                "Error creating customer"
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

// login customer
