use rscs::core::structs::theme::Theme;

use crate::conf::structs::Delimiters;

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

#[allow(dead_code)]
pub struct DebloatUninstallProgramsObject {
    pub appx_packages: &'static [&'static str],
}
#[allow(dead_code)]
pub static DEBLOAT_UNINSTALL_APPX_PACKAGES: DebloatUninstallProgramsObject =
    DebloatUninstallProgramsObject {
        appx_packages: &[
            "Microsoft.StartExperiencesApp_1.195.0.0_x64__8wekyb3d8bbwe",
            "Microsoft.PowerAutomateDesktop_1.0.2058.0_x64__8wekyb3d8bbwe",
            "Microsoft.BioEnrollment_10.0.19587.1000_neutral__cw5n1h2txyewy",
            "Microsoft.Win32WebViewHost_10.0.26100.1_neutral_neutral_cw5n1h2txyewy",
            "MicrosoftWindows.Client.CoreAI_1000.26100.7623.0_x64__cw5n1h2txyewy",
            "Microsoft.XboxGamingOverlay_7.325.11061.0_x64__8wekyb3d8bbwe",
            "Microsoft.WindowsFeedbackHub_1.2512.16303.0_x64__8wekyb3d8bbwe",
            "Microsoft.GetHelp_10.2409.33293.0_x64__8wekyb3d8bbwe",
            "microsoft.windowscommunicationsapps_16005.14326.22342.0_x64__8wekyb3d8bbwe",
            "Microsoft.BingSearch_1.1.40.0_x64__8wekyb3d8bbwe",
            "Clipchamp.Clipchamp_4.5.10020.0_x64__yxz26nhyzhsrt",
            "Microsoft.Edge.GameAssist_1.0.3590.0_x64__8wekyb3d8bbwe",
            "Microsoft.People_10.2202.100.0_x64__8wekyb3d8bbwe",
            "Microsoft.MicrosoftStickyNotes_4.0.6104.0_x64__8wekyb3d8bbwe",
            "Microsoft.Todos_0.153.5851.0_x64__8wekyb3d8bbwe",
            "Microsoft.BingWeather_3.2.3.0_x64__8wekyb3d8bbwe",
            "Microsoft.MSPaint_6.2410.13017.0_x64__8wekyb3d8bbwe",
            "MicrosoftCorporationII.QuickAssist_2.0.29.0_x64__8wekyb3d8bbwe",
            "Microsoft.SkypeApp_15.150.3125.0_x64__kzf8qxf38zg5c",
            "Microsoft.MicrosoftSolitaireCollection_4.25.1130.0_x64__8wekyb3d8bbwe",
            "SpotifyAB.SpotifyMusic_1.281.264.0_x64__zpdnekdrzrea0",
            "Microsoft.StartExperiencesApp_1.218.0.0_x64__8wekyb3d8bbwe",
            "Microsoft.WindowsMaps_1.0.65.0_x64__8wekyb3d8bbwe",
            "Microsoft.GamingApp_2512.1001.36.0_x64__8wekyb3d8bbwe",
            "Microsoft.XboxApp_48.104.4001.0_x64__8wekyb3d8bbwe",
            "Microsoft.XboxGameOverlay_1.54.4001.0_x64__8wekyb3d8bbwe",
            "Microsoft.XboxSpeechToTextOverlay_1.21.13002.0_x64__8wekyb3d8bbwe",
            "Microsoft.XboxGameCallableUI_1000.25128.1000.0_neutral_neutral_cw5n1h2txyewy",
            "Microsoft.XboxIdentityProvider_12.130.16001.0_x64__8wekyb3d8bbwe",
            "Microsoft.Xbox.TCUI_1.24.10001.0_x64__8wekyb3d8bbwe",
            "MicrosoftWindows.CrossDevice_0.25112.60.0_x64__cw5n1h2txyewy",
            "Microsoft.Windows.DevHome_0.0.0.0_x64__8wekyb3d8bbwe",
            "Microsoft.MicrosoftOfficeHub_19.2601.56031.0_x64__8wekyb3d8bbwe",
            "MSTeams_26005.204.4249.1621_x64__8wekyb3d8bbwe",
            "Microsoft.BingNews_4.55.62231.0_x64__8wekyb3d8bbwe",
            "Microsoft.OutlookForWindows_1.2026.114.100_x64__8wekyb3d8bbwe",
            "Microsoft.YourPhone_0.25122.50.0_x64__8wekyb3d8bbwe",
        ],
    };
