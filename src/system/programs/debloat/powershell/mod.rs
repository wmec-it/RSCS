use crate::system::utils::commands::templates;

pub fn appx_package(program_id: &str) {
    templates::admin(
        "Deleting Windows APPX Package...",
        "Successfully deleted Windows APPX Package!",
        "Error deleting Windows APPX Package :(",
        format!("Remove-AppxPackage -package {} -confirm:$false", program_id).as_str(),
    );
}
