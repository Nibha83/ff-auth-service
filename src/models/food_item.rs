use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoodItemRequest {
    pub name: String,
    pub price: f32,
    pub restaurant_id: Uuid,
    pub cuisine_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoodItem {
    pub id: Uuid,
    pub name: String,
    pub price: f32,
    pub restaurant_id: Uuid,
    pub cuisine_id: Option<Uuid>,
}