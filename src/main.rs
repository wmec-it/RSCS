use colored_text::Colorize;
use std::process::Command;
use terminal_menu::*;

use conf::vars::{INSTALL_PROGRAMS, INSTALL_TYPES, MAIN_THEME, PROGRAM_TITLE, PUNCHDOWN_PAUL};
use utils::{message, message::MessageType, wait};

mod conf;
mod utils;
mod winget;

fn main() {
    if cfg!(target_os = "windows") {
        open_menu();
    } else {
        message::error(MessageType::Print, "You cannot run this script on Linux!");

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

            message::warning(
                MessageType::Print,
                "Make sure you have permission from Mr. Getz if you are using this program...\n",
            );

            let mm = mut_menu(&menu);
            println!(
                "{} {}",
                message::info(MessageType::Return, "Selected Install Type: ").unwrap(),
                mm.selection_value("Install Type")
            );

            handle_install_type(mm.selection_value("Install Type"));
        }
    }
}

fn handle_install_type(_type: &str) {
    if _type == INSTALL_TYPES[0] {
        handle_run_install_full();
    } else if _type == INSTALL_TYPES[1] {
        handle_run_install_programs();
    } else if _type == INSTALL_TYPES[2] {
        handle_run_remove_installed_programs();
    } else {
        message::error(MessageType::Print, "Invalid Entry (idk what is wrong)");
    }
}

fn install_programs() {
    for program in INSTALL_PROGRAMS {
        winget::winget_install(program);
    }

    println!("");

    message::success(
        MessageType::Print,
        "+ Finished Installing Programs Successfully!\n",
    );
}

fn handle_run_install_full() {
    message::success(MessageType::Print, "+ Starting Full Install...\n");

    install_programs();

    message::success(
        MessageType::Print,
        "-- Full Install Finished Successfully!!! --\n",
    );
}

fn handle_run_install_programs() {
    message::success(MessageType::Print, "Starting Programs Install...\n");

    install_programs();

    message::success(
        MessageType::Print,
        "Programs Install Finished Successfully!!!\n",
    );
}

fn handle_run_remove_installed_programs() {
    for program in INSTALL_PROGRAMS {
        winget::winget_remove(program);
    }

    message::success(
        MessageType::Print,
        "Finished Removing Installed Programs Successfully!\n",
    );
}
