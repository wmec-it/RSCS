use std::process::Command;
use terminal_menu::*;
use crossterm::style::Color;

const PROGRAM_TITLE: &str = "West-Mec Repair Shop Configuration Tool";
const INSTALL_TYPES: &[&str] = &[
    "Full Install",
    "Install Programs",
    "Remove Installed Programs (from this script)",
    "Remove Unecessary Programs (or bad ones)",
];

fn main() {
    if cfg!(target_os = "windows") {

        let menu = menu(vec![
            label(PROGRAM_TITLE).colorize(Color::Yellow),
            label(""),
            scroll("Install Type", INSTALL_TYPES.iter().copied()),
            label(""),
            button("Start Install"),
        ]);
        run(&menu);
        {
            let mm = mut_menu(&menu);
            println!("{}", mm.selection_value("Install Type"));
            println!("{}", mm.selected_item_name());

            handle_install_type(mm.selection_value("Install Type"));
        }
    } else {
        Command::new("clear")
            .status()
            .unwrap();
    }
}

fn handle_install_type(_type: &str) {
    if (_type == INSTALL_TYPES[0]) {
        println!("Hello!!!!");
    } else {
        println!("Goodbye!!!!");
    }
}

fn winget_install(program_name: &str) {
    let installing_message: String = format!("Installing {}...", program_name);
    let install_message_command: String = format!("Write-Host {}", installing_message);

    let install_message_command_output = Command::new("powershell")
        .arg("-Command")
        .arg(install_message_command)
        .output()
        .expect("Failed to send \"Installing\" message...");
    
    String::from_utf8_lossy(&install_message_command_output.stdout);

    if !install_message_command_output.status.success() {
        eprintln!("PowerShell returned an error:\n{}", String::from_utf8_lossy(&install_message_command_output.stderr));
    }
}

// fn install_pograms() {
    
// }