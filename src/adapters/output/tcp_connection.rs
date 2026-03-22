use crate::domain::models::message::Message;
use crate::domain::ports::server_connection_port::ServerConnectionPort;
use crate::domain::ports::message_storage_port::MessageStoragePort;
use serde_json::json;

const SERVER_URL: &str = "http://127.0.0.1:8080";

/// Adaptador de salida: Implementa conexión HTTP para cliente-servidor
pub struct HttpConnectionAdapter;

impl HttpConnectionAdapter {
    /// Crea un nuevo adaptador HTTP
    pub fn new() -> Self {
        HttpConnectionAdapter
    }

    /// Inicia el servidor HTTP
    pub fn start_server(storage: &dyn MessageStoragePort) -> Result<(), String> {
        let server = tiny_http::Server::http("127.0.0.1:8080")
            .map_err(|e| format!("Failed to bind server: {}", e))?;

        println!("Server listening on port 8080");
        println!("HTTP endpoints:");
        println!("  POST   http://localhost:8080/message - Send a message");
        println!("  GET    http://localhost:8080/messages - Get all messages");
        println!("  GET    http://localhost:8080/messages?count=N - Get last N messages\n");

        for request in server.incoming_requests() {
            let method = request.method().to_string();
            let path = request.url().to_string();

            match (method.as_str(), path.as_str()) {
                ("POST", "/message") => {
                    handle_post_message(request, storage);
                }
                ("GET", url) if url.starts_with("/messages") => {
                    handle_get_messages(request, storage, url);
                }
                _ => {
                    let response = tiny_http::Response::from_string("Not Found").with_status_code(404);
                    let _ = request.respond(response);
                }
            }
        }

        Ok(())
    }

    /// Envía un mensaje al servidor vía HTTP
    pub async fn send_message_http(sender: &str, content: &str) -> Result<Message, String> {
        let client = reqwest::Client::new();
        let response = client
            .post(format!("{}/message", SERVER_URL))
            .json(&json!({ "sender": sender, "content": content }))
            .send()
            .await
            .map_err(|e| format!("Failed to send message: {}", e))?;

        if response.status().is_success() {
            response.json().await.map_err(|e| format!("Failed to parse response: {}", e))
        } else {
            Err(format!("Server error: {}", response.status()))
        }
    }

    /// Obtiene todos los mensajes desde el servidor vía HTTP
    pub async fn fetch_all_messages_http() -> Result<Vec<Message>, String> {
        let client = reqwest::Client::new();
        let response = client
            .get(format!("{}/messages", SERVER_URL))
            .send()
            .await
            .map_err(|e| format!("Failed to fetch messages: {}", e))?;

        if response.status().is_success() {
            response.json().await.map_err(|e| format!("Failed to parse response: {}", e))
        } else {
            Err(format!("Server error: {}", response.status()))
        }
    }

    /// Obtiene últimos N mensajes desde el servidor vía HTTP
    pub async fn fetch_last_messages_http(count: usize) -> Result<Vec<Message>, String> {
        let client = reqwest::Client::new();
        let response = client
            .get(format!("{}/messages?count={}", SERVER_URL, count))
            .send()
            .await
            .map_err(|e| format!("Failed to fetch messages: {}", e))?;

        if response.status().is_success() {
            response.json().await.map_err(|e| format!("Failed to parse response: {}", e))
        } else {
            Err(format!("Server error: {}", response.status()))
        }
    }
}

/// Maneja GET /messages
fn handle_get_messages(
    request: tiny_http::Request,
    storage: &dyn MessageStoragePort,
    url: &str,
) {
    // Parsear parámetros de query: ?count=N
    let count = if let Some(query_start) = url.find('?') {
        let query = &url[query_start + 1..];
        if query.starts_with("count=") {
            query[6..].parse::<usize>().unwrap_or(10)
        } else {
            10
        }
    } else {
        10
    };

    let result = if count == usize::MAX {
        storage.get_all_messages()
    } else {
        storage.get_last_messages(count)
    };

    match result {
        Ok(messages) => {
            let json = serde_json::to_string(&messages).unwrap_or_else(|_| "[]".to_string());
            let response = tiny_http::Response::from_string(json)
                .with_header(tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap());
            let _ = request.respond(response);
        }
        Err(e) => {
            let response = tiny_http::Response::from_string(format!(r#"{{"error":"{}"}}"#, e))
                .with_status_code(500);
            let _ = request.respond(response);
        }
    }
}

/// Maneja POST /message
fn handle_post_message(mut request: tiny_http::Request, storage: &dyn MessageStoragePort) {
    let mut content = String::new();
    if request.as_reader().read_to_string(&mut content).is_err() {
        let _ = request.respond(tiny_http::Response::from_string("Bad Request").with_status_code(400));
        return;
    }

    match serde_json::from_str::<serde_json::Value>(&content) {
        Ok(json) => {
            let sender = json.get("sender").and_then(|v| v.as_str()).unwrap_or("");
            let msg_content = json.get("content").and_then(|v| v.as_str()).unwrap_or("");

            match Message::new(sender.to_string(), msg_content.to_string()) {
                Ok(message) => {
                    match storage.save_message(&message) {
                        Ok(_) => {
                            let response_json = serde_json::to_string(&message)
                                .unwrap_or_else(|_| "{}".to_string());
                            let response = tiny_http::Response::from_string(response_json)
                                .with_header(tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap())
                                .with_status_code(201);
                            let _ = request.respond(response);
                        }
                        Err(e) => {
                            let response = tiny_http::Response::from_string(format!(r#"{{"error":"{}"}}"#, e))
                                .with_status_code(500);
                            let _ = request.respond(response);
                        }
                    }
                }
                Err(e) => {
                    let response = tiny_http::Response::from_string(format!(r#"{{"error":"{}"}}"#, e))
                        .with_status_code(400);
                    let _ = request.respond(response);
                }
            }
        }
        Err(_) => {
            let response = tiny_http::Response::from_string("Invalid JSON").with_status_code(400);
            let _ = request.respond(response);
        }
    }
}

impl ServerConnectionPort for HttpConnectionAdapter {
    fn start_server(&self, _port: u16) -> Result<(), String> {
        // El puerto se ignora, siempre usa 8080
        Err("Use HttpConnectionAdapter::start_server() instead".to_string())
    }

    fn send_message(&self, _message: &Message) -> Result<(), String> {
        Err("Use HttpConnectionAdapter::send_message_http() instead".to_string())
    }

    fn receive_message(&self) -> Result<Message, String> {
        Err("HTTP is stateless, use fetch_messages_http() instead".to_string())
    }

    fn close(&self) -> Result<(), String> {
        Ok(())
    }
}
