use crate::conf::structs::MenuOption;
use crate::{system::utils::commands::exec::ps1, utils::files::create_temp_file};
use serde::{Deserialize, Serialize};

use std::error::Error;
use std::fs::{self, File};
use std::io::{BufReader, Read};
use std::path::Path;

pub fn testing() {
    let p = Path::new("./menu_options/template-menu-option.json");
    let text = fs::read_to_string(p).unwrap();
    let cfg: MenuOption = serde_json::from_str(&text).unwrap();

    println!("DisplayName = {}", cfg.display_name);
}