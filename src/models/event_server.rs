use rocket::tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel};
use rocket::tokio::sync::Mutex;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct EventServer {
    event_receiver: Mutex<HashMap<String, UnboundedReceiver<Event>>>,
    event_sender: Mutex<HashMap<String, UnboundedSender<Event>>>
}

pub struct Event {
    pub event_type: Type,
    pub data: Option<EventData>
}

pub enum Type {
    Message,
    OnlineStatus,
    FriendRequestStatus,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct EventData {
    pub message: Option<String>,
    pub sender_id: i32,
    pub status: Option<bool>,
}

impl EventServer {
    pub async fn new_session(&self, session_id: &str) {
        let (tx, rx) = unbounded_channel::<Event>();
        self.event_sender.lock().await.insert(session_id.to_string(), tx);
        self.event_receiver.lock().await.insert(session_id.to_string(), rx);
    }
}