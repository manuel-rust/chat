use crate::domain::models::message::Message;
use crate::domain::ports::message_storage_port::MessageStoragePort;
use crate::domain::services::chat_service::ChatService;

/// Caso de uso: Obtener mensajes
pub struct FetchMessagesUseCase;

impl FetchMessagesUseCase {
    /// Obtiene todos los mensajes
    pub fn execute_all(
        storage: &dyn MessageStoragePort,
    ) -> Result<Vec<Message>, String> {
        ChatService::fetch_all_messages(storage)
    }

    /// Obtiene los últimos N mensajes
    pub fn execute_last(
        storage: &dyn MessageStoragePort,
        count: usize,
    ) -> Result<Vec<Message>, String> {
        ChatService::fetch_last_messages(storage, count)
    }

    /// Obtiene mensajes después de un timestamp
    pub fn execute_since(
        storage: &dyn MessageStoragePort,
        timestamp: u64,
    ) -> Result<Vec<Message>, String> {
        ChatService::fetch_messages_since(storage, timestamp)
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

        fn add_message(&self, msg: Message) {
            let mut msgs = self.messages.lock().unwrap();
            msgs.push(msg);
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
    fn test_fetch_all_messages() {
        let storage = MockStorage::new();
        storage.add_message(
            Message::with_id_timestamp("alice".to_string(), "Hello".to_string(), "msg1".to_string(), 100).unwrap()
        );
        storage.add_message(
            Message::with_id_timestamp("bob".to_string(), "Hi".to_string(), "msg2".to_string(), 200).unwrap()
        );

        let messages = FetchMessagesUseCase::execute_all(&storage).unwrap();
        assert_eq!(messages.len(), 2);
    }

    #[test]
    fn test_fetch_last_messages() {
        let storage = MockStorage::new();
        storage.add_message(
            Message::with_id_timestamp("alice".to_string(), "Hello".to_string(), "msg1".to_string(), 100).unwrap()
        );
        storage.add_message(
            Message::with_id_timestamp("bob".to_string(), "Hi".to_string(), "msg2".to_string(), 200).unwrap()
        );
        storage.add_message(
            Message::with_id_timestamp("charlie".to_string(), "Hey".to_string(), "msg3".to_string(), 300).unwrap()
        );

        let messages = FetchMessagesUseCase::execute_last(&storage, 2).unwrap();
        assert_eq!(messages.len(), 2);
    }
}
