use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct Message {
    pub id: Uuid,
    pub body: String,
    pub created_at: DateTime<Utc>,
}
