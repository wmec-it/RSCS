use std::process::Command;
use terminal_menu::*;
use colored_text::Colorize;

use utils::wait;
use conf::vars::{PROGRAM_TITLE, MAIN_THEME, INSTALL_TYPES, PUNCHDOWN_PAUL, INSTALL_PROGRAMS};

mod utils;
mod conf;
mod winget;

fn main() {
    if cfg!(target_os = "windows") {
        let menu = menu(
            vec![
                label(format!("{}", PROGRAM_TITLE).hex(MAIN_THEME.primary)),
                label(""),
                scroll("Install Type", INSTALL_TYPES.iter().copied()),
                label(""),
                button("Start Install")
            ]
        );
        run(&menu);
        {
            if !mut_menu(&menu).canceled() == true {
                println!("{}", format!("{}", PUNCHDOWN_PAUL).hex(MAIN_THEME.primary));

                let mm = mut_menu(&menu);
                println!(
                    "{} {}",
                    format!("{}", "Selected Install Type: ".hex(MAIN_THEME.info)),
                    mm.selection_value("Install Type")
                );

                handle_install_type(mm.selection_value("Install Type"));
            }
        }
    } else {
        println!("{}", "You cannot run this script on Linux!".hex(MAIN_THEME.error));
        
        wait::miliseconds(2000);

        Command::new("clear").status().unwrap();
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
        println!("{}", "Invalid Entry (idk what is wrong)".hex(MAIN_THEME.error));
    }
}

fn install_programs() {
    for program in INSTALL_PROGRAMS {
        winget::winget_install(program);
    }

    println!("");

    println!("{}", "+ Finished Installing Programs Successfully!\n".hex(MAIN_THEME.success));
}

fn handle_run_install_full() {
    println!("{}", "+ Starting Full Install...\n".hex(MAIN_THEME.success));

    install_programs();

    println!("{}", "-- Full Install Finished Successfully!!! --\n".hex(MAIN_THEME.success));
}

fn handle_run_install_programs() {
    println!("{}", "Starting Programs Install...\n".hex(MAIN_THEME.success));

    install_programs();

    println!("{}", "Programs Install Finished Successfully!!!\n".hex(MAIN_THEME.success));
}

fn handle_run_remove_installed_programs() {
    for program in INSTALL_PROGRAMS {
        winget::winget_remove(program);
    }

    println!("{}", "Finished Removing Installed Programs Successfully!\n".hex(MAIN_THEME.success));
}
