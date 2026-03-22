use crate::domain::models::message::Message;
use crate::domain::ports::message_storage_port::MessageStoragePort;
use crate::domain::services::chat_service::ChatService;

/// Caso de uso: Enviar un mensaje
pub struct SendMessageUseCase;

impl SendMessageUseCase {
    /// Ejecuta el caso de uso de enviar mensaje
    pub fn execute(
        storage: &dyn MessageStoragePort,
        sender: &str,
        content: &str,
    ) -> Result<Message, String> {
        // Delega al servicio de dominio
        ChatService::send_message(storage, sender, content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_send_message_use_case() {
        let storage = MockStorage::new();
        let result = SendMessageUseCase::execute(&storage, "alice", "Hello World");
        assert!(result.is_ok());
        
        let msg = result.unwrap();
        assert_eq!(msg.sender, "alice");
        assert_eq!(msg.content, "Hello World");
    }
}
