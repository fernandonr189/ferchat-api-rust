use crate::models::event_server::{Event, EventData, EventServer, Type};
use crate::models::request_models::msg_request::MsgRequest;
use crate::models::response::{Jwt, NetworkResponse, Response};
use chrono::Utc;
use rocket::serde::json::Json;
use rocket::State;

#[post("/msg", format = "json", data = "<req>")]
pub async fn msg<'r>(
    user: Jwt,
    req: Json<MsgRequest>,
    chat_server: &State<EventServer>,
) -> NetworkResponse<'r, String> {
    let user_id = user.claims.subject_id;
    let message_req: MsgRequest = req.into_inner();

    let destination_tx = match chat_server.get_session_tx(&message_req.destination).await {
        None => {
            chat_server.new_session(&message_req.destination).await;
            chat_server
                .get_session_tx(&message_req.destination)
                .await
                .unwrap()
        }
        Some(tx) => tx,
    };

    // Create message event
    let event = Event {
        event_type: Type::Message,
        data: EventData {
            message: Some(message_req.msg),
            sender_id: user_id,
            status: None,
        },
        timestamp: Utc::now().to_string(),
    };

    match destination_tx.send(event) {
        Ok(_) => NetworkResponse::Ok(Json(Response::<String> {
            error_code: None,
            message: "Message sent!",
            data: None,
        })),
        Err(_) => NetworkResponse::InternalServerError(Json(Response::<String> {
            error_code: Some(500),
            message: "Could not send message",
            data: None,
        })),
    }
}
