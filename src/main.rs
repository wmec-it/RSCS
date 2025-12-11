use std::process::Command;
use terminal_menu::*;
use colored_text::Colorize;

const PUNCHDOWN_PAUL: &str = "
                                                                                            
                                                                                            
                                                                                            
                            5µµOµµµµOµµµOxyTÎO                                                
                    yµµµµµµµµµµµµµµµµµµµµµµµµµµO6OOOyxxU                                   
                OOµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµOOy¾f                       
            µµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµOµOf              
            µµµµµµµµµµµµµµµµµµµµµµµUkàZähS6µµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµ              
            µµµµµµµµµµµµµµµûpÆÆÆÆÆÆÆÆÆÆÆÆÆÆÆÆÆÆÆBZµµµµµµµµµµµµµµµµµµµµµµµµµµµ              
            µµµµµµµµµµUêÆÆÆÆÆÆÆÆÆÆÆÆŒØÈÈØŒÆÆÆÆÆÆÆÆÆÆëµµµµµµµ§ÉÆÆÆÆÆÆÆÆÆÆÆØÖ6µ              
            Oµµµµµµ6ÆÆÆÆÆÆÆÆÀSµµµµµµµµµµµµµµµµµSÆÆÆÆÆWµµµµÆÆÆÆÆÆÆÆÆÆÆÆÆÆÆÆÆÆÆÆÆ            
            µµµµµµÞÆÆÆÆÆÆÆÆµµµµµµµµµµµµµµµµµµµµµµëÆÆÆÂµµµ6ÆÆÆØ6µµµµµµµµµµUNÆÆÆÆÆÆÆÆ        
            µµµµµµµÆÆÆÆÆÆÆÆ6µµµµµµµµµµµµµµµµµµµµµèÆÆÆÅµµµAÆÆÆµµµµµµµµµµµµµµµµ ÆÆÆÆÆÆÆ      
            µµµµµµµµµµµ9ÆÆÆÂµµµµµµµµµµµµµµµµµµµµµpÆÆÆÒD¶ÝœÆÆÆµµµµµµµµµµµµµµµy   ÆÆÆÆÆÆÆ    
            µµµµµµ6hëqHþØÆÆÆµµµµµµµµµµµµµµµµµµµµµßÆÆÆÆÆÆÆÆÆÆWµµµµµµµµµµµµµµµ6  ÆÆÆÆÆÆÆÆ    
            ÆÆÆÆÆÆÆÆÆÆÆÆÆÆÆÆÆHµµµµµµµµµµµµµµµµµµµµÆÆÆÆÆÆÆÆÆÆÆqµµµµµµµµµµµµµµµ5  ÆÆÆ   Æ     
        ÆÆÆÆÆÆÆÆÆÆÆÆÆÆÆÆR9µµÆÆÆµµµµµµµµµµµµµµµµµµµeÆÆÆeµµµµûÆÆqµµµµµµµµµµµµµµµ  ÆÆÆÆ         
    ÆÆÆÆÆÆÆÆÆÆÆÆÉÖ6µµµµµµµSÆÆÆµµµµµµµµµµµµµµµµµµÈÆÆ®µµµµµµÑÆŒµµµµµµµµµµµµµµ6 ÆÆÆÆ          
    ÆÆÆÆÆÆÆÆÆÆÆ6µµµµµµµµµµµµµkÆÆHµµµµµµµµµµµµµµµµµÆÆÞµµµµµµµàÆÆµµµµµµµµµµµµµµµÆÆÆÆ           
    ÆÆÆÆÆÆÆÆ    µµµµµµµµµµµµµµµUÆÆÆøFÝ9UUUS§DèøœÆÆÆÆgµµµµµµµµµŠÆÆÆqD6µµµµµµµµµÆÆÆ             
    ÆÆÆÆÆ       OµµµµµµµµµµµµµµµµäØÆÆÆÆÆÆÆÆÆÆÆÆÆÁ¶Ýµµµµµµµµµµµµµ9¶ØÆÆÆÆÆÆÆÆÆÆÆÆÆ              
    ÆÆ          UOµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµ9hZäU                
                OµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµO                
                µµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµ                 
                eµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµOOOµµOy                   
                µµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµOµ¾                              
                    yµµOOµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµy                                  
                                ¾O6OOµµµµµµµµµµµµµµµµµµµµµµµOOx                            
                            ¾OµOµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµµOµ                        
                        eµµµµµµµµµµyOO OµµµµµµµµUµµµUµµµµµµµµO  5OOµµµµµ                      
                    TµOµµµµµµ6Oµ      6µµµµµµµ8ÆÆšÜÆÆ¥µµµµµµµy   6Oµµµx                      
                    ÎµµµµµµµµOO       µµµµµµµµ8ÆÆÆÆþÆÆÆàµµµµµµxµOµµµµµ                       
                    eµµµµµµµµµµµOOOOµµµµµµµµ8ÆÆÆÆÆÒÆÆÆŒµµµµµµµµµµO5                        
                        ¾µµµµµµµµµµµµµµµµµµµµµ8ÆÆÆÆWÆÆÆÞµµµµµµµµµµy                          
                            xOµµµµµµµµµµµµµµµµ8ÆÆFÜÆÆZµµµµµµµµµOO                            
                                üµµµµµµµµµµµµµµµ®µµµSµµµµµµµµµµO                              
                                    µÎOOµµµµµµµµµµµµµµµµµµµµµµµµ                              
                                    ¾µµµµµµµµµµµµµµµµµµµµµµµµ                              
                                        yµµµµµµµµµµµµµµµµµµµµµµy                              
                                        eµµµµµµµµµµµµµµµµµµµµO                                
                                    OOµµµ¾  OµOµOOOµOyOµµµx                                
                                    µµµµµõ            yµµµO                                
                                    µµµµµµ             xµµµµ                                
                                    Oµµµµµ              eµµµµÎ                               
                                    Oµµµµµµ              xµµµµO                               
                                µµµµµµµ               µµµµµµ                               
                                OµµµµµµO              yOµµµµµ                               
                        yyOµOµµµµµµµµµOOÎ             µµµµµµµ6OµµOµµµµOµµT                  
                    OOµµµµµµµµµµµµµµµµµµµy           Oµµµµµµµµµµµµµµµµµµµy                 
                    µµµµµµµµµµµµµµµµµµµµµx          µµµµµµµµµµµµµµµµµµµµµµx                
                    eµµµµµµµµµµµµµµµµµµµµµU         yµµµµµµµµµµµµµµµµµµµµµµO                
                                                                                            
                                                                                            
