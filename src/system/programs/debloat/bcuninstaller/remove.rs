// DOCS: https://htmlpreview.github.io/?https://github.com/Klocman/Bulk-Crap-Uninstaller/blob/master/doc/BCU_manual.html#__RefHeading___Toc524_1610471585

use crate::system::utils::commands::templates;

// TODO: Find the path to executable dynamically
// TODO: Add other removal methods
// TODO: Handle errors when app is already installed

pub fn storehelper(program_id: &str) {
    templates::admin(
        "run",
        "success",
        "error",
        format!("StoreAppHelper /uninstall {}", program_id).as_str(),
    );
}
