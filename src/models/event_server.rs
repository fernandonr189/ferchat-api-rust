use rocket::tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use rocket::tokio::sync::Mutex;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Default)]
pub struct EventServer {
    event_receiver: Mutex<HashMap<i32, Arc<Mutex<UnboundedReceiver<Event>>>>>,
    event_sender: Mutex<HashMap<i32, UnboundedSender<Event>>>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Event {
    pub event_type: Type,
    pub data: EventData,
    pub timestamp: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
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
        self.event_receiver
            .lock()
            .await
            .insert(*session_id, Arc::new(Mutex::new(rx)));
    }

    pub async fn get_session_tx(&self, session_id: &i32) -> Option<UnboundedSender<Event>> {
        self.event_sender.lock().await.get(session_id).cloned()
    }
    pub async fn get_session_rx(
        &self,
        session_id: &i32,
    ) -> Option<Arc<Mutex<UnboundedReceiver<Event>>>> {
        self.event_receiver.lock().await.get(session_id).cloned()
    }
}
