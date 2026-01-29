use crate::function::utils::commands::templates;

pub fn restart() {
    powershell::default(
        "Restarting explorer...",
        "Successfully restarted Explorer!",
        "Failed to restart Explorer...",
        "Stop-Process -Name explorer -Force; Start-Process explorer",
    );
}
