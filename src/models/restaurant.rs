use uuid::Uuid;

struct RestaurantInput{
    pub email: String,
    pub password: String,
    pub name: String,
    pub address: String,
    pub contact_number: String,
    pub cuisines: Vec<Cuisine>
}

struct Restaurant{
    pub id: Uuid,
    pub email: String,
    pub password: String,
    pub name: String,
    pub address: String,
    pub contact_number: String,
    pub cuisines: Vec<Cuisine>
}

struct Cuisine{
    pub name: String,
    pub description: Option<String>,
}