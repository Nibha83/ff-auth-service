use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct DeliveryPartner{
    pub id: Uuid,
    pub name: String,
    pub phone_number: String,
    pub email: String,
    pub password: String,
    pub vehicle_number: String,
    pub rating: f32,
    pub is_available: bool   
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateDeliveryPartnerRequest{
    pub name: String,
    pub phone_number: String,
    pub email: String,
    pub password: String,
    pub vehicle_number: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DeliveryPartnerLoginRequest{
    pub email: String,
    pub password: String
}