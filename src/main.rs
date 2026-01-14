use colored_text::Colorize;
use indicatif::ProgressBar;
use std::{io, process::Command};
use terminal_menu::*;

use conf::enums::{DelimiterType, MessageType};
use conf::vars::{INSTALL_TYPES, MAIN_THEME, PROGRAM_TITLE, PUNCHDOWN_PAUL};
use testing::testing;
use utils::{message, user};

use crate::conf::vars::INSTALL_PROGRAMS;

mod conf;
mod external;
mod handles;
mod system;
mod testing;
mod types;
mod utils;

struct AppContext {
    pbl: u8,
    pb: ProgressBar,
}

impl AppContext {
    pub fn new(initial_value: u8) -> Self {
        let pb = Self::create_progressbar(initial_value);
        Self {
            pbl: initial_value,
            pb,
        }
    }

    fn create_progressbar(value: u8) -> ProgressBar {
        ProgressBar::new(value as u64)
    }

    // Mutator method with custom logic
    pub fn set_pbl_value(&mut self, new_value: u8) {
        self.pbl = new_value;
        // You can run any other function or logic here
        self.run_extra_pbl_logic();
    }

    // Accessor method
    pub fn get_pbl_value(&self) -> u8 {
        self.pbl
    }

    // Additional function to run on update
    fn run_extra_pbl_logic(&self) {
        self.pb.set_length(self.pbl as u64);
    }
}

fn main() -> Result<(), io::Error> {
    let mut ctx = AppContext::new(0);
    ctx.set_pbl_value(INSTALL_PROGRAMS.len() as u8);
    // Check if OS is Windows
    if cfg!(target_os = "windows") {
        // Open menu and catch any errors
        match open_menu(&mut ctx, "") {
            Ok(()) => (),
            Err(error) => ctx.pb.println(error.to_string()),
        }

        ctx.pb.finish();

        // Prompt user to exit
        let mut buffer = String::new();
        let stdin = io::stdin();
        ctx.pb.println("Press enter to exit...");
        stdin.read_line(&mut buffer)?;

        // Exit
        Ok(())
    } else {
        // Show error for nerds on Linux
        message::error(
            &mut ctx,
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

// Opens menu
fn open_menu(ctx: &mut AppContext, operation_type: &str) -> Result<(), io::Error> {
    match operation_type {
        "testing" => {
            // Lets you use the PROJECT_ROOT/src/testing/testing.rs file for testing stuff
            testing(ctx);
            Ok(())
        }
        "skip" => {
            // Enables sudo
            user::enable_sudo(ctx);
            // Run prerequisites and catch errors
            match prerequisites(ctx) {
                Ok(()) => (),
                Err(error) => {
                    message::error(
                        ctx,
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
            // Handle install type selection
            handles::install_type(ctx, "Skip all tweaks");
            Ok(())
        }
        _ => {
            // Menu layout definition
            let menu = menu(vec![
                label(format!("{}", PROGRAM_TITLE).hex(MAIN_THEME.primary)),
                label(""),
                scroll("Install Type", INSTALL_TYPES.iter().copied()),
                label(""),
                button("Start Install"),
            ]);
            // I don't need to explain this...
            run(&menu);
            {
                // Catch if there is an oopsie in the menu logic
                match menu_logic(ctx, menu) {
                    Ok(()) => (),
                    Err(error) => ctx.pb.println(error.to_string()),
                };
            }
            Ok(())
        }
    }
}

fn menu_logic(
    ctx: &mut AppContext,
    menu: std::sync::Arc<std::sync::RwLock<TerminalMenuStruct>>,
) -> Result<(), io::Error> {
    if !mut_menu(&menu).canceled() == true {
        // Prints the orange god
        ctx.pb
            .println(format!("{}", PUNCHDOWN_PAUL).hex(MAIN_THEME.primary));

        // Warning banner to make sure Mr. Getz doesn't smite you
        message::error_banner(
            ctx,
            MessageType::Print,
            "   Make sure you have permission from Mr. Getz if you are using this program...    ",
        );

        // Obviously enables sudo
        user::enable_sudo(ctx);

        let mm = mut_menu(&menu);
        // Displays install type
        // Using ctx.pb.println("{} {}") So that the selection value is just normal text color but formatting is the same
        let formatted_install_type_message = format!(
            "{} {}",
            message::info(
                ctx,
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
        ctx.pb.println(formatted_install_type_message);

        // Run prerequisites and catch errors
        match prerequisites(ctx) {
            Ok(()) => (),
            Err(error) => {
                // Display error message
                utils::errors::function(ctx, "Prerequisites failed to run...");
                return Err(error);
            }
        }

        // Handle install type selection
        handles::install_type(ctx, mm.selection_value("Install Type"));

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

fn prerequisites(ctx: &mut AppContext) -> Result<(), io::Error> {
    // Handle errors from restore point
    match system::manage::backups::create_restore_point(ctx) {
        // Everything is ok
        Ok(()) => Ok(()),
        Err(error) => {
            // Display error message
            utils::errors::function(ctx, "Failed to handle menu logic...");
            return Err(error);
        }
    }
}
