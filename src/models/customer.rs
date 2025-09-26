use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Customer {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub addresses: Vec<Address>,
    pub phone: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub name: String,
    pub pincode: String,
    pub phone: String,
    pub address: String,
    pub is_default: bool,
}