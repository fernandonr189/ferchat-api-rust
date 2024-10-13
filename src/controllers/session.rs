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
                //println!("Message received {}", json::to_string(&msg).unwrap());
                match stream
                    .send(Message::Text(json::to_string(&msg).unwrap()))
                    .await
                {
                    Ok(_) => {
                        println!("Message sent!");
                        let pong = future::timeout(Duration::from_secs(2), stream.next()).await;
                        match pong {
                            Ok(msg_opt_res) => {
                                match msg_opt_res {
                                    None => {
                                        return Ok(());
                                    }
                                    Some(msg_res) => match msg_res {
                                        Ok(msg) => match msg {
                                            Message::Text(_) => {
                                            }
                                            _ => {
                                                return Ok(());
                                            }
                                        },
                                        Err(_) => {
                                            return Ok(());
                                        }
                                    },
                                };
                            }
                            Err(_) => {
                                return Ok(());
                            }
                        }
                    }
                    Err(_) => {
                        return Ok(());
                    }
                }
            }
            Ok(())
        })
    })
}
