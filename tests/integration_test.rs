use chat::adapters::output::json_storage::JsonStorageAdapter;
use chat::application::use_cases::send_message_use_case::SendMessageUseCase;
use chat::application::use_cases::fetch_messages_use_case::FetchMessagesUseCase;
use chat::domain::models::message::Message;
use std::fs;

/// Test de integración: Flujo completo de envío y recuperación de mensajes
#[test]
fn test_send_and_fetch_messages() {
    let test_file = "test_integration_chat.json";
    
    // Limpiar archivo de test si existe
    if std::path::Path::new(test_file).exists() {
        fs::remove_file(test_file).unwrap();
    }

    let storage = JsonStorageAdapter::new(test_file);
    storage.init().unwrap();

    // Enviar varios mensajes
    let _msg1 = SendMessageUseCase::execute(&storage, "alice", "Hello").unwrap();
    let _msg2 = SendMessageUseCase::execute(&storage, "bob", "Hi there").unwrap();
    let _msg3 = SendMessageUseCase::execute(&storage, "alice", "How are you?").unwrap();

    // Recuperar todos los mensajes
    let all_messages = FetchMessagesUseCase::execute_all(&storage).unwrap();
    assert_eq!(all_messages.len(), 3);
    assert_eq!(all_messages[0].sender, "alice");
    assert_eq!(all_messages[1].sender, "bob");
    assert_eq!(all_messages[2].sender, "alice");

    // Recuperar últimos 2 mensajes
    let last_two = FetchMessagesUseCase::execute_last(&storage, 2).unwrap();
    assert_eq!(last_two.len(), 2);
    assert_eq!(last_two[0].sender, "bob");
    assert_eq!(last_two[1].sender, "alice");

    // Limpiar
    fs::remove_file(test_file).unwrap();
}

/// Test de integración: Persistencia en archivo JSON
#[test]
fn test_persistence_across_sessions() {
    let test_file = "test_persistence_chat.json";
    
    // Limpiar archivo de test si existe
    if std::path::Path::new(test_file).exists() {
        fs::remove_file(test_file).unwrap();
    }

    // Primera "sesión"
    {
        let storage = JsonStorageAdapter::new(test_file);
        storage.init().unwrap();
        
        SendMessageUseCase::execute(&storage, "alice", "Message 1").unwrap();
        SendMessageUseCase::execute(&storage, "bob", "Message 2").unwrap();
    }

    // Verificar que el archivo tiene 2 mensajes
    let content = fs::read_to_string(test_file).unwrap();
    let messages: Vec<Message> = serde_json::from_str(&content).unwrap();
    assert_eq!(messages.len(), 2);

    // Segunda "sesión"
    {
        let storage = JsonStorageAdapter::new(test_file);
        
        // Los mensajes anteriores deben estar ahí
        let existing = FetchMessagesUseCase::execute_all(&storage).unwrap();
        assert_eq!(existing.len(), 2);

        // Agregar uno más
        SendMessageUseCase::execute(&storage, "charlie", "Message 3").unwrap();
    }

    // Verificar que ahora hay 3
    let final_content = fs::read_to_string(test_file).unwrap();
    let final_messages: Vec<Message> = serde_json::from_str(&final_content).unwrap();
    assert_eq!(final_messages.len(), 3);

    // Limpiar
    fs::remove_file(test_file).unwrap();
}

/// Test de integración: Validación de mensajes vacíos
#[test]
fn test_reject_empty_messages() {
    let test_file = "test_empty_messages.json";
    
    if std::path::Path::new(test_file).exists() {
        fs::remove_file(test_file).unwrap();
    }

    let storage = JsonStorageAdapter::new(test_file);
    storage.init().unwrap();

    // Intentar enviar mensaje vacío
    let result = SendMessageUseCase::execute(&storage, "alice", "");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Content cannot be empty"));

    // Intentar enviar con usuario vacío
    let result = SendMessageUseCase::execute(&storage, "", "Hello");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Sender cannot be empty"));

    // Verificar que no se guardó nada
    let messages = FetchMessagesUseCase::execute_all(&storage).unwrap();
    assert_eq!(messages.len(), 0);

    fs::remove_file(test_file).unwrap();
}

/// Test de integración: Fetch de mensajes por timestamp
#[test]
fn test_fetch_messages_since_timestamp() {
    let test_file = "test_timestamp_fetch.json";
    
    if std::path::Path::new(test_file).exists() {
        fs::remove_file(test_file).unwrap();
    }

    let storage = JsonStorageAdapter::new(test_file);
    storage.init().unwrap();

    let msg1 = SendMessageUseCase::execute(&storage, "alice", "Message 1").unwrap();
    let msg2 = SendMessageUseCase::execute(&storage, "bob", "Message 2").unwrap();
    let _msg3 = SendMessageUseCase::execute(&storage, "charlie", "Message 3").unwrap();

    // Fetch mensajes después del timestamp del primer mensaje
    let since_msg1 = FetchMessagesUseCase::execute_since(&storage, msg1.timestamp).unwrap();
    assert!(since_msg1.len() >= 3); // Incluye msg1, msg2, msg3

    // Fetch mensajes después del timestamp del segundo mensaje
    // Nota: Como los mensajes se crean en el mismo segundo, todos tienen el mismo timestamp
    let since_msg2 = FetchMessagesUseCase::execute_since(&storage, msg2.timestamp).unwrap();
    assert!(since_msg2.len() >= 2); // Incluye msg2 y msg3 (al menos)

    fs::remove_file(test_file).unwrap();
}
