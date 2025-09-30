use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRestaurantRequest{
    pub email: String,
    pub password: String,
    pub name: String,
    pub address: String,
    pub contact_number: String,
    pub cuisines: Vec<Cuisine>
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Restaurant{
    pub id: Uuid,
    pub email: String,
    pub password: String,
    pub name: String,
    pub address: String,
    pub contact_number: String,
    pub cuisines: Vec<Cuisine>
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cuisine{
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestaurantLoginRequest{
    pub email: String,
    pub password: String,
}