use rocket::futures::SinkExt;
use rocket::State;
use rocket::tokio::sync::mpsc::UnboundedReceiver;
use rocket::serde::json::Json;
use crate::models::chat_server::ChatServer;
use crate::models::response::Jwt;
use crate::models::event_server::{Event, EventServer};

#[get("/session")]
pub async fn session(
    ws: ws::WebSocket,
    target_id: i32,
    jwt: Jwt,
    event_server: &State<EventServer>,
) -> ws::Channel<'static> {
    let user_id = jwt.claims.subject_id;

    let mut session_rx = match event_server.get_session_rx(&user_id).await {
        None => {
            event_server.new_session(&user_id).await;
            event_server.get_session_rx(&user_id).await.unwrap()
        }
        Some(session) => session
    };
    ws.channel(move |mut stream| {
        Box::pin(async move {
            while let Some(msg) = session_rx.recv().await {
                let _ = stream.send(ws::Message::Text(Json(msg).to_string())).await;
            };
            Ok(())
        })
    })
}