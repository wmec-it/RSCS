use colored_text::Colorize;
use std::{io, process::Command};
use terminal_menu::*;

use conf::enums::{DelimiterType, MessageType};
use conf::vars::{INSTALL_TYPES, MAIN_THEME, PROGRAM_TITLE, PUNCHDOWN_PAUL};
use testing::testing;
use utils::{message, user};

mod conf;
mod external;
mod handles;
mod system;
mod testing;
mod types;
mod utils;

fn main() -> Result<(), io::Error> {
    // Check if OS is Windows
    if cfg!(target_os = "windows") {
        // Open menu and catch any errors
        match open_menu("") {
            Ok(()) => (),
            Err(error) => println!("{}", error),
        };

        // Prompt user to exit
        let mut buffer = String::new();
        let stdin = io::stdin();
        println!("Press enter to exit...");
        stdin.read_line(&mut buffer)?;

        // Exit
        Ok(())
    } else {
        // Show error
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

        // Clear terminal
        Command::new("clear").status()?;

        // Return error
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "You cannot run this script on Linux!",
        ))
    }
}

fn open_menu(operation_type: &str) -> Result<(), io::Error> {
    match operation_type {
        "testing" => {
            testing();
            Ok(())
        }
        "skip" => {
            user::enable_sudo();
            prerequisites();
            handles::install_type("Skip all tweaks");
            Ok(())
        }
        _ => {
            let menu = menu(vec![
                label(format!("{}", PROGRAM_TITLE).hex(MAIN_THEME.primary)),
                label(""),
                scroll("Install Type", INSTALL_TYPES.iter().copied()),
                label(""),
                button("Start Install"),
            ]);
            run(&menu);
            {
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
        println!("{}", format!("{}", PUNCHDOWN_PAUL).hex(MAIN_THEME.primary));

        println!(
                        "{}{}",
                        message
                            ::error_banner(
                                MessageType::Return,
                                "   Make sure you have permission from Mr. Getz if you are using this program...    "
                            )
                            .unwrap(),
                        "\n"
                    );

        user::enable_sudo();

        let mm = mut_menu(&menu);
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

        prerequisites();

        handles::install_type(mm.selection_value("Install Type"));
    }
    Ok(())
}

fn prerequisites() -> Result<(), io::Error> {
    system::manage::backups::create_restore_point();
    Ok(())
}
