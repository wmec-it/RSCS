use colored_text::Colorize;
use std::process::Command;
use terminal_menu::*;

use conf::enums::{DelimiterType, MessageType};
use conf::vars::{INSTALL_TYPES, MAIN_THEME, PROGRAM_TITLE, PUNCHDOWN_PAUL};
use testing::testing;
use utils::{message, user, wait};

mod conf;
mod handles;
mod system;
mod testing;
mod utils;
mod external;

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

                    prerequisites();

                    handles::install_type(mm.selection_value("Install Type"));

                    wait::seconds(720);
                }
            }
        }
    }
}

fn prerequisites() {
    system::manage::backups::create_restore_point();
}
