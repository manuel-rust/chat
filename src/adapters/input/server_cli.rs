use std::io::{self, Write};
use crate::domain::ports::message_storage_port::MessageStoragePort;

/// Adaptador de entrada: Servidor CLI
pub struct ServerCliAdapter;

impl ServerCliAdapter {
    /// Ejecuta el servidor CLI
    pub fn run(storage: &dyn MessageStoragePort) -> Result<(), String> {
        println!("Chat Server Started");
        println!("Listening for messages...");
        println!("Type /status to show stats");
        println!("Type /exit to stop server\n");

        loop {
            print!("server> ");
            io::stdout().flush().ok();

            let mut input = String::new();
            io::stdin().read_line(&mut input)
                .map_err(|e| format!("Failed to read input: {}", e))?;

            let input = input.trim();

            if input == "/status" {
                Self::handle_status(storage)?;
            } else if input == "/exit" {
                println!("Server shutting down...");
                break;
            } else if !input.is_empty() {
                println!("Unknown command. Type /status or /exit");
            }
        }

        Ok(())
    }

    /// Muestra el estado del servidor
    fn handle_status(storage: &dyn MessageStoragePort) -> Result<(), String> {
        match storage.get_all_messages() {
            Ok(messages) => {
                println!("\n--- Server Status ---");
                println!("Total messages: {}", messages.len());
                if !messages.is_empty() {
                    let users: std::collections::HashSet<_> = 
                        messages.iter().map(|m| &m.sender).collect();
                    println!("Active users: {}", users.len());
                    println!("Users: {:?}", users);
                }
                println!("--------------------\n");
            }
            Err(e) => println!("Error getting status: {}", e),
        }

        Ok(())
    }
}
