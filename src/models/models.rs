pub mod models;
#[allow(non_snake_case)]
#[derive(Debug, PartialEq, Serialize, Deserialize, FromRow)]

pub struct GameModel {
    pub id: Uuid,
    pub field_name: String,
    pub adress: String,
    pub date: choro::DateTime<chrono::Utc>,
    pub created_at: Option::DateTime<chrono::Utc>,
    pub updated_at: Option::DateTime<chrono::Utc>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, FromRow)]
pub struct CreateGameSchema {
    pub field_name: String,
    pub adress: String,
    pub date: choro::DateTime<chrono::Utc>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, FromRow)]
pub struct UpdateGameSchema {
    pub field_name: String,
    pub adress: String,
    pub date: choro::DateTime<chrono::Utc>,
}
