use std::sync::Arc;
use std::task::Context;
use rocket::tokio;
use rocket::tokio::sync::Mutex;
use crate::models::chat_server::ChatServer;
use crate::models::response::Jwt;
use rocket::futures::task::noop_waker;
use rocket::State;
use rocket::futures::{SinkExt, StreamExt};

#[get("/hear/<target_id>")]
pub async fn hear(
    ws: ws::WebSocket,
    target_id: i32,
    jwt: Jwt,
    chat_server: &State<ChatServer>,
) -> ws::Channel<'static> {
    let user_id = jwt.claims.subject_id;

    let session_id = format!(
        "{}-{}-{}",
        if user_id < target_id {
            user_id
        } else {
            target_id
        },
        if user_id < target_id {
            target_id
        } else {
            user_id
        },
        user_id
    );

    println!("session_id: {}", session_id);

    let rx = match chat_server.get_session_rx(&session_id).await {
        Some(rx) => {
          rx  
        },
        None => {
            chat_server.new_session(&session_id).await;
            chat_server.get_session_rx(&session_id).await.unwrap()
        },
    };

    ws.channel(move |mut stream| {
        Box::pin(async move {
            let mut rx_locked = rx.lock().await;
            while let Some(msg) = rx_locked.recv().await {
                match stream.send(ws::Message::text(msg.to_string())).await {
                    Ok(_) => {
                        println!("message sent to peer ws: {}", msg);
                    },
                    Err(_) => {
                        println!("Error sending message");
                        return Ok(());
                    },
                };
            };
            Ok(())
        })
    })
}