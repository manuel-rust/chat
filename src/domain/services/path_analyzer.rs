use crate::domain::models::path_stats::PathStats;
use crate::domain::ports::file_system_port::FileSystemPort;

/// Servicio de dominio: Analiza rutas
pub struct PathAnalyzerService;

impl PathAnalyzerService {
    /// Analiza una ruta usando el puerto de acceso a sistema de archivos
    pub fn analyze(
        fs_port: &dyn FileSystemPort,
        path: &str,
    ) -> Result<PathStats, String> {
        let (files, directories) = fs_port.get_path_stats(path)?;
        Ok(PathStats::new(path.to_string(), files, directories))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Mock para testing del FileSystemPort
    struct MockFileSystem {
        files: u64,
        directories: u64,
        should_fail: bool,
    }

    impl MockFileSystem {
        fn new(files: u64, directories: u64) -> Self {
            MockFileSystem {
                files,
                directories,
                should_fail: false,
            }
        }

        fn with_error() -> Self {
            MockFileSystem {
                files: 0,
                directories: 0,
                should_fail: true,
            }
        }
    }

    impl FileSystemPort for MockFileSystem {
        fn get_path_stats(&self, _path: &str) -> Result<(u64, u64), String> {
            if self.should_fail {
                Err("Simulated error: path not found".to_string())
            } else {
                Ok((self.files, self.directories))
            }
        }
    }

    #[test]
    fn test_analyze_with_files_and_directories() {
        let mock = MockFileSystem::new(10, 5);
        let result = PathAnalyzerService::analyze(&mock, "/test/path");

        assert!(result.is_ok());
        let stats = result.unwrap();
        assert_eq!(stats.path, "/test/path");
        assert_eq!(stats.files, 10);
        assert_eq!(stats.directories, 5);
        assert_eq!(stats.total(), 15);
    }

    #[test]
    fn test_analyze_empty_directory() {
        let mock = MockFileSystem::new(0, 0);
        let result = PathAnalyzerService::analyze(&mock, "/empty");

        assert!(result.is_ok());
        let stats = result.unwrap();
        assert_eq!(stats.files, 0);
        assert_eq!(stats.directories, 0);
        assert_eq!(stats.total(), 0);
    }

    #[test]
    fn test_analyze_only_files() {
        let mock = MockFileSystem::new(42, 0);
        let result = PathAnalyzerService::analyze(&mock, "/documents");

        assert!(result.is_ok());
        let stats = result.unwrap();
        assert_eq!(stats.files, 42);
        assert_eq!(stats.directories, 0);
        assert_eq!(stats.total(), 42);
    }

    #[test]
    fn test_analyze_only_directories() {
        let mock = MockFileSystem::new(0, 8);
        let result = PathAnalyzerService::analyze(&mock, "/folders");

        assert!(result.is_ok());
        let stats = result.unwrap();
        assert_eq!(stats.files, 0);
        assert_eq!(stats.directories, 8);
        assert_eq!(stats.total(), 8);
    }

    #[test]
    fn test_analyze_with_error() {
        let mock = MockFileSystem::with_error();
        let result = PathAnalyzerService::analyze(&mock, "/nonexistent");

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Simulated error: path not found".to_string()
        );
    }

    #[test]
    fn test_path_stats_total() {
        let stats = PathStats::new("/test".to_string(), 15, 3);
        assert_eq!(stats.total(), 18);
    }
}
