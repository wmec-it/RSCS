use colored_text::Colorize;
use serde::{Deserialize, Serialize};
use std::fs::ReadDir;
use std::{io, process::Command};
use terminal_menu::*;

use conf::enums::{DelimiterType, MessageType};
use conf::vars::{MAIN_THEME, PROGRAM_TITLE, PUNCHDOWN_PAUL};
use std::sync::Mutex;
use testing::testing;
use utils::{message, user};

mod conf;
mod external;
mod handles;
mod system;
mod testing;
mod types;
mod utils;

#[derive(Debug)]
pub struct NAMEPATH {
    paths: Vec<String>,
    names: Vec<String>,
}
static NAME_PATH_RESOLUTION: std::sync::LazyLock<Mutex<NAMEPATH>> =
    std::sync::LazyLock::new(|| {
        Mutex::new(NAMEPATH {
            paths: Vec::new(),
            names: Vec::new(),
        })
    });

fn get_install_types() -> Vec<std::string::String> {
    let dir: &str = "./menu_options"; // TODO: Make this a command line argument, global config option, or calculated

    //:& Gets paths
    let paths: ReadDir = std::fs::read_dir(&dir).unwrap();
    //:& Created new Vec to store found file paths
    let mut file_paths: Vec<String> = Vec::new();
    for path in paths {
        //:& Iterate through paths
        //:& Push file paths to previously created Vec
        NAME_PATH_RESOLUTION
            .lock()
            .unwrap()
            .paths
            .push(path.unwrap().file_name().into_string().unwrap());
    }
    let paths: ReadDir = std::fs::read_dir(&dir).unwrap();
    for path in paths {
        //:& Iterate through paths
        //:& Push file paths to previously created Vec
        file_paths.push(path.unwrap().file_name().into_string().unwrap());
    }
    //:& Create new Vec to store found entry names with path previously found
    let mut entry_names: Vec<String> = Vec::new();
    for file in &file_paths {
        //:& Iterate through files
        //:& Write file contents to String
        let contents =
            std::fs::read_to_string(format!("{}/{}", dir, file)).expect("Couldn't read file...");
        //:& Parse file contents
        let config: Config =
            serde_json::from_str(contents.as_str()).expect("JSON was not well-formatted");
        //:& Push DisplayNames from each config to previouslt created Vec
        entry_names.push(config.display_name);
    }
    for file in &file_paths {
        //:& Iterate through files
        //:& Write file contents to String
        let contents =
            std::fs::read_to_string(format!("{}/{}", dir, file)).expect("Couldn't read file...");
        //:& Parse file contents
        let config: Config =
            serde_json::from_str(contents.as_str()).expect("JSON was not well-formatted");
        //:& Push DisplayNames from each config to previouslt created Vec
        NAME_PATH_RESOLUTION
            .lock()
            .unwrap()
            .names
            .push(config.display_name);
    }
    //:& Return all DisplayNames inside of a Vec
    return entry_names;
}

fn main() -> Result<(), io::Error> {
    //:& Check if OS is Windows
    if cfg!(target_os = "windows") {
        //:& Open menu and catch any errors
        match open_menu("") {
            Ok(()) => (),
            Err(error) => println!("{}", error),
        }

        //:& Prompt user to exit
        let mut buffer = String::new();
        let stdin = io::stdin();
        println!("Press enter to exit...");
        stdin.read_line(&mut buffer)?;

        //:& Exit
        Ok(())
    } else {
        //:& Show error for nerds on Linux
        message::error(
            MessageType::Print,
            message::add_delimiter(
                DelimiterType::Layer1Error,
                "You cannot run this script on Linux!".to_string(),
                Some(true),
                None,
                Some(true),
            )
            .unwrap()
            .as_str(),
        );

        //:& Clear terminal
        Command::new("clear").status()?;

        //:& Return error
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "You cannot run this script on Linux!",
        ))
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Config {
    // TODO: Add full json structure
    display_name: String,
}

