use crate::models::chat_server::ChatServer;
use crate::models::chat_server::ChatSession;
use crate::models::response::JWT;
use futures::task::noop_waker;
use rocket::futures;
use rocket::futures::StreamExt;
use rocket::tokio::sync::Mutex;
use rocket::tokio::sync::MutexGuard;
use rocket::State;
use std::sync::Arc;
use std::thread::sleep;
use std::task::{Context, Poll};
use std::time::Duration;

#[get("/echo/<target_id>/<source_id>")]
pub async fn echo(
    ws: ws::WebSocket,
    target_id: i32,
    source_id: i32,
    chat_server: &State<ChatServer>,
) -> ws::Channel<'static> {
    let user_id = source_id;

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

    let chat_session: (Arc<Mutex<ChatSession>>, Arc<Mutex<ChatSession>>) =
        match chat_server.get_session(&session_id).await {
            Some(session_tuple) => session_tuple,
            _ => chat_server.new_session(&session_id).await,
        };

    let current_session: Arc<Mutex<ChatSession>> = if user_id < target_id {
        chat_session.0
    } else {
        chat_session.1
    };

    ws.channel(move |mut stream| {
        Box::pin(async move {
            let mut session: MutexGuard<ChatSession> = current_session.lock().await;
            let waker = noop_waker();
            let mut cx = Context::from_waker(&waker);
            loop {
                match session.receiver.poll_recv(&mut cx) {
                    Poll::Ready(message_option) => {
                        // Send message to user via stream
                            let msg = match message_option {
                                Some(msg) => msg,
                                _ => "No message received".to_string(),
                            };
                            println!("Message received from peer: {}", msg);
                    }
                    Poll::Pending => {
                        sleep(Duration::from_secs(1));
                    }
                };
                match stream.poll_next_unpin(&mut cx) {
                    Poll::Ready(message_option) => {
                        let message_result = match message_option {
                            Some(msg_res) => match msg_res {
                                Ok(msg) => msg,
                                Err(_) => ws::Message::text("Could not get message"),
                            },
                            _ => ws::Message::text("Could not get message"),
                        };
                        match session.sender.send(message_result.to_string()) {
                            Ok(_) => {
                                println!("Mensage enviado correctamente: {}", message_result.to_string());
                            },
                            Err(_) => {
                                println!("Ocurrio un error al enviar el mensaje: {}", message_result.to_string());
                            },
                        }
                    }
                    Poll::Pending => {
                        sleep(Duration::from_secs(1));
                    },
                }
            }
        })
    })
}


// bugs:
// Hay que solucionar lo que sucede al desconectarse el cliente
// Las sesiones tienen que estar cruzadas, actualmente se envia y recibe del mismo canal causando un bucle