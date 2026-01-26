use crate::system::utils::commands::templates;

pub fn appx_package(program_id: &str) {
    templates::admin(
        "run",
        "success",
        "error",
        format!("Remove-AppxPackage -package {} -confirm:$false", program_id).as_str(),
    );
}
