use rocket::tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use rocket::tokio::sync::Mutex;
use std::collections::HashMap;
use std::sync::Arc;


#[derive(Default)]
pub struct ChatServer {
    session_receiver: Mutex<HashMap<String, Arc<Mutex<UnboundedReceiver<String>>>>>,
    session_sender: Mutex<HashMap<String, UnboundedSender<String>>>,
}

impl ChatServer {
    pub async fn new_session(&self, session_id: &str) {
        let (tx, rx) = mpsc::unbounded_channel::<String>();
        self.session_receiver.lock().await.insert(session_id.to_string(), Arc::new(Mutex::new(rx)));
        self.session_sender.lock().await.insert(session_id.to_string(), tx);
    }
    pub async fn get_session_tx(&self, session_id: &str) -> Option<UnboundedSender<String>> {
        self.session_sender.lock().await.get(session_id).cloned()
    }
    pub async fn get_session_rx(&self, session_id: &str) -> Option<Arc<Mutex<UnboundedReceiver<String>>>> {
        match self.session_receiver.lock().await.get(session_id) {
            Some(dat) => Some(dat.clone()),
            None => None,
        }
    }
}
