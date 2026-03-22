use crate::adapters::input::cli::CliInput;
use crate::adapters::input::cli_output::CliOutput;
use crate::adapters::output::file_system::FileSystemAdapter;
use crate::application::use_cases::analyze_path::AnalyzePathUseCase;

/// Bootstrap: Configura e inyecta las dependencias
pub fn run() {
    // 1. Obtener entrada desde CLI
    let path = match CliInput::get_path() {
        Ok(p) => p,
        Err(e) => {
            CliOutput::print_error(&e);
            return;
        }
    };

    // 2. Crear adaptador del puerto
    let fs_adapter = FileSystemAdapter;

    // 3. Ejecutar caso de uso
    match AnalyzePathUseCase::execute(&fs_adapter, &path) {
        Ok(stats) => CliOutput::print_stats(&stats),
        Err(e) => CliOutput::print_error(&e),
    }
}