";
const PROGRAM_TITLE: &str = "West-Mec Repair Shop Configuration Tool";
const INSTALL_TYPES: &[&str] = &[
    "Full Install",
    "Install Programs",
    "Remove Installed Programs (from this script)",
    "Remove Unecessary Programs (or bad ones)",
];
const INSTALL_PROGRAMS: &[&str] = &[
    "NZXT.CAM",
    "WhirlwindFX.SignalRgb",
    "CrystalRich.LockHunter",
    "Klocman.BulkCrapUninstaller",
    "valinet.ExplorerPatcher",
    "Git.Git",
    "Hibbiki.Chromium",
    "GitHub.GitHubDesktop",
    "Microsoft.VisualStudioCode",
    "Microsoft.VisualStudio.2019.BuildTools"
];


fn main() {
    if cfg!(target_os = "windows") {

        let menu = menu(vec![
            label(format!("{}", PROGRAM_TITLE).hex("F57E20")),
            label(""),
            scroll("Install Type", INSTALL_TYPES.iter().copied()),
            label(""),
            button("Start Install"),
        ]);
        run(&menu);
        {
            if !mut_menu(&menu).canceled() == true {
                println!("{}", format!("{}", PUNCHDOWN_PAUL).hex("F57E20"));

                let mm = mut_menu(&menu);
                println!("{}", mm.selection_value("Install Type"));
                println!("{}", mm.selected_item_name());

                handle_install_type(mm.selection_value("Install Type"));
            }
        }
    } else {
        Command::new("clear")
            .status()
            .unwrap();
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
        println!("{}", "Invalid Entry (idk what is wrong)".hex("FF0000"));
    }
}

fn winget_install(program_name: &str) {
    let installing_message: String = format!("Installing {}...", program_name.hex("F57E20"));
    let install_command: String = format!("winget install -e --id {} --silent --disable-interactivity --accept-package-agreements --accept-source-agreements", program_name);

    println!("{}\n", installing_message);

    let install_command_output = Command::new("powershell")
        .arg("-Command")
        .arg(install_command)
        .output()
        .expect("Failed to install program via winget...");

    if !install_command_output.status.success() {
        eprintln!("{}", format!("PowerShell returned an error:\n{}", String::from_utf8_lossy(&install_command_output.stdout)).hex("FF0000"));
    } else {
        println!("{}", String::from_utf8_lossy(&install_command_output.stdout));
    }
}

fn winget_remove(program_name: &str) {
    let removing_message: String = format!("Removing {}...", program_name.hex("FF0000"));
    let remove_command: String = format!("remove -e --id {}", program_name);

    println!("{}\n", removing_message);

    let remove_command_output = Command::new("powershell")
        .arg("-Command")
        .arg(remove_command)
        .output()
        .expect("Failed to remove program via winget...");

    if !remove_command_output.status.success() {
        eprintln!("{}", format!("PowerShell returned an error:\n{}", String::from_utf8_lossy(&remove_command_output.stdout)).hex("FF0000"));
    } else {
        println!("{}", String::from_utf8_lossy(&remove_command_output.stdout));
    }
}

fn install_programs() {
    for program in INSTALL_PROGRAMS {
        winget_install(program);
    }
}

fn handle_run_install_full() {
    println!("Starting Full Install...");

    install_programs();
}

fn handle_run_install_programs() {
    println!("Starting Programs Install...");

    install_programs();
}

fn handle_run_remove_installed_programs() {
    for program in INSTALL_PROGRAMS {
        winget_remove(program);
    }
}