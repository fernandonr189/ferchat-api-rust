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
