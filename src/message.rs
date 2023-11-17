use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub sender: String,
    pub recipient: String,
    pub content: String,
    pub timestamp: DateTime<Utc>,
    pub message_type: MessageType,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MessageType {
    Text,
    File,
}
