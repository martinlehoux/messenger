use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct MessageDto {
    pub id: Uuid,
    pub body: String,
    pub created_at: DateTime<Utc>,
}
