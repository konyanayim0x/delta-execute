pub mod executor {
    use std::process::Command;

    pub fn run_script(script: &str) -> Result<String, String> {
        let output = Command::new("roblox_exe_path")
            .arg("-execute")
            .arg(script)
            .output()
            .map_err(|e| e.to_string())?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).to_string())
        }
    }
}