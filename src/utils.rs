// utils.rs
pub fn format_error(e: &dyn std::error::Error) -> String {
    format!("Error: {}", e)
}
