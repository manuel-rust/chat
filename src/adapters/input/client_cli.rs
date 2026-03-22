use std::io::{self, Write};
use crate::application::use_cases::send_message_use_case::SendMessageUseCase;
use crate::application::use_cases::fetch_messages_use_case::FetchMessagesUseCase;
use crate::domain::ports::message_storage_port::MessageStoragePort;
use crate::adapters::output::tcp_connection::HttpConnectionAdapter;

/// Adaptador de entrada: Cliente CLI interactivo
pub struct ClientCliAdapter;

impl ClientCliAdapter {
    /// Ejecuta el loop interactivo del cliente con HTTP
    pub fn run_http() -> Result<(), String> {
        println!("Chat Client Started (HTTP Mode)");
        println!("Commands:");
        println!("  /send <username> <message> - Send a message");
        println!("  /fetch [count] - Fetch last N messages (default: 10)");
        println!("  /exit - Exit chat");
        println!();

        // Usar runtime de Tokio para operaciones async
        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| format!("Failed to create runtime: {}", e))?;

        rt.block_on(async {
            loop {
                print!("> ");
                io::stdout().flush().ok();

                let mut input = String::new();
                io::stdin().read_line(&mut input)
                    .map_err(|e| format!("Failed to read input: {}", e))?;

                let input = input.trim();

                if input.starts_with("/send ") {
                    Self::handle_send_http(input).await?;
                } else if input.starts_with("/fetch") {
                    Self::handle_fetch_http(input).await?;
                } else if input == "/exit" {
                    println!("Goodbye!");
                    break;
                } else if !input.is_empty() {
                    println!("Unknown command. Type /send <username> <message> or /fetch");
                }
            }
            Ok::<(), String>(())
        })
    }

    /// Maneja el comando /send vía HTTP
    async fn handle_send_http(input: &str) -> Result<(), String> {
        // Formato: /send <username> "message"
        let input = input.trim_start_matches("/send").trim();
        
        // Encontrar el primer espacio para separar username del mensaje
        let parts: Vec<&str> = input.splitn(2, ' ').collect();
        
        if parts.len() < 2 {
            return Err("Usage: /send <username> \"message\"".to_string());
        }

        let username = parts[0];
        let message_part = parts[1].trim();
        
        // Si el mensaje está entre comillas, removerlas
        let message = if (message_part.starts_with('"') && message_part.ends_with('"')) ||
                         (message_part.starts_with('\'') && message_part.ends_with('\'')) {
            &message_part[1..message_part.len() - 1]
        } else {
            message_part
        };

        match HttpConnectionAdapter::send_message_http(username, message).await {
            Ok(msg) => println!("Message sent: {} [{}]", msg.sender, msg.id),
            Err(e) => println!("Error sending message: {}", e),
        }

        Ok(())
    }

    /// Maneja el comando /fetch vía HTTP
    async fn handle_fetch_http(input: &str) -> Result<(), String> {
        let parts: Vec<&str> = input.split_whitespace().collect();
        let count = if parts.len() > 1 {
            parts[1].parse::<usize>().unwrap_or(10)
        } else {
            10
        };

        match HttpConnectionAdapter::fetch_last_messages_http(count).await {
            Ok(messages) => {
                if messages.is_empty() {
                    println!("No messages found");
                } else {
                    println!("\n--- Last {} messages ---", messages.len());
                    for msg in messages {
                        println!("[{}] {}: {}", msg.id, msg.sender, msg.content);
                    }
                    println!("------------------------\n");
                }
            }
            Err(e) => println!("Error fetching messages: {}", e),
        }

        Ok(())
    }

    /// Ejecuta el loop interactivo del cliente (versión local con JSON)
    pub fn run(storage: &dyn MessageStoragePort) -> Result<(), String> {
        println!("Chat Client Started");
        println!("Commands:");
        println!("  /send <username> <message> - Send a message");
        println!("  /fetch [count] - Fetch last N messages (default: 10)");
        println!("  /exit - Exit chat");
        println!();

        loop {
            print!("> ");
            io::stdout().flush().ok();

            let mut input = String::new();
            io::stdin().read_line(&mut input)
                .map_err(|e| format!("Failed to read input: {}", e))?;

            let input = input.trim();

            if input.starts_with("/send ") {
                Self::handle_send(storage, input)?;
            } else if input.starts_with("/fetch") {
                Self::handle_fetch(storage, input)?;
            } else if input == "/exit" {
                println!("Goodbye!");
                break;
            } else if !input.is_empty() {
                println!("Unknown command. Type /send <username> <message> or /fetch");
            }
        }

        Ok(())
    }

    /// Maneja el comando /send
    fn handle_send(storage: &dyn MessageStoragePort, input: &str) -> Result<(), String> {
        // Formato: /send <username> "message"
        let input = input.trim_start_matches("/send").trim();
        
        // Encontrar el primer espacio para separar username del mensaje
        let parts: Vec<&str> = input.splitn(2, ' ').collect();
        
        if parts.len() < 2 {
            return Err("Usage: /send <username> \"message\"".to_string());
        }

        let username = parts[0];
        let message_part = parts[1].trim();
        
        // Si el mensaje está entre comillas, removerlas
        let message = if (message_part.starts_with('"') && message_part.ends_with('"')) ||
                         (message_part.starts_with('\'') && message_part.ends_with('\'')) {
            &message_part[1..message_part.len() - 1]
        } else {
            message_part
        };

        match SendMessageUseCase::execute(storage, username, message) {
            Ok(msg) => println!("Message sent: {} [{}]", msg.sender, msg.id),
            Err(e) => println!("Error sending message: {}", e),
        }

        Ok(())
    }

    /// Maneja el comando /fetch
    fn handle_fetch(storage: &dyn MessageStoragePort, input: &str) -> Result<(), String> {
        let parts: Vec<&str> = input.split_whitespace().collect();
        let count = if parts.len() > 1 {
            parts[1].parse::<usize>().unwrap_or(10)
        } else {
            10
        };

        match FetchMessagesUseCase::execute_last(storage, count) {
            Ok(messages) => {
                if messages.is_empty() {
                    println!("No messages found");
                } else {
                    println!("\n--- Last {} messages ---", messages.len());
                    for msg in messages {
                        println!("[{}] {}: {}", msg.id, msg.sender, msg.content);
                    }
                    println!("------------------------\n");
                }
            }
            Err(e) => println!("Error fetching messages: {}", e),
        }

        Ok(())
    }
}
