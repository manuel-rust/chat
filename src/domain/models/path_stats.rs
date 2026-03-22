/// Entidad del dominio: Estadísticas de una ruta
#[derive(Debug, Clone)]
pub struct PathStats {
    pub path: String,
    pub files: u64,
    pub directories: u64,
}

impl PathStats {
    pub fn new(path: String, files: u64, directories: u64) -> Self {
        PathStats {
            path,
            files,
            directories,
        }
    }

    pub fn total(&self) -> u64 {
        self.files + self.directories
    }
}
