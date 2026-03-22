use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};

/// Entidad del dominio: Mensaje en una conversación
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Message {
    pub id: String,
    pub sender: String,
    pub content: String,
    pub timestamp: u64,
}

impl Message {
    /// Crea un nuevo mensaje con ID y timestamp actuales
    pub fn new(sender: String, content: String) -> Result<Self, String> {
        if sender.trim().is_empty() {
            return Err("Sender cannot be empty".to_string());
        }
        if content.trim().is_empty() {
            return Err("Content cannot be empty".to_string());
        }

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| format!("Time error: {}", e))?
            .as_secs();

        let id = format!("{}-{}", sender, timestamp);

        Ok(Message {
            id,
            sender,
            content,
            timestamp,
        })
    }

    /// Crea un mensaje con ID y timestamp específicos (para testing)
    pub fn with_id_timestamp(sender: String, content: String, id: String, timestamp: u64) -> Result<Self, String> {
        if sender.trim().is_empty() {
            return Err("Sender cannot be empty".to_string());
        }
        if content.trim().is_empty() {
            return Err("Content cannot be empty".to_string());
        }

        Ok(Message {
            id,
            sender,
            content,
            timestamp,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_creation() {
        let msg = Message::new("alice".to_string(), "Hello".to_string()).unwrap();
        assert_eq!(msg.sender, "alice");
        assert_eq!(msg.content, "Hello");
        assert!(msg.timestamp > 0);
    }

    #[test]
    fn test_message_empty_sender() {
        let result = Message::new("".to_string(), "Hello".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_message_empty_content() {
        let result = Message::new("alice".to_string(), "".to_string());
        assert!(result.is_err());
    }
}
