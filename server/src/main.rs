#[macro_use]
extern crate rocket;
use rocket::serde::json::Json;
use rocket::{Build, Rocket};
use serde::Deserialize;

mod application;
mod domain;
use application::dto::message::MessageDto;
use application::mutation::{send_message::SendMessageMutation, Mutation};
use application::query::{list_messages::ListMessagesQuery, Query};

#[get("/")]
fn list_messages() -> Json<Vec<MessageDto>> {
    let query = ListMessagesQuery {};
    let result = query.execute();
    return Json(result);
}

#[derive(Deserialize)]
struct SendMessageDto {
    body: String,
}

#[post("/", data = "<data>")]
fn send_message(data: Json<SendMessageDto>) -> Json<MessageDto> {
    let mutation = SendMessageMutation {
        body: data.body.clone(),
    };
    let message = mutation.execute();
    return Json(message);
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/messages", routes![list_messages, send_message])
}
