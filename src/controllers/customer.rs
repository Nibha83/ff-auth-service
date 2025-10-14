
use crate::models::customer::LoginCustomer;
use crate::{
    middleware::auth::{extract_customer_from_token, get_customer_id_by_email},
    models::{CreateAddressRequest, CreateCustomerRequest, CustomerResponse},
    utils::create_secret_token::generate_token,
};
use actix_web::{HttpRequest, HttpResponse, Result, web};
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
            let error_message = if err.to_string().contains("duplicate key")
                || err.to_string().contains("unique constraint")
                || err.to_string().contains("customers_email_key")
            {
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

pub async fn login_customer(
    pool: web::Data<PgPool>,
    customer: web::Json<LoginCustomer>,
) -> Result<HttpResponse> {
    let customer = customer.into_inner();
    let query = "SELECT id, password FROM customers WHERE email = $1";
    let result = sqlx::query_as::<_, (Uuid, String)>(query)
        .bind(&customer.email)
        .fetch_one(&**pool)
        .await;
    match result {
        Ok((customer_id, hashed_password)) => {
            if verify_password(&customer.password, &hashed_password).unwrap_or(false) {
                let access_token = generate_token(
                    &customer.email.to_string(),
                    &std::env::var("SECRET_KEY").unwrap(),
                );
                let access_token_id = Uuid::new_v4();
                let access_token_query="INSERT INTO access_tokens (id, access_token, user_role, user_id, created_at, updated_at) VALUES ($1, $2, $3, $4, now(), now()) RETURNING id";
                let _access_token_result= sqlx::query_scalar::<_, Uuid>(access_token_query)
                    .bind(access_token_id)
                    .bind(&access_token)
                    .bind("customer")
                    .bind(&customer_id)
                    .fetch_one(&**pool)
                    .await;

                match _access_token_result {
                    Ok(_) => (),
                    Err(err) => {
                        eprintln!("Database error: {:?}", err);
                        return Ok(HttpResponse::InternalServerError().json(json!({
                            "Success": false,
                            "message": "Error creating access token"
                        })));
                    }
                }

                Ok(HttpResponse::Ok().json(json!({
                    "Success": true,
                    "message": "Login successful",
                    "token":access_token
                })))
            } else {
                Ok(HttpResponse::Unauthorized().json(json!({
                    "Success": false,
                    "message": "Invalid credentials"
                })))
            }
        }
        Err(_) => Ok(HttpResponse::Unauthorized().json(json!({
            "Success": false,
            "message": "Invalid credentials"
        }))),
    }
}

// add address - requires authentication

pub async fn add_address(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    address_data: web::Json<CreateAddressRequest>,
) -> Result<HttpResponse> {
    // Extract customer email from JWT token
    let customer_email = match extract_customer_from_token(&req) {
        Ok(email) => email,
        Err(response) => return Ok(response),
    };

    // Get customer ID from email
    let customer_id = match get_customer_id_by_email(&customer_email, &pool).await {
        Ok(id) => id,
        Err(response) => return Ok(response),
    };

    let address_data = address_data.into_inner();

    // If this address is set as default, unset other default addresses for this customer
    if address_data.is_default {
        let update_query =
            "UPDATE addresses SET is_default = false WHERE customer_id = $1 AND is_default = true";
        if let Err(err) = sqlx::query(update_query)
            .bind(customer_id)
            .execute(&**pool)
            .await
        {
            eprintln!("Error updating default addresses: {:?}", err);
            return Ok(HttpResponse::InternalServerError().json(json!({
                "Success": false,
                "message": "Error updating default addresses"
            })));
        }
    }

    // Insert new address
    let address_id = Uuid::new_v4();
    let query = "INSERT INTO addresses (id, customer_id, name, pincode, phone, address, is_default) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING id";
    let result = sqlx::query_scalar::<_, Uuid>(query)
        .bind(address_id)
        .bind(customer_id)
        .bind(&address_data.name)
        .bind(&address_data.pincode)
        .bind(&address_data.phone)
        .bind(&address_data.address)
        .bind(address_data.is_default)
        .fetch_one(&**pool)
        .await;

    match result {
        Ok(created_address_id) => Ok(HttpResponse::Ok().json(json!({
            "Success": true,
            "message": "Address added successfully",
            "address": {
                "id": created_address_id,
                "customer_id": customer_id,
                "name": address_data.name,
                "pincode": address_data.pincode,
                "phone": address_data.phone,
                "address": address_data.address,
                "is_default": address_data.is_default,
            }
        }))),
        Err(err) => {
            eprintln!("Database error: {:?}", err);
            Ok(HttpResponse::InternalServerError().json(json!({
                "Success": false,
                "message": "Error adding address"
            })))
        }
    }
}

pub async fn get_customer(req: HttpRequest, pool: web::Data<PgPool>) -> Result<HttpResponse> {
    // Extract customer email from JWT token
    let customer_email = match extract_customer_from_token(&req) {
        Ok(email) => email,
        Err(response) => return Ok(response),
    };
    let query =
        "SELECT id, name, email, phone, created_at, updated_at FROM customers WHERE email = $1";
    let result = sqlx::query_as::<_, CustomerResponse>(query)
        .bind(&customer_email)
        .fetch_one(&**pool)
        .await;
    match result {
        Ok(customer) => Ok(HttpResponse::Ok().json(json!({
            "Success": true,
            "message": "Customer fetched successfully",
            "customer": customer
        }))),
        Err(err) => {
            eprintln!("Database error: {:?}", err);
            Ok(HttpResponse::InternalServerError().json(json!({
                "Success": false,
                "message": "Error fetching customer"
            })))
        }
    }
}

pub async fn get_customers(pool: web::Data<PgPool>) -> Result<HttpResponse> {
    let query = "SELECT id, name, email, phone, created_at, updated_at FROM customers";
    let result = sqlx::query_as::<_, CustomerResponse>(query)
        .fetch_all(&**pool)
        .await;

    match result {
        Ok(customers) => {
            let customer_length = customers.len();
            Ok(HttpResponse::Ok().json(json!({
                "Success": true,
                "message": "Customers fetched successfully",
                "customers": customers,
                "length": customer_length
            })))
        }
        Err(_) => {
            eprintln!("Database error");
            Ok(HttpResponse::InternalServerError().json(json!({
                "Success": false,
                "message": "Error fetching customers"
            })))
        }
    }
}

pub async fn get_customer_by_id(pool: web::Data<PgPool>, id: web::Path<Uuid>) -> Result<HttpResponse> {
    let query = "SELECT id, name, email, phone, created_at, updated_at FROM customers WHERE id = $1";
    let result = sqlx::query_as::<_, CustomerResponse>(query)
        .bind(id.into_inner())
        .fetch_one(&**pool)
        .await;
    match result {
        Ok(customer) => Ok(HttpResponse::Ok().json(json!({
            "Success": true,
            "message": "Customer fetched successfully",
            "customer": customer
        }))),
        Err(_) => {
            eprintln!("Database error");
            Ok(HttpResponse::InternalServerError().json(json!({
                "Success": false,
                "message": "Error fetching customer"
            })))
        }
    }
}

