use chrono::Utc;
use rusqlite::{params, Connection};
use uuid::Uuid;

use crate::domain::entity::message::Message;

use super::super::error::{Error, ErrorType};
use super::super::dto::message::MessageDto;
use super::Mutation;

pub struct SendMessageMutation {
    pub body: String,
}

impl Mutation<MessageDto> for SendMessageMutation {
    fn execute(&self) -> Result<MessageDto, Error> {
        let conn = match Connection::open("db.sqlite3") {
            Ok(conn) => conn,
            Err(err) => return Err(Error { error_type: ErrorType::Unavailable, message: String::from("database is not available") }),
        };
        let message = Message {
            id: Uuid::new_v4(),
            body: self.body.clone(),
            created_at: Utc::now(),
        };
        match conn.execute(
            "INSERT INTO messages (id, body, created_at) VALUES (?1, ?2, ?3)",
            params![message.id, message.body, message.created_at],
        ) {
            Err(err) => match err {
                rusqlite::Error::
            }

        };
        return Ok(MessageDto {
            id: message.id,
            body: message.body,
            created_at: message.created_at,
        });
    }
}
