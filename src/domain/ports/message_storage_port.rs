use crate::domain::models::message::Message;

/// Puerto (Interfaz) para acceder al almacenamiento de mensajes
pub trait MessageStoragePort {
    /// Guarda un mensaje en el almacenamiento
    fn save_message(&self, message: &Message) -> Result<(), String>;

    /// Obtiene todos los mensajes del almacenamiento
    fn get_all_messages(&self) -> Result<Vec<Message>, String>;

    /// Obtiene mensajes después de un timestamp específico
    fn get_messages_since(&self, timestamp: u64) -> Result<Vec<Message>, String>;

    /// Obtiene los últimos N mensajes
    fn get_last_messages(&self, count: usize) -> Result<Vec<Message>, String>;
}
