use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct ScoreModel {
    pub id: Uuid,
    pub username: String,
    pub score: f32,
    pub num_update: u32,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateScoreSchema {
    pub username: String,
    pub score: f32,
    pub num_update: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateScoreSchema {
    pub username: Option<String>,
    pub score: Option<f32>,
    pub num_update: Option<u32>
}
