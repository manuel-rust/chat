use std::env;

/// Adaptador de entrada: Interfaz CLI
pub struct CliInput;

impl CliInput {
    /// Obtiene la ruta desde los argumentos de línea de comandos
    pub fn get_path() -> Result<String, String> {
        let args: Vec<String> = env::args().collect();

        if args.len() < 2 {
            return Err("Usage: cargo run <path>".to_string());
        }

        Ok(args[1].clone())
    }
}