//:& Opens menu
fn open_menu(operation_type: &str) -> Result<(), io::Error> {
    // TODO: Find a better way to run tests and stuff
    match operation_type {
        "testing" => {
            //:& Lets you use the PROJECT_ROOT/src/testing/testing.rs file for testing stuff
            testing();
            Ok(())
        }
        "skip" => {
            //:& Enables sudo
            user::enable_sudo();
            //:& Run prerequisites and catch errors
            match prerequisites() {
                Ok(()) => (),
                Err(error) => {
                    message::error(
                        MessageType::Print,
                        message::add_delimiter(
                            DelimiterType::Layer1Error,
                            "You cannot run this script on Linux!".to_string(),
                            Some(true),
                            None,
                            Some(true),
                        )
                        .unwrap()
                        .as_str(),
                    );
                    return Err(error);
                }
            }
            //:& Handle install type selection
            handles::handle_install_type("Skip all tweaks");
            Ok(())
        }
        _ => {
            //:& Menu layout definition
            let menu = menu(vec![
                label(format!("{}", PROGRAM_TITLE).hex(MAIN_THEME.primary)),
                label(""),
                scroll("Install Type", get_install_types().iter()),
                label(""),
                button("Start Install"),
            ]);
            // I don't need to explain this...
            run(&menu);
            {
                //:& Catch if there is an oopsie in the menu logic
                match menu_logic(menu) {
                    Ok(()) => (),
                    Err(error) => println!("{}", error),
                };
            }
            Ok(())
        }
    }
}

fn menu_logic(
    menu: std::sync::Arc<std::sync::RwLock<TerminalMenuStruct>>,
) -> Result<(), io::Error> {
    if !mut_menu(&menu).canceled() == true {
        //:& Prints the orange god
        println!("{}", format!("{}", PUNCHDOWN_PAUL).hex(MAIN_THEME.primary));

        //:& Warning banner to make sure Mr. Getz doesn't smite you
        message::error_banner(
            MessageType::Print,
            "   Make sure you have permission from Mr. Getz if you are using this program...    ",
        );

        //:& Obviously enables sudo
        user::enable_sudo();

        let mm = mut_menu(&menu);

        // TODO: Match selected Install Type with the 2 vecs inside of NAMEPATH to find current path of selected menu option
        // let mut global_config_storage = HANDLE_CONFIG.lock().unwrap();
        // *global_config_storage = CONFIG {
        //     path:
        // }
        // println!("{:#?}", NAME_PATH_RESOLUTION);
        // use crate::NAME_PATH_RESOLUTION;
        //:& Displays install type
        //:& Using println!("{} {}") So that the selection value is just normal text color but formatting is the same
        println!(
            "{} {}",
            message::info(
                MessageType::Return,
                message::add_delimiter(
                    DelimiterType::Layer1Info,
                    "Selected Install Type: ".to_string(),
                    Some(true),
                    None,
                    None
                )
                .unwrap()
                .as_str()
            )
            .unwrap(),
            mm.selection_value("Install Type")
        );

        //:& Run prerequisites and catch errors
        match prerequisites() {
            Ok(()) => (),
            Err(error) => {
                //:& Display error message
                utils::errors::function("Prerequisites failed to run...");
                return Err(error);
            }
        }

        //:& Handle install type selection
        handles::handle_install_type(mm.selection_value("Install Type"));

        // Everything is fine :3
        Ok(())
    } else {
        // Oopsy woopsie, there is an error :(
        Err(io::Error::new(
            io::ErrorKind::Other,
            "Failed to handle menu logic...",
        ))
    }
}

fn prerequisites() -> Result<(), io::Error> {
    //:& Handle errors from restore point
    match system::manage::backups::create_restore_point() {
        // Everything is ok
        Ok(()) => Ok(()),
        Err(error) => {
            //:& Display error message
            utils::errors::function("Failed to handle menu logic...");
            return Err(error);
        }
    }
}
