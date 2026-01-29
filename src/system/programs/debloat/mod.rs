use crate::conf;

pub mod bcuninstaller;
pub mod powershell;
pub mod manual;

pub fn start() {
    for program_id in conf::vars::DEBLOAT_UNINSTALL_APPX_PACKAGES.appx_packages {
        powershell::appx_package(program_id);
    }
    manual::builtin();
    // TODO: Add the other removal/debloat methods
}
