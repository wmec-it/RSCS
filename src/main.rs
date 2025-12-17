use colored_text::Colorize;
use std::process::Command;
use terminal_menu::*;

use conf::enums::MessageType;
use conf::vars::{INSTALL_PROGRAMS, INSTALL_TYPES, MAIN_THEME, PROGRAM_TITLE, PUNCHDOWN_PAUL};
use utils::{errors::idk, message, user, wait};

mod conf;
mod utils;
mod winget;

fn main() {
    if cfg!(target_os = "windows") {
        open_menu();
    } else {
        message::error(
            MessageType::Print,
            "--! You cannot run this script on Linux!",
        );

        wait::miliseconds(2000);

        Command::new("clear").status().unwrap();
    }
}

fn open_menu() {
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
                message::info(MessageType::Return, "-[i]> Selected Install Type: ").unwrap(),
                mm.selection_value("Install Type")
            );

            handle_install_type(mm.selection_value("Install Type"));
        }
    }
}

fn handle_install_type(_type: &str) {
    INSTALL_TYPES.iter().for_each(|_type| match *_type {
        "Full Install" => handle_run_install_full(),
        "Install Programs" => handle_run_install_programs(),
        "Remove Installed Programs (from this script)" => handle_run_remove_installed_programs(),
        "Remove Unecessary Programs (or bad ones)" => handle_run_remove_unnecessary_programs(),
        _ => idk(),
    });
}

fn install_programs() {
    for program in INSTALL_PROGRAMS {
        winget::winget_install(program);
    }

    println!("");

    message::success(
        MessageType::Print,
        "--> Finished Installing Programs Successfully!\n",
    );
}

fn handle_run_install_full() {
    message::success(MessageType::Print, "+ Starting Full Install...\n");

    install_programs();

    message::success(
        MessageType::Print,
        "=> Full Install Finished Successfully!!!\n",
    );
}

fn handle_run_install_programs() {
    message::success(MessageType::Print, "+ Starting Programs Install...\n");

    install_programs();

    message::success(
        MessageType::Print,
        "--> Programs Install Finished Successfully!!!\n",
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
