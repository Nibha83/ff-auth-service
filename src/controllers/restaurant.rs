use crate::{
    models::restaurant::{CreateRestaurantRequest, RestaurantLoginRequest},
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

// Register a new restaurant

pub async fn create_restaurant(
    pool: web::Data<PgPool>,
    restaurant: web::Json<CreateRestaurantRequest>,
) -> Result<HttpResponse> {
    let restaurant = restaurant.into_inner();

    // Validate email format
    if !validate_email(&restaurant.email) {
        return Ok(HttpResponse::BadRequest().json(json!({
            "Success": false,
            "message": "Invalid email format"
        })));
    }

    // Validate password strength
    if let Err(password_error) = validate_password(&restaurant.password) {
        return Ok(HttpResponse::BadRequest().json(json!({
            "Success": false,
            "message": password_error
        })));
    }

    let id = Uuid::new_v4();

    // Hash the password using bcrypt
    let hashed_password = match hash(&restaurant.password, DEFAULT_COST) {
        Ok(hashed) => hashed,
        Err(err) => {
            eprintln!("Password hashing error: {:?}", err);
            return Ok(HttpResponse::InternalServerError().json(json!({
                "Success": false,
                "message": "Error processing password"
            })));
        }
    };

    let query = "INSERT INTO restaurants (id, name, email, password, contact_number, address) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id";

    let result = sqlx::query_scalar::<_, Uuid>(query)
        .bind(id)
        .bind(&restaurant.name)
        .bind(&restaurant.email)
        .bind(&hashed_password)
        .bind(&restaurant.contact_number)
        .bind(&restaurant.address)
        .fetch_one(&**pool)
        .await;

    match result {
        Ok(restaurant_id) => {
            for cuisine in restaurant.cuisines {
                let id = Uuid::new_v4();
                let result_cuisine = if cuisine.description.is_none() {
                    let query_cuisine = "INSERT INTO cuisines (id, restaurant_id, name) VALUES ($1, $2, $3) returning id";
                    sqlx::query_scalar::<_, Uuid>(query_cuisine)
                        .bind(id)
                        .bind(restaurant_id)
                        .bind(cuisine.name)
                        .fetch_one(&**pool)
                        .await
                } else {
                    let query_cuisine = "INSERT INTO cuisines (id, restaurant_id, name, description) VALUES ($1, $2, $3, $4) returning id";
                    sqlx::query_scalar::<_, Uuid>(query_cuisine)
                        .bind(id)
                        .bind(restaurant_id)
                        .bind(cuisine.name)
                        .bind(cuisine.description)
                        .fetch_one(&**pool)
                        .await
                };
                match result_cuisine {
                    Ok(cuisine_id) => {
                        println!("Cuisine created successfully with id: {}", cuisine_id);
                    }
                    Err(err) => {
                        eprintln!("Database error: {:?}", err);
                    }
                }
            }

            Ok(HttpResponse::Ok().json(json!({
                "Success": true,
                "message": "Restaurant created successfully",
                "restaurant": {
                    "id": restaurant_id,
                    "name": restaurant.name,
                    "email": restaurant.email,
                    "contact number": restaurant.contact_number,
                    "address":restaurant.address
                }
            })))
        }

        Err(err) => {
            eprintln!("Database error: {:?}", err);

            // Check if it's a unique constraint violation (duplicate email)
            let error_message = if err.to_string().contains("duplicate key")
                || err.to_string().contains("unique constraint")
                || err.to_string().contains("restaurants_email_key")
            {
                "Email address already exists"
            } else {
                "Error creating restaurant"
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

pub async fn login_restaurant(
    pool: web::Data<PgPool>,
    login_request: web::Json<RestaurantLoginRequest>,
) -> Result<HttpResponse> {
    let login_request = login_request.into_inner();

    let query = "SELECT id, password FROM restaurants WHERE email = $1";

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
                    "restaurant_id": id,
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
                "message": "Restaurant not found"
            })))
        }
    }
}
