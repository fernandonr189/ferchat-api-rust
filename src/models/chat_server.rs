use rocket::tokio::sync::mpsc;
use rocket::tokio::sync::Mutex;
use std::collections::HashMap;
use std::sync::Arc;


pub struct ChatSession {
    pub sender: mpsc::UnboundedSender<String>,
    pub receiver: mpsc::UnboundedReceiver<String>,
}

#[derive(Default)]
pub struct ChatServer {
    sessions: Mutex<HashMap<String, (Arc<Mutex<ChatSession>>, Arc<Mutex<ChatSession>>)>>,
}

impl ChatServer {
    pub async fn new_session(&self, session_id: &str) -> (Arc<Mutex<ChatSession>>, Arc<Mutex<ChatSession>>) {
        let (atx, arx) = mpsc::unbounded_channel();
        let (btx, brx) = mpsc::unbounded_channel();
        let session_a = Arc::new(Mutex::new(ChatSession {
            sender: atx,
            receiver: arx,
        }));
        let session_b = Arc::new(Mutex::new(ChatSession {
            sender: btx,
            receiver: brx,
        }));
        self.sessions
            .lock()
            .await
            .insert(session_id.to_string(), (Arc::clone(&session_a), Arc::clone(&session_b)));
        (session_a, session_b)
    }
    pub async fn get_session(
        &self,
        session_id: &str,
    ) -> Option<(Arc<Mutex<ChatSession>>, Arc<Mutex<ChatSession>>)> {
        self.sessions.lock().await.get(session_id).cloned()
    }
}
