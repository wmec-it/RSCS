use crate::utils::errors;
use crate::{AppContext, types};

pub fn install_type(ctx: &mut AppContext, install_type: &str) {
    match install_type {
        "Full Install" => types::install::full(ctx),
        "Install Programs" => types::install::programs(ctx),
        "Remove Installed Programs (from this script)" => types::remove::installed_programs(ctx),
        "Remove Unecessary Programs (or bad ones)" => types::remove::unnecessary_programs(ctx),
        "Skip all tweaks" => types::skip::all_tweaks(ctx),
        _ => errors::idk(ctx),
    }
}
