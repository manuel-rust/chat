use crate::domain::models::message::Message;
use crate::domain::ports::message_storage_port::MessageStoragePort;

/// Servicio de dominio: Orquesta la lógica del chat
pub struct ChatService;

impl ChatService {
    /// Envía un mensaje (validación + almacenamiento)
    pub fn send_message(
        storage: &dyn MessageStoragePort,
        sender: &str,
        content: &str,
    ) -> Result<Message, String> {
        // 1. Crear el mensaje (incluye validación básica)
        let message = Message::new(sender.to_string(), content.to_string())?;

        // 2. Guardar en almacenamiento
        storage.save_message(&message)?;

        Ok(message)
    }

    /// Obtiene todos los mensajes
    pub fn fetch_all_messages(
        storage: &dyn MessageStoragePort,
    ) -> Result<Vec<Message>, String> {
        storage.get_all_messages()
    }

    /// Obtiene los últimos N mensajes
    pub fn fetch_last_messages(
        storage: &dyn MessageStoragePort,
        count: usize,
    ) -> Result<Vec<Message>, String> {
        storage.get_last_messages(count)
    }

    /// Obtiene mensajes después de un timestamp
    pub fn fetch_messages_since(
        storage: &dyn MessageStoragePort,
        timestamp: u64,
    ) -> Result<Vec<Message>, String> {
        storage.get_messages_since(timestamp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Mock para testing
    struct MockStorage {
        messages: std::sync::Arc<std::sync::Mutex<Vec<Message>>>,
    }

    impl MockStorage {
        fn new() -> Self {
            MockStorage {
                messages: std::sync::Arc::new(std::sync::Mutex::new(Vec::new())),
            }
        }
    }

    impl MessageStoragePort for MockStorage {
        fn save_message(&self, message: &Message) -> Result<(), String> {
            let mut msgs = self.messages.lock().unwrap();
            msgs.push(message.clone());
            Ok(())
        }

        fn get_all_messages(&self) -> Result<Vec<Message>, String> {
            let msgs = self.messages.lock().unwrap();
            Ok(msgs.clone())
        }

        fn get_messages_since(&self, timestamp: u64) -> Result<Vec<Message>, String> {
            let msgs = self.messages.lock().unwrap();
            Ok(msgs.iter().filter(|m| m.timestamp >= timestamp).cloned().collect())
        }

        fn get_last_messages(&self, count: usize) -> Result<Vec<Message>, String> {
            let msgs = self.messages.lock().unwrap();
            let len = msgs.len();
            let start = if len > count { len - count } else { 0 };
            Ok(msgs[start..].to_vec())
        }
    }

    #[test]
    fn test_send_message() {
        let storage = MockStorage::new();
        let result = ChatService::send_message(&storage, "alice", "Hello");
        assert!(result.is_ok());
        
        let messages = storage.get_all_messages().unwrap();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].sender, "alice");
        assert_eq!(messages[0].content, "Hello");
    }

    #[test]
    fn test_fetch_all_messages() {
        let storage = MockStorage::new();
        ChatService::send_message(&storage, "alice", "Hello").unwrap();
        ChatService::send_message(&storage, "bob", "Hi").unwrap();

        let messages = ChatService::fetch_all_messages(&storage).unwrap();
        assert_eq!(messages.len(), 2);
    }

    #[test]
    fn test_send_invalid_sender() {
        let storage = MockStorage::new();
        let result = ChatService::send_message(&storage, "", "Hello");
        assert!(result.is_err());
    }
}
