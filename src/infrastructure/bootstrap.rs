use crate::adapters::input::client_cli::ClientCliAdapter;
use crate::adapters::output::json_storage::JsonStorageAdapter;
use crate::adapters::output::tcp_connection::HttpConnectionAdapter;
use std::env;

const CHAT_STORAGE_FILE: &str = "data/chat_messages.json";

/// Bootstrap: Configura e inyecta las dependencias
pub fn run() {
    let args: Vec<String> = env::args().collect();

    // Detectar modo: server o client (default)
    let mode = if args.len() > 1 {
        args[1].as_str()
    } else {
        "client"
    };

    match mode {
        "server" => run_chat_server(),
        _ => run_chat_client(), // Default: cliente
    }
}

/// Bootstrap para el servidor de chat con HTTP
fn run_chat_server() {
    println!("Starting Chat Server with HTTP...");

    // 1. Inicializar almacenamiento
    let storage = JsonStorageAdapter::new(CHAT_STORAGE_FILE);
    if let Err(e) = storage.init() {
        eprintln!("Error initializing storage: {}", e);
        return;
    }

    // 2. Ejecutar servidor HTTP
    if let Err(e) = HttpConnectionAdapter::start_server(&storage) {
        eprintln!("Server error: {}", e);
    }
}

/// Bootstrap para el cliente de chat
fn run_chat_client() {
    println!("Starting Chat Client (HTTP)...");

    if let Err(e) = ClientCliAdapter::run_http() {
        eprintln!("Client error: {}", e);
    }
}
