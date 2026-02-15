use rscs::core::{command::powershell, helper::winget, process, structs::config::ConfigStructure};

use crate::{
    NAME_PATH_RESOLUTION,
    function::{self, tweaks},
    utils,
};

pub fn find_selected_type(install_type: &str) -> String {
    let npr = NAME_PATH_RESOLUTION.lock().unwrap();
    let names = &npr.names;
    let found_index = names.iter().position(|r| *r == install_type).unwrap();
    let paths = &npr.paths;
    let path = paths[found_index].as_str();
    println!("Path: {}", format!("./menu_options/{}", path));
    return format!("./menu_options/{}", path);
}

pub fn handle_install_type(install_type: &str) -> Result<(), std::io::Error> {
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

pub fn remove_dir_contents<P: AsRef<std::path::Path>>(path: P) -> std::io::Result<()> {
    for entry in std::fs::read_dir(path)? {
        std::fs::remove_file(entry?.path())?;
    }
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
                        "winget" => {
                            println!("Installing {}", program.name.as_str());
                            winget::install::winget(program.id.as_str());
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

    if config.debloat.enabled {
        if config.debloat.bc_uninstaller.enabled {
            for program in config.debloat.bc_uninstaller.store_helper {
                //  TODO: Implement
            }
        }
        if config.debloat.powershell.enabled {
            for program in config.debloat.powershell.appx_package {
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
        }
    }

    if config.tweaks.enabled {
        for tweak in config.tweaks.powershell {
            match tweak.id.as_str() {
                "install-and-configure-powershell7" => {
                    tweaks::powershell::ps7::full();
                }
                "remove-powershell7-telemetry" => {
                    tweaks::powershell::ps7telemetry::disable();
                }
                &_ => (),
            }
        }
        for tweak in config.tweaks.registry {
            match tweak.id.as_str() {
                "disable-bingsearch-in-startmenu" => {
                    tweaks::registry::bingsearch_startmenu::disable();
                }
                "enable-dark-mode" => {
                    tweaks::registry::darkmode::enable();
                }
                "enable-detailed-bsod" => {
                    tweaks::registry::detailedbsod::enable();
                }
                "enable-display-performance-mode" => {
                    tweaks::registry::displayperformance_mode::enable();
                }
                "disable-explorer-home-gallery" => {
                    tweaks::registry::explorer_homegallery::disable();
                }
                "enable-file-extension-visibility" => {
                    tweaks::registry::fileextensionvisibility::enable();
                }
                "enable-hidden-files-visibility" => {
                    tweaks::registry::hiddenfilesvisibility::enable();
                }
                "disable-intel-mm-lms" => {
                    tweaks::registry::intel_mm_lms::disable();
                }
                "disable-microsoft-copilot" => {
                    tweaks::registry::microsoftcopilot::disable();
                }
                "disable-notification-tray" => {
                    tweaks::registry::notificationtray::disable();
                }
                "disable-onedrive" => {
                    tweaks::registry::onedrive::disable();
                }
                "enable-prefer-ipv4" => {
                    tweaks::registry::prefer_ipv4::enable();
                }
                "enable-rclick-end-task" => {
                    tweaks::registry::rclick_end_task::enable();
                }
                "disable-stickykeys-startup" => {
                    tweaks::registry::stickykeys_startup::disable();
                }
                "taskbar-alignment-left" => {
                    tweaks::registry::taskbar_alignment::left();
                }
                "enable-taskbar-search-button" => {
                    tweaks::registry::taskbar_search_button::enable();
                }
                "disable-taskbar-taskview-button" => {
                    tweaks::registry::taskbar_taskview_button::disable();
                }
                "disable-taskbar-widget-button" => {
                    tweaks::registry::taskbar_widgets_button::disable();
                }
                "enable-verbose-logon-messages" => {
                    tweaks::registry::verbose_logon_messages::enable();
                }
                &_ => (),
            }
        }
        for tweak in config.tweaks.power {
            match tweak.id.as_str() {
                "enable-ultimatepowerplan" => {
                    tweaks::power::ultimate_powerplan::enable();
                }
                &_ => (),
            }
        }
    }

    if config.post_configuration.enabled {
        if config.post_configuration.explorer_patcher.enabled {
            if config
                .post_configuration
                .explorer_patcher
                .import_config
                .enabled
            {
                match config
                    .post_configuration
                    .explorer_patcher
                    .import_config
                    .config_type
                    .as_str()
                {
                    "Basic" => {
                        tweaks::registry::explorerpatcher_config::enable();
                    }
                    &_ => (),
                }
            }
        }
        if config.post_configuration.chromium.enabled {
            if config.post_configuration.chromium.save_session_on_close {
                // TODO: Handle save session on close
            }
            for extention in config.post_configuration.chromium.extensions {
                // TODO: Handle extentions
            }
            if !config.post_configuration.chromium.security.level.is_empty() {
                // TODO: Handle security
            }
            if config.post_configuration.chromium.downloads.enabled {
                if config.post_configuration.chromium.downloads.ask_to_download {
                    // TODO: Handle ask to download
                }
                if !config
                    .post_configuration
                    .chromium
                    .downloads
                    .default_download_directory
                    .is_empty()
                {
                    // TODO: Handle default download directory
                }
            }
        }
    }

    if config.branding.enabled {
        if config.branding.dark_mode_enabled {
            tweaks::registry::darkmode::enable();
        }
        if config.branding.custom_taskbar_selection {
            // TODO: Handle custom taskbar selection
        }
        if config.branding.clear_desktop {
            let username = whoami::username().unwrap_or_else(|_| "<unknown>".to_string());

            let path = format!("C:\\Users\\{}\\Desktop", username);
            remove_dir_contents(&path).unwrap();
            match remove_dir_contents(&path) {
                Ok(()) => (),
                Err(error) => println!("Error deleting {} contents: {}", &path, error),
            }
            // TODO: Make this backup shortcuts instead of just deleting them
            match remove_dir_contents("C:\\Users\\Public\\Desktop") {
                Ok(()) => (),
                Err(error) => println!(
                    "Error deleting C:\\Users\\Public\\Desktop contents: {}",
                    error
                ),
            }
        }
        if config.branding.logo.enabled {
            for file in config.branding.logo.get {
                // TODO: Handle logo setting
                match file.replaces.as_str() {
                    "Chromium" => (),
                    &_ => {
                        let username =
                            whoami::username().unwrap_or_else(|_| "<unknown>".to_string());

                        let desktop_shortcut_path =
                            format!("C:\\Users\\{}{}", username, file.desktop_shortcut_path);

                        if std::path::Path::new(desktop_shortcut_path.as_str()).exists() {
                            std::fs::remove_file(&desktop_shortcut_path)?;
                        }

                        // TODO: Implement taskbar path too

                        let ps_script = format!(
                            r#"
                            $target = "{}"
                            $shortcutPath = "{}"
                            $icon = "{}"
                            $icon_filename = Split-Path $icon -leaf
                            $iconsDir = "C:\\Users\\{username}\\Branding\\"

                            if (-not (Test-Path $iconsDir)) {{
                                New-Item -Path $iconsDir -ItemType Directory
                            }}

                            cp -r "$icon" "$iconsDir$icon_filename"

                            $shell = New-Object -ComObject WScript.Shell
                            $lnk = $shell.CreateShortcut($shortcutPath)

                            $lnk.TargetPath = $target
                            $lnk.IconLocation = "$iconsDir$icon_filename"
                            $lnk.Arguments = "{}"
                            $lnk.Save()
                            "#,
                            format!("C:\\Users\\{}{}", username, file.executable_path),
                            desktop_shortcut_path,
                            file.icon_path,
                            if !file.args.is_empty() {
                                file.args
                            } else {
                                "".to_string()
                            },
                            username = username
                        );

                        std::process::Command::new("powershell")
                            .arg("-NoProfile")
                            .arg("-Command")
                            .arg(ps_script)
                            .output()
                            .expect("Failed to run PowerShell");
                    }
                }
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

    if config.extra.enabled {
        for script in config.extra.scripts {
            println!("{}", script.run_message);
            // TODO: Handle method, locator, execMethod, and privileges
            println!("{}", script.completion_message);
        }
        for command in config.extra.commands {
            println!("{}", command.run_message);
            // TODO: Handle privileges and command
            println!("{}", command.completion_message);
        }
    }

    process::explorer::restart();

    Ok(())
}
