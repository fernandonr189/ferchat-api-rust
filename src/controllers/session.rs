use rocket::futures::SinkExt;
use rocket::State;
use rocket::serde::json;
use crate::models::response::Jwt;
use crate::models::event_server::EventServer;

#[get("/session")]
pub async fn session(
    ws: ws::WebSocket,
    jwt: Jwt,
    event_server: &State<EventServer>,
) -> ws::Channel<'static> {
    let user_id = jwt.claims.subject_id;

    let session_rx_mutex = match event_server.get_session_rx(&user_id).await {
        None => {
            event_server.new_session(&user_id).await;
            event_server.get_session_rx(&user_id).await.unwrap()
        }
        Some(session) => session
    };
    ws.channel(move |mut stream| {
        Box::pin(async move {
            let mut session_rx = session_rx_mutex.lock().await;
            while let Some(msg) = session_rx.recv().await {
                let _ = stream.send(ws::Message::Text(json::to_string(&msg).unwrap())).await;
            };
            Ok(())
        })
    })
}