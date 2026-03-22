use crate::domain::models::path_stats::PathStats;
use crate::domain::ports::file_system_port::FileSystemPort;
use crate::domain::services::path_analyzer::PathAnalyzerService;

/// Caso de uso: Analizar una ruta
pub struct AnalyzePathUseCase;

impl AnalyzePathUseCase {
    pub fn execute(
        fs_port: &dyn FileSystemPort,
        path: &str,
    ) -> Result<PathStats, String> {
        PathAnalyzerService::analyze(fs_port, path)
    }
}
