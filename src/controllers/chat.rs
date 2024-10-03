use rocket::futures::{SinkExt, StreamExt};

use crate::models::response::JWT;

#[get("/echo/<target_id>")]
pub fn echo(ws: ws::WebSocket, target_id: i32, jwt: JWT) -> ws::Channel<'static> {
    let user_id = jwt.claims.subject_id;

    ws.channel(move |mut stream| {
        Box::pin(async move {
            while let Some(message) = stream.next().await {
                let response = match message {
                    Ok(msg) => ws::Message::Text(format!("Message from {}: {}", user_id, msg)),
                    Err(_err) => ws::Message::Text("There was an error".to_string()),
                };
                let _ = stream.send(response).await;
            }

            Ok(())
        })
    })
}

// TODO implementacion de chat en timepo real:
// 1.- Utilizar el State de rocket para manejar un hashmap con todas las sesiones de chat
// 2.- Crear un struct para representar los mensajes
// 3.- Utilizar la librerio tokio para manejar los mpsc's
// 4.- Asegurar que el struct de mensajes implemente Send de forma segura
// 5.- Implementar metodos asyncronos para que el websocket lea y envie mensajes simultaneamente
//
