use chrono::{Utc, Duration};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // Subject (user id, email, etc.)
    pub exp: usize,   // Expiration time (as UTC timestamp)
}

/// Generate a JWT for a given user_id.
/// Token expires in 1 hour.
pub fn generate_token(user_id: &str, secret: &str) -> String {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(48))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    ).expect("Token creation failed")
}

/// Verify and decode a JWT.
/// Returns Claims if valid, or an error if invalid/expired.
pub fn verify_token(token: &str, secret: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )?;

    Ok(token_data.claims)
}
