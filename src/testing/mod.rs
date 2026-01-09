use crate::{system::utils::commands::exec::ps1, utils::files::create_temp_file};

pub fn testing() {
    let script_path =
        create_temp_file(include_bytes!("../external/scripts/popup.ps1"), "popup.ps1")
            .expect("Failed to create temp file");

    match ps1(&script_path) {
        Ok(()) => println!("\x1b[92mSuccessfully ran script!\x1b[0m"),
        Err(error) => println!("Error: {}", error),
    };
}
