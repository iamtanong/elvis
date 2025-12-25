use std::path::PathBuf;

#[derive(Debug)]
pub struct PrinterOptions {
    pub summary_only: bool,
    pub max_entries: usize,
    pub cwd: PathBuf,
    pub use_color: bool,
}

impl Default for PrinterOptions {
    fn default() -> Self {
        Self {
            summary_only: false,
            max_entries: 50,
            cwd: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
            use_color: true,
        }
    }
}
