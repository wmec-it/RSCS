use rscs::core::command::powershell;

pub fn appx_package(program_id: &str) {
    powershell::admin(
        "Deleting Windows APPX Package...",
        "Successfully deleted Windows APPX Package!",
        "Error deleting Windows APPX Package :(",
        format!("Remove-AppxPackage -package {} -confirm:$false", program_id).as_str(),
    );
}
