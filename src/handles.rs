use rscs::core::{helper::winget, structs::config::ConfigStructure};

use crate::{NAME_PATH_RESOLUTION, function, utils};

pub fn find_selected_type(install_type: &str) -> String {
    let npr = NAME_PATH_RESOLUTION.lock().unwrap();
    let names = &npr.names;
    println!("{:#?}", names);
    let found_index = names.iter().position(|r| *r == install_type).unwrap();
    println!("Found Index: {}", found_index);
    let paths = &npr.paths;
    println!("{:#?}", paths);
    let path = paths[found_index].as_str();
    println!("Path: {}", format!("./menu_options/{}", path));
    return format!("./menu_options/{}", path);
}

pub fn handle_install_type(install_type: &str) -> Result<(), std::io::Error> {
    println!("{}", install_type);
    let config_path = find_selected_type(install_type);
    println!("Finding selected type... {}", config_path);
    let json_file = std::fs::read_to_string(&config_path)
        .unwrap_or_else(|e| panic!("Failed to read {}: {}", config_path, e));
    let json: ConfigStructure = serde_json::from_str(json_file.as_str())
        .unwrap_or_else(|e| panic!("Invalid JSON in {}: {}", config_path, e));
    println!("Converting json to Point...");
    match start(json) {
        Ok(()) => (),
        Err(error) => return Err(error),
    };
    Ok(())
}

pub fn start(config: ConfigStructure) -> Result<(), std::io::Error> {
    match config.does_back_up {
        true => {
            match function::backups::create_restore_point() {
                // Everything is ok
                Ok(()) => (),
                Err(error) => {
                    //:& Display error message
                    utils::errors::function(
                        format!("Failed to handle menu logic...\n   {}", error).as_str(),
                    );
                }
            }
        }
        false => (),
    }

    if config.programs.enabled {
        //:& Install
        if config.programs.install.enabled {
            for program in config.programs.install.winget {
                if !program.id.as_str().is_empty() {
                    println!("Installing {}", program.name.as_str());
                    winget::install::official_repo(program.id.as_str());
                }
            }
        }
    }

    Ok(())
}
