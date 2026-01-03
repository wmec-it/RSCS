use crate::system::utils::commands::templates;

pub fn restart() {
    templates::default(
        "Restarting explorer...",
        "Successfully restarted Explorer!",
        "Failed to restart Explorer...",
        "Stop-Process -Name explorer -Force; Start-Process explorer",
    );
}
