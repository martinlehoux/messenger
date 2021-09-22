use rusqlite::Connection;

use super::super::dto::message::MessageDto;
use super::Query;

pub struct ListMessagesQuery;

impl Query<Vec<MessageDto>> for ListMessagesQuery {
    fn execute(&self) -> Vec<MessageDto> {
        let conn = Connection::open("db.sqlite3").unwrap();
        let mut query = conn
            .prepare("SELECT id, body, created_at FROM messages")
            .unwrap();
        let messages_map = query
            .query_map([], |row| {
                return Ok(MessageDto {
                    id: row.get("id")?,
                    body: row.get("body")?,
                    created_at: row.get("created_at")?,
                });
            })
            .unwrap();
        let mut messages = Vec::<MessageDto>::new();
        for message in messages_map {
            messages.push(message.unwrap());
        }
        return messages;
    }
}
