use crate::models::event_server::EventServer;
use crate::models::response::Jwt;
use async_std::future;
use rocket::futures::{SinkExt, StreamExt};
use rocket::serde::json;
use rocket::State;
use std::time::Duration;
use ws::Message;

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
        Some(session) => session,
    };

    ws.channel(move |mut stream| {
        Box::pin(async move {
            let mut session_rx = session_rx_mutex.lock().await;
            while let Some(msg) = session_rx.recv().await {
                match stream
                    .send(Message::Text(json::to_string(&msg).unwrap()))
                    .await
                {
                    Ok(res) => res,
                    Err(_) => {
                        return Ok(());
                    }
                };
                let pong = future::timeout(Duration::from_secs(2), stream.next()).await;
                let opt_pong = match pong {
                    Ok(msg_opt_res) => msg_opt_res,
                    Err(_) => {
                        return Ok(());
                    }
                };
                let msg_res = match opt_pong {
                    Some(msg_res) => msg_res,
                    None => {
                        return Ok(());
                    }
                };
                let msg_type = match msg_res {
                    Ok(msg_type) => msg_type,
                    Err(_) => {
                        return Ok(());
                    }
                };
                match msg_type {
                    Message::Text(_) => {}
                    _ => {
                        return Ok(());
                    }
                };
            }
            Ok(())
        })
    })
}
