use crate::conf::structs::{Delimiters, Theme};

// TODO: Make all ts read from a config file or something, this is getting too cluttered

pub const PUNCHDOWN_PAUL: &str = "
                                                                                            
                                                                                            
                                                                                            
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
pub const PROGRAM_TITLE: &str = "West-Mec Repair Shop Configuration Tool";

pub const MAIN_THEME: Theme = Theme {
    primary: "F57E20",
    success: "69FF90",
    error: "E63C3C",
    info: "69D0FF",
    warning: "FFFA69",
};
pub const DELIMITERS: Delimiters = Delimiters {
    layer1: "->",
    layer1info: "-[i]>",
    layer1error: "-[!]>",
    layer1success: "=>",
    layer1add: "+",
    layer2: "-->",
    layer2info: "--[i]>",
    layer2error: "--[!]>",
    layer2success: "==>",
    layer2add: "-[+]>",
    layer3: "--->",
    layer3info: "---[i]>",
    layer3error: "---[!]>",
    layer3success: "===>",
    layer3add: "--[+]>",
    frown: "-[ :( ]>",
};

//:& &[&str]
//:& &1 = Infinitely extendable and accessable
//:& [] = Defining Typed Vector
//:& &2 = Values can be strings as long as you want
//:& &3 = Force it to be infinite (extends &1)
pub const INSTALL_TYPES: &[&str] = &[
    "Full Install",
    "Install Programs",
    #[cfg(feature = "bambulabs")]
    "Full Install + Bambu Labs Slicer",
    #[cfg(feature = "bambulabs")]
    "Bambu Labs Slicer",
    "Remove Installed Programs (from this script)",
    "Remove Unecessary Programs (or bad ones)",
];
pub const INSTALL_PROGRAMS: &[&str] = &[
    "NZXT.CAM",
    "WhirlwindFX.SignalRgb",
    "CrystalRich.LockHunter",
    "Klocman.BulkCrapUninstaller",
    "valinet.ExplorerPatcher",
    "Git.Git",
    "Hibbiki.Chromium",
    "GitHub.GitHubDesktop",
    "Microsoft.VisualStudioCode",
    "Microsoft.VisualStudio.2019.BuildTools",
    "AngusJohnson.ResourceHacker",
    "Genymobile.scrcpy",
    "Google.PlatformTools",
    "Rufus.Rufus",
    "WinsiderSS.SystemInformer",
    "dorssel.usbipd-win",
    "Microsoft.DotNet.SDK.10",
    "TigerVNC.TigerVNC",
];
pub const REMOVE_PROGRAMS_STOREHELPER: &[&str] = &[
    "Microsoft.StartExperiencesApp_1.195.0.0_x64__8wekyb3d8bbwe",
    "Microsoft.PowerAutomateDesktop_1.0.2058.0_x64__8wekyb3d8bbwe",
    "Microsoft.BioEnrollment_10.0.19587.1000_neutral__cw5n1h2txyewy",
    "Microsoft.Win32WebViewHost_10.0.26100.1_neutral_neutral_cw5n1h2txyewy",
    "MicrosoftWindows.Client.CoreAI_1000.26100.7623.0_x64__cw5n1h2txyewy",
];
