use crate::system::utils::templates;

pub fn restart() {
    templates::default(
        "Restarting explorer...",
        "Successfully restarted Explorer!",
        "Failed to restart Explorer...",
        "Stop-Process -Name explorer -Force; Start-Process explorer",
    );
}