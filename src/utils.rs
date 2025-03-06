pub fn validate_script(script: &str) -> bool {
    !script.is_empty() && script.len() < 500
}