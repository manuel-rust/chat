use crate::domain::models::path_stats::PathStats;

/// Adaptador de salida: Imprime en consola
pub struct CliOutput;

impl CliOutput {
    pub fn print_stats(stats: &PathStats) {
        println!("Path: {}", stats.path);
        println!("Files: {}", stats.files);
        println!("Directories: {}", stats.directories);
        println!("Total: {}", stats.total());
    }

    pub fn print_error(error: &str) {
        println!("Error: {}", error);
    }
}
