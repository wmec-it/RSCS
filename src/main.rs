use colored_text::Colorize;
use std::process::Command;
use terminal_menu::*;

use conf::enums::{DelimiterType, MessageType};
use conf::vars::{INSTALL_PROGRAMS, INSTALL_TYPES, MAIN_THEME, PROGRAM_TITLE, PUNCHDOWN_PAUL};
use testing::testing;
use tweaks::powershell::ps7;
use utils::{errors::idk, message, user, wait};

mod conf;
mod testing;
mod tweaks;
mod utils;
mod winget;

fn main() {
    if cfg!(target_os = "windows") {
        open_menu(false);
    } else {
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

        wait::miliseconds(2000);

        Command::new("clear").status().unwrap();
    }
}

fn open_menu(is_testing: bool) {
    match is_testing {
        true => {
            testing();
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

                    handle_install_type(mm.selection_value("Install Type"));

                    wait::miliseconds(7200000);
                }
            }
        }
    }
}

fn handle_install_type(install_type: &str) {
    match install_type {
        "Full Install" => handle_run_install_full(),
        "Install Programs" => handle_run_install_programs(),
        "Remove Installed Programs (from this script)" => handle_run_remove_installed_programs(),
        "Remove Unecessary Programs (or bad ones)" => handle_run_remove_unnecessary_programs(),
        _ => idk(),
    }
}

fn install_programs() {
    for program in INSTALL_PROGRAMS {
        winget::winget_install(program);
    }

    println!("{}", "|".hex(MAIN_THEME.success));

    message::success(
        MessageType::Print,
        message::add_delimiter(
            DelimiterType::Layer1Success,
            "Finished Installing Programs Successfully!".to_string(),
            Some(true),
            None,
            Some(true),
        )
        .unwrap()
        .as_str(),
    );
}

fn run_tweaks() {
    message::success(
        MessageType::Print,
        message::add_delimiter(
            DelimiterType::Layer1Add,
            "Running Tweaks...\n|".to_string(),
            Some(true),
            Some(true),
            None,
        )
        .unwrap()
        .as_str(),
    );

    ps7();
}

fn handle_run_install_full() {
    message::success(
        MessageType::Print,
        message::add_delimiter(
            DelimiterType::Layer1Add,
            "Starting Full Install...\n|".to_string(),
            Some(true),
            None,
            None,
        )
        .unwrap()
        .as_str(),
    );

    install_programs();
    message::seperator();
    run_tweaks();

    message::success(
        MessageType::Print,
        message::add_delimiter(
            DelimiterType::Layer1Success,
            "Full Install Finished Successfully!!!".to_string(),
            Some(true),
            None,
            Some(true),
        )
        .unwrap()
        .as_str(),
    );
}

fn handle_run_install_programs() {
    message::success(
        MessageType::Print,
        message::add_delimiter(
            DelimiterType::Layer1Add,
            "Starting Programs Install...\n|".to_string(),
            Some(true),
            None,
            None,
        )
        .unwrap()
        .as_str(),
    );

    install_programs();

    message::success(
        MessageType::Print,
        message::add_delimiter(
            DelimiterType::Layer2Success,
            "Programs Install Finished Successfully!!!".to_string(),
            Some(true),
            None,
            Some(true),
        )
        .unwrap()
        .as_str(),
    );
}

fn handle_run_remove_installed_programs() {
    for program in INSTALL_PROGRAMS {
        winget::winget_remove(program);
    }

    message::success(
        MessageType::Print,
        "--> Finished Removing Installed Programs Successfully!\n",
    );
}

fn handle_run_remove_unnecessary_programs() {
    // Nothing for now...
}
