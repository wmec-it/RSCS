use super::super::command::powershell;

pub fn restart() {
    powershell::default(
        "Restarting explorer...",
        "Successfully restarted Explorer!",
        "Failed to restart Explorer...",
        "Stop-Process -Name explorer -Force; Start-Process explorer",
    );
}
