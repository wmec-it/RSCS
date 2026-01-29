use rscs::core::{command::powershell, helper::winget, structs::config::ConfigStructure};

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
    if config.does_back_up {
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

    if config.branding.enabled {
        if config.branding.dark_mode_enabled {
            // TODO: Handle Branding dark mode
        }
        if config.branding.logo.enabled {
            for file in config.branding.logo.get {
                // TODO: Handle logo setting
            }
        }
        if config.branding.windows.enabled {
            if config.branding.windows.wallpapers.enabled {
                for file in config.branding.windows.wallpapers.get {
                    // TODO: Handle windows wallpaper setting
                }
            }
            if config.branding.windows.accent_color.enabled {
                // TODO: Handle color
            }
        }
        if config.branding.chromium.enabled {
            for file in config.branding.chromium.icons {
                // TODO: Handle icons
            }
        }
        if config.branding.signal_rgb.enabled {
            if config.branding.signal_rgb.configuration.enabled {
                if config.branding.signal_rgb.configuration.full.enabled {
                    // TODO: Handle SignalRGB colors
                }
            }
        }
    }

    if config.programs.enabled {
        //:& Install
        if config.programs.install.enabled {
            for program in config.programs.install.winget {
                if !program.id.as_str().is_empty() {
                    match program.source.as_str() {
                        "msstore" => {
                            println!("Installing {}", program.name.as_str());
                            winget::install::msstore(program.id.as_str());
                        }
                        &_ => {
                            println!("Installing {}", program.name.as_str());
                            winget::install::official_repo(program.id.as_str());
                        }
                    }
                }
            }
            for program in config.programs.install.powershell {
                // TODO: Handle powershell installs
            }
            for program in config.programs.install.appx_package {
                // TODO: Handle AppxPackage installs
            }
        }
        if config.programs.remove.enabled {
            for program in config.programs.remove.winget {
                // TODO: Implement winget uninstalls
            }
            for program in config.programs.remove.powershell {
                // TODO: Implement powershell uninstalls
            }
            for program in config.programs.remove.appx_package {
                let run_message = format!("Deleting {} APPX Package...", program.name);
                let success_message =
                    format!("Successfully deleted {} APPX Package!", program.name);
                let error_message = format!("Error deleting {} APPX Package...", program.name);
                powershell::admin(
                    if !program.name.is_empty() {
                        run_message.as_str()
                    } else {
                        "Deleting Unknown APPX Package..."
                    },
                    if !program.name.is_empty() {
                        success_message.as_str()
                    } else {
                        "Successfully deleted Unknown APPX Package!"
                    },
                    if !program.name.is_empty() {
                        error_message.as_str()
                    } else {
                        "Error deleting Unknown APPX Package..."
                    },
                    format!("Remove-AppxPackage -package {} -confirm:$false", program.id).as_str(),
                );
            }
            if config.programs.remove.bc_uninstaller.enabled {
                for program in config.programs.remove.bc_uninstaller.store_helper {
                    // TODO: Implement store helper function
                }
                for program in config.programs.remove.bc_uninstaller.powershell {
                    // TODO: Implement powershell function
                }
            }
            for program in config.programs.remove.manual {
                println!("Uninstalling {}...", program.name);
                if program.official_uninstaller {
                    std::process::Command::new(program.command.program)
                        .args(program.command.args.iter().map(|a| a.arg.as_str()));
                }
            }
        }
    }

    Ok(())
}
