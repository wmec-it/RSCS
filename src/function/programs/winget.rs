use rscs::core::helper::winget;

#[allow(dead_code)]
pub fn winget_install(program_id: &str) {
    // println!(
    //     "{} {}...",
    //     message::add_delimiter(
    //         DelimiterType::Layer1,
    //         "Installing".to_string(),
    //         Some(true),
    //         None,
    //         None
    //     )
    //     .unwrap()
    //     .as_str(),
    //     &program_name.hex(MAIN_THEME.primary)
    // );
    winget::install::official_repo(program_id); // if !install_command_output.status.success() {
    //     if String::from_utf8_lossy(&install_command_output.stdout)
    //         .contains("Found an existing package")
    //     {
    //         message::warning(
    //             MessageType::Print,
    //             message::add_delimiter(
    //                 DelimiterType::Layer2,
    //                 "This package is already installed!".to_string(),
    //                 Some(true),
    //                 None,
    //                 None,
    //             )
    //             .unwrap()
    //             .as_str(),
    //         );
    //     } else {
    //         message::error(
    //             MessageType::Print,
    //             message::add_delimiter(
    //                 DelimiterType::Layer2Error,
    //                 String::from_utf8_lossy(&install_command_output.stdout).to_string(),
    //                 Some(true),
    //                 None,
    //                 None,
    //             )
    //             .unwrap()
    //             .as_str(),
    //         );
    //     }
    // } else {
    //     message::success(
    //         MessageType::Print,
    //         message::add_delimiter(
    //             DelimiterType::Layer2Success,
    //             format!("Successfully Installed {}!", &program_name),
    //             Some(true),
    //             None,
    //             None,
    //         )
    //         .unwrap()
    //         .as_str(),
    //     );
    // }
}

#[allow(dead_code)]
pub fn winget_remove(program_id: &str) {
    // println!(
    //     "{} {}...",
    //     message::add_delimiter(
    //         DelimiterType::Layer1,
    //         "Removing".to_string(),
    //         Some(true),
    //         None,
    //         None
    //     )
    //     .unwrap()
    //     .as_str(),
    //     &program_name.hex(MAIN_THEME.error)
    // );

    winget::remove::basic(program_id);

    // if !remove_command_output.status.success() {
    //     message::error(
    //         MessageType::Print,
    //         message::add_delimiter(
    //             DelimiterType::Layer2Error,
    //             format!(
    //                 "PowerShell returned an error:\n{}",
    //                 String::from_utf8_lossy(&remove_command_output.stdout)
    //             )
    //             .to_string(),
    //             Some(true),
    //             None,
    //             None,
    //         )
    //         .unwrap()
    //         .as_str(),
    //     );
    // } else {
    //     println!(
    //         "{}",
    //         String::from_utf8_lossy(&remove_command_output.stdout).hex(MAIN_THEME.info)
    //     );
    //     message::info(
    //         MessageType::Print,
    //         message::add_delimiter(
    //             DelimiterType::Layer2Info,
    //             String::from_utf8_lossy(&remove_command_output.stdout).to_string(),
    //             Some(true),
    //             None,
    //             None,
    //         )
    //         .unwrap()
    //         .as_str(),
    //     );
    // }
}
