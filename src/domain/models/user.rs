use serde::{Deserialize, Serialize};

/// Entidad del dominio: Usuario en el sistema de chat
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct User {
    pub id: String,
    pub username: String,
}

impl User {
    /// Crea un nuevo usuario
    pub fn new(username: String) -> Result<Self, String> {
        if username.trim().is_empty() {
            return Err("Username cannot be empty".to_string());
        }

        Ok(User {
            id: format!("user-{}", uuid_simple::generate()),
            username,
        })
    }

    /// Crea un usuario con ID específico (para testing)
    pub fn with_id(username: String, id: String) -> Result<Self, String> {
        if username.trim().is_empty() {
            return Err("Username cannot be empty".to_string());
        }

        Ok(User { id, username })
    }
}

// Simple UUID generator sin dependencias
mod uuid_simple {
    use std::time::SystemTime;

    pub fn generate() -> String {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        format!("{:x}", timestamp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation() {
        let user = User::new("alice".to_string()).unwrap();
        assert_eq!(user.username, "alice");
        assert!(!user.id.is_empty());
    }

    #[test]
    fn test_user_empty_username() {
        let result = User::new("".to_string());
        assert!(result.is_err());
    }
}
