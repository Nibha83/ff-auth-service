use sqlx::prelude::FromRow;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

// DTO for creating a new customer (without database-generated fields)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCustomerRequest {
    pub name: String,
    pub email: String,
    pub password: String,
    pub phone: String,
}

// DTO for adding a new address (without customer_id - will be extracted from token)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAddressRequest {
    pub name: String,
    pub pincode: String,
    pub phone: String,
    pub address: String,
    #[serde(default)]
    pub is_default: bool,
}

// Simple customer response without addresses (for database queries)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CustomerResponse {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize,FromRow)]
pub struct Customer {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub addresses: Vec<Address>,
    pub phone: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Address {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub name: String,
    pub pincode: String,
    pub phone: String,
    pub address: String,
    #[serde(default)]
    pub is_default: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct LoginCustomer{
    pub email: String,
    pub password: String,
}