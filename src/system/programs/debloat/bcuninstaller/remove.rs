// DOCS: https://htmlpreview.github.io/?https://github.com/Klocman/Bulk-Crap-Uninstaller/blob/master/doc/BCU_manual.html#__RefHeading___Toc524_1610471585

use crate::system::utils::commands::templates;

// TODO: Find the path to executable dynamically
// TODO: Add other removal methods
// TODO: Handle errors when app is already installed

#[allow(dead_code)]
pub fn storehelper(program_id: &str) {
    templates::admin(
        "Using BCUninstaller to uninstall bloatware...",
        "Successfully removed Microsoft's viruses off of your PC :3",
        "Error removing viruses, Microsoft has your current location and is closing in fast OwO",
        format!("StoreAppHelper /uninstall {}", program_id).as_str(),
    );
}
