use crate::domain::models::message::Message;

/// Puerto (Interfaz) para gestionar conexiones servidor-cliente
pub trait ServerConnectionPort {
    /// Abre/inicia el servidor en un puerto específico
    fn start_server(&self, port: u16) -> Result<(), String>;

    /// Envía un mensaje a través de la conexión
    fn send_message(&self, message: &Message) -> Result<(), String>;

    /// Recibe un mensaje de la conexión
    fn receive_message(&self) -> Result<Message, String>;

    /// Cierra la conexión
    fn close(&self) -> Result<(), String>;
}
