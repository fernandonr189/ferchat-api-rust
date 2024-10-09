use crate::models::chat_server::ChatServer;
use crate::models::response::NetworkResponse;
use rocket::State;
use rocket::serde::json::Json;
use crate::models::response::Response;
use rocket::futures::{SinkExt, StreamExt};

#[get("/hear/<target_id>/<source_id>")]
pub async fn hear(
    ws: ws::WebSocket,
    target_id: i32,
    source_id: i32,
    chat_server: &State<ChatServer>,
) -> ws::Channel<'static> {
    let user_id = source_id;
    let target_id = target_id;

    let session_id = format!(
        "{}-{}",
        if user_id < target_id {
            user_id
        } else {
            target_id
        },
        if user_id < target_id {
            target_id
        } else {
            user_id
        }
    );

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
                match stream.send(ws::Message::text(&msg.to_string())).await {
                    Ok(_) => {
                        println!("message sent: {}", msg);
                    },
                    Err(_) => {
                        println!("Error sending message");
                    },
                };
            };
            Ok(())
        })
    })
}

#[get("/yell/<target_id>/<source_id>")]
pub async fn yell(
    ws: ws::WebSocket,
    target_id: i32,
    source_id: i32,
    chat_server: &State<ChatServer>,
) -> ws::Channel<'static> {
    let user_id = source_id;
    let target_id = target_id;

    let session_id = format!(
        "{}-{}",
        if user_id < target_id {
            user_id
        } else {
            target_id
        },
        if user_id < target_id {
            target_id
        } else {
            user_id
        }
    );

    let tx = match chat_server.get_session_tx(&session_id).await {
        Some(tx) => {
          tx  
        },
        None => {
            chat_server.new_session(&session_id).await;
            chat_server.get_session_tx(&session_id).await.unwrap()
        },
    };

    ws.channel(move |mut stream| {
        Box::pin(async move {
            while let Some(msg_res) = stream.next().await {
                let msg = match msg_res {
                    Ok(msg) => msg,
                    Err(_) => ws::Message::text("There was an error getting the message"),
                };
                let _ = tx.send(msg.to_text()?.to_string());
            }
            Ok(())
        })
    })
}


#[get("/msg/<target_id>/<source_id>")]
pub async fn msg<'r>(
    target_id: i32,
    source_id: i32,
    chat_server: &State<ChatServer>,
) -> NetworkResponse<'r, String> {
    let user_id = source_id;
    let target_id = target_id;

    let session_id = format!(
        "{}-{}",
        if user_id < target_id {
            user_id
        } else {
            target_id
        },
        if user_id < target_id {
            target_id
        } else {
            user_id
        }
    );

    let tx = match chat_server.get_session_tx(&session_id).await {
        Some(tx) => {
          tx  
        },
        None => {
            chat_server.new_session(&session_id).await;
            chat_server.get_session_tx(&session_id).await.unwrap()
        },
    };

    match tx.send("This is a message sent from another client!".to_string()) {
        Ok(_) => {
            return NetworkResponse::Ok(Json(Response {
                error_code: None,
                message: "Mensaje enviado exitosamente",
                data: None
            }));
        },
        Err(_) => {
            return NetworkResponse::InternalServerError(Json(Response {
                error_code: Some(500),
                message: "Error al enviar el mensaje",
                data: None
            }));
        },
    }
}