use std::fs;
use crate::domain::ports::file_system_port::FileSystemPort;

/// Adaptador de salida: Implementa el acceso real al sistema de archivos
pub struct FileSystemAdapter;

impl FileSystemPort for FileSystemAdapter {
    fn get_path_stats(&self, path: &str) -> Result<(u64, u64), String> {
        let entries = fs::read_dir(path)
            .map_err(|e| format!("Error reading path: {}", e))?;

        let mut files = 0u64;
        let mut directories = 0u64;

        for entry in entries.flatten() {
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_file() {
                    files += 1;
                } else if metadata.is_dir() {
                    directories += 1;
                }
            }
        }

        Ok((files, directories))
    }
}
