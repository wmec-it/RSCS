use crate::types;
use crate::utils::errors;

pub fn install_type(install_type: &str) {
    match install_type {
        "Full Install" => types::install::full(),
        "Install Programs" => types::install::programs(),
        "Remove Installed Programs (from this script)" => types::remove::installed_programs(),
        "Remove Unecessary Programs (or bad ones)" => types::remove::unnecessary_programs(),
        "Skip all tweaks" => types::skip::all_tweaks(),
        _ => errors::idk(),
    }
}
