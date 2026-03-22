/// Puerto (Interfaz) para acceder al sistema de archivos
pub trait FileSystemPort {
    /// Obtiene las estadísticas de una ruta
    fn get_path_stats(&self, path: &str) -> Result<(u64, u64), String>;
}
