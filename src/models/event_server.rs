use rocket::tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel};
use rocket::tokio::sync::Mutex;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct EventServer {
    event_receiver: Mutex<HashMap<i32, UnboundedReceiver<Event>>>,
    event_sender: Mutex<HashMap<i32, UnboundedSender<Event>>>
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
    pub async fn new_session(&self, session_id: &i32) {
        let (tx, rx) = unbounded_channel::<Event>();
        self.event_sender.lock().await.insert(*session_id, tx);
        self.event_receiver.lock().await.insert(*session_id, rx);
    }

    pub async fn get_session_tx(&self, session_id: &i32) -> Option<UnboundedSender<Event>> {
        self.event_sender.lock().await.get(session_id).cloned()
    }
    pub async fn get_session_rx(&self, session_id: &i32) -> Option<&UnboundedReceiver<Event>> {
        self.event_receiver.lock().await.get(session_id)
    }
}