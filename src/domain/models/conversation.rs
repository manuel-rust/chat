use crate::domain::models::message::Message;
use serde::{Deserialize, Serialize};

/// Entidad del dominio: Conversación (colección de mensajes)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub id: String,
    pub name: String,
    pub messages: Vec<Message>,
}

impl Conversation {
    /// Crea una nueva conversación
    pub fn new(name: String) -> Result<Self, String> {
        if name.trim().is_empty() {
            return Err("Conversation name cannot be empty".to_string());
        }

        Ok(Conversation {
            id: format!("conv-{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()),
            name,
            messages: Vec::new(),
        })
    }

    /// Agrega un mensaje a la conversación
    pub fn add_message(&mut self, message: Message) {
        self.messages.push(message);
    }

    /// Obtiene todos los mensajes ordenados por timestamp
    pub fn get_messages(&self) -> Vec<Message> {
        let mut messages = self.messages.clone();
        messages.sort_by_key(|m| m.timestamp);
        messages
    }

    /// Obtiene los últimos N mensajes
    pub fn get_last_messages(&self, count: usize) -> Vec<Message> {
        let messages = self.get_messages();
        messages.iter().rev().take(count).cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversation_creation() {
        let conv = Conversation::new("General".to_string()).unwrap();
        assert_eq!(conv.name, "General");
        assert!(conv.messages.is_empty());
    }

    #[test]
    fn test_add_message() {
        let mut conv = Conversation::new("General".to_string()).unwrap();
        let msg = Message::new("alice".to_string(), "Hello".to_string()).unwrap();
        conv.add_message(msg.clone());
        assert_eq!(conv.messages.len(), 1);
        assert_eq!(conv.messages[0], msg);
    }

    #[test]
    fn test_conversation_empty_name() {
        let result = Conversation::new("".to_string());
        assert!(result.is_err());
    }
}
