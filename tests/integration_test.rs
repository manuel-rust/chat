use agenda::adapters::output::file_system::FileSystemAdapter;
use agenda::application::use_cases::analyze_path::AnalyzePathUseCase;

/// Test de integración: Verifica que el caso de uso puede analizar el directorio actual
#[test]
fn test_analyze_current_directory() {
    let adapter = FileSystemAdapter;
    let result = AnalyzePathUseCase::execute(&adapter, ".");

    assert!(result.is_ok());
    let stats = result.unwrap();
    
    // Verificar que obtenemos estadísticas válidas
    assert_eq!(stats.path, ".");
    assert!(stats.total() > 0, "El directorio debe tener al menos un archivo o carpeta");
}

/// Test de integración: Verifica el manejo de rutas inválidas
#[test]
fn test_analyze_nonexistent_path() {
    let adapter = FileSystemAdapter;
    let result = AnalyzePathUseCase::execute(&adapter, "/path/that/does/not/exist/12345");

    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.contains("Error reading path"));
}

/// Test de integración: Verifica que la salida total es la suma de archivos y carpetas
#[test]
fn test_total_calculation() {
    let adapter = FileSystemAdapter;
    let result = AnalyzePathUseCase::execute(&adapter, ".");

    assert!(result.is_ok());
    let stats = result.unwrap();
    assert_eq!(stats.total(), stats.files + stats.directories);
}
