use std::fs;
use std::path::Path;
use crate::domain::models::message::Message;
use crate::domain::ports::message_storage_port::MessageStoragePort;

/// Adaptador de salida: Implementa almacenamiento con archivos JSON
pub struct JsonStorageAdapter {
    file_path: String,
}

impl JsonStorageAdapter {
    /// Crea un nuevo adaptador de almacenamiento JSON
    pub fn new(file_path: &str) -> Self {
        JsonStorageAdapter {
            file_path: file_path.to_string(),
        }
    }

    /// Inicializa el archivo si no existe
    pub fn init(&self) -> Result<(), String> {
        if !Path::new(&self.file_path).exists() {
            // Crear directorio si es necesario
            if let Some(parent) = Path::new(&self.file_path).parent() {
                if !parent.as_os_str().is_empty() {
                    fs::create_dir_all(parent)
                        .map_err(|e| format!("Failed to create directory: {}", e))?;
                }
            }
            // Crear archivo vacío con array JSON
            fs::write(&self.file_path, "[]")
                .map_err(|e| format!("Failed to create storage file: {}", e))?;
        }
        Ok(())
    }

    /// Lee todos los mensajes del archivo
    fn read_messages(&self) -> Result<Vec<Message>, String> {
        let content = fs::read_to_string(&self.file_path)
            .map_err(|e| format!("Failed to read storage file: {}", e))?;

        serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse JSON: {}", e))
    }

    /// Escribe todos los mensajes al archivo
    fn write_messages(&self, messages: &[Message]) -> Result<(), String> {
        let json = serde_json::to_string_pretty(messages)
            .map_err(|e| format!("Failed to serialize messages: {}", e))?;

        fs::write(&self.file_path, json)
            .map_err(|e| format!("Failed to write storage file: {}", e))
    }
}

impl MessageStoragePort for JsonStorageAdapter {
    fn save_message(&self, message: &Message) -> Result<(), String> {
        let mut messages = self.read_messages()?;
        messages.push(message.clone());
        self.write_messages(&messages)
    }

    fn get_all_messages(&self) -> Result<Vec<Message>, String> {
        self.read_messages()
    }

    fn get_messages_since(&self, timestamp: u64) -> Result<Vec<Message>, String> {
        let messages = self.read_messages()?;
        Ok(messages.into_iter().filter(|m| m.timestamp >= timestamp).collect())
    }

    fn get_last_messages(&self, count: usize) -> Result<Vec<Message>, String> {
        let messages = self.read_messages()?;
        let len = messages.len();
        let start = len.saturating_sub(count);
        Ok(messages[start..].to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_init_creates_file() {
        let test_file = "test_messages_init.json";
        if Path::new(test_file).exists() {
            fs::remove_file(test_file).unwrap();
        }

        let adapter = JsonStorageAdapter::new(test_file);
        assert!(adapter.init().is_ok());
        assert!(Path::new(test_file).exists());

        fs::remove_file(test_file).unwrap();
    }

    #[test]
    fn test_save_and_read_message() {
        let test_file = "test_messages_save.json";
        if Path::new(test_file).exists() {
            fs::remove_file(test_file).unwrap();
        }

        let adapter = JsonStorageAdapter::new(test_file);
        adapter.init().unwrap();

        let msg = Message::new("alice".to_string(), "Hello".to_string()).unwrap();
        assert!(adapter.save_message(&msg).is_ok());

        let messages = adapter.get_all_messages().unwrap();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].sender, "alice");

        fs::remove_file(test_file).unwrap();
    }
}
