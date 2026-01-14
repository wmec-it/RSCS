use crate::{AppContext, system::utils::commands::templates};

pub fn restart(ctx: &mut AppContext) {
    templates::default(
        ctx,
        "Restarting explorer...",
        "Successfully restarted Explorer!",
        "Failed to restart Explorer...",
        "Stop-Process -Name explorer -Force; Start-Process explorer",
    );
}
