use crate::models::chat_server::ChatServer;
use crate::models::chat_server::ChatSession;
use crate::models::response::JWT;
use futures::future::BoxFuture;
use futures::task::noop_waker;
use rocket::futures;
use rocket::futures::{SinkExt, StreamExt};
use rocket::tokio;
use rocket::tokio::sync::Mutex;
use rocket::tokio::sync::MutexGuard;
use rocket::State;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

#[get("/echo/<target_id>")]
pub async fn echo(
    ws: ws::WebSocket,
    target_id: i32,
    jwt: JWT,
    chat_server: &State<ChatServer>,
) -> ws::Channel<'static> {
    let user_id = jwt.claims.subject_id;

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
            loop {
                let mut receiver: MutexGuard<ChatSession> = current_session.lock().await;
                receiver.receiver.recv();
            }

            Ok(())
        })
    })
}

fn poll_my_future(fut: &mut Pin<&mut dyn Future<Output = String>>) -> Poll<String> {
    // Create a no-op waker (in real-world, you would get this from an executor)
    let waker = noop_waker();

    // Create a task context from the waker
    let mut cx = Context::from_waker(&waker);

    // Poll the future
    fut.as_mut().poll(&mut cx)
}
// TODO implementacion de chat en timepo real:
// 1.- Utilizar el State de rocket para manejar un hashmap con todas las sesiones de chat
// 2.- Crear un struct para representar los mensajes
// 3.- Utilizar la librerio tokio para manejar los mpsc's
// 4.- Asegurar que el struct de mensajes implemente Send de forma segura
// 5.- Implementar metodos asyncronos para que el websocket lea y envie mensajes simultaneamente
//
