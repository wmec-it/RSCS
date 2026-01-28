use serde::{Deserialize, Serialize};

//:& &'static str = Compacts &str to static length after definition
//:& &'a str      = Only defined for as long as what uses it is defined
//:& <'a>         = Defines the same as above, but for the entire Struct
#[allow(dead_code)]
pub struct Theme<'a> {
    pub primary: &'a str,
    pub success: &'a str,
    pub error: &'a str,
    pub info: &'a str,
    pub warning: &'a str,
}

#[allow(dead_code)]
pub struct Delimiters<'a> {
    pub layer1: &'a str,
    pub layer1info: &'a str,
    pub layer1error: &'a str,
    pub layer1success: &'a str,
    pub layer1add: &'a str,
    pub layer2: &'a str,
    pub layer2info: &'a str,
    pub layer2error: &'a str,
    pub layer2success: &'a str,
    pub layer2add: &'a str,
    pub layer3: &'a str,
    pub layer3info: &'a str,
    pub layer3error: &'a str,
    pub layer3success: &'a str,
    pub layer3add: &'a str,
    pub frown: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MenuOption {
    pub display_name: String,
    pub verbose: bool,
    pub debug: bool,
    pub does_back_up: bool,

    pub branding: Branding,
    pub programs: Programs,
    pub debloat: Debloat,
    pub tweaks: Tweaks,
    pub post_configuration: PostConfiguration,
    pub extra: Extra,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Branding {
    pub enabled: bool,
    pub dark_mode_enabled: bool,
    pub logo: Logo,
    pub windows: WindowsBranding,
    pub chromium: ChromiumBranding,
    pub signal_rgb: SignalRgb,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Logo {
    pub enabled: bool,
    pub get: Vec<GetItem>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct WindowsBranding {
    pub enabled: bool,
    pub wallpapers: Wallpapers,
    pub accent_color: AccentColor,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Wallpapers {
    pub enabled: bool,
    pub get: Vec<GetItem>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AccentColor {
    pub enabled: bool,
    pub color: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ChromiumBranding {
    pub enabled: bool,
    pub icons: Vec<IconReplace>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct IconReplace {
    pub replaces: String,
    pub method: String,
    pub locator: String,
    pub size: String,
    pub file_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SignalRgb {
    pub enabled: bool,
    pub configuration: SignalRgbConfiguration,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SignalRgbConfiguration {
    pub enabled: bool,
    pub full: SignalRgbFull,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SignalRgbFull {
    pub enabled: bool,
    pub config_type: String,
    pub color: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetItem {
    pub method: String,
    pub locator: String,
    pub size: String,
    pub file_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Programs {
    pub enabled: bool,
    pub install: ProgramInstall,
    pub remove: ProgramRemove,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ProgramInstall {
    pub enabled: bool,
    pub winget: Vec<IdItem>,
    pub powershell: Vec<IdItem>,
    pub appx_package: Vec<IdItem>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ProgramRemove {
    pub enabled: bool,
    pub winget: Vec<IdItem>,
    pub powershell: Vec<IdItem>,
    pub appx_package: Vec<IdItem>,
    pub bc_uninstaller: BcUninstallerRemove,
    pub manual: Vec<IdItem>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BcUninstallerRemove {
    pub enabled: bool,
    pub store_helper: Vec<IdItem>,
    pub powershell: Vec<IdItem>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Debloat {
    pub enabled: bool,
    pub bc_uninstaller: DebloatBcUninstaller,
    pub powershell: DebloatPowershell,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DebloatBcUninstaller {
    pub enabled: bool,
    pub store_helper: Vec<IdItem>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DebloatPowershell {
    pub enabled: bool,
    pub appx_package: Vec<IdItem>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Tweaks {
    pub enabled: bool,
    pub powershell: Vec<IdItem>,
    pub registry: Vec<IdItem>,
    pub power: Vec<IdItem>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PostConfiguration {
    pub enabled: bool,
    pub explorer_patcher: ExplorerPatcher,
    pub chromium: PostChromium,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ExplorerPatcher {
    pub enabled: bool,
    pub import_config: ImportConfig,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ImportConfig {
    pub enabled: bool,
    pub config_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PostChromium {
    pub enabled: bool,
    pub save_session_on_close: bool,
    pub extensions: Vec<ExtensionItem>,
    pub security: ChromiumSecurity,
    pub downloads: ChromiumDownloads,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ExtensionItem {
    pub enabled: bool,
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ChromiumSecurity {
    pub level: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ChromiumDownloads {
    pub ask_to_download: bool,
    pub default_download_directory: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Extra {
    pub enabled: bool,
    pub scripts: Vec<ExtraScript>,
    pub commands: Vec<ExtraCommand>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ExtraScript {
    pub run_message: String,
    pub method: String,
    pub locator: String,
    pub exec_method: String,
    pub privileges: String,
    pub completion_message: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ExtraCommand {
    pub run_message: String,
    pub privileges: String,
    pub command: String,
    pub completion_message: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct IdItem {
    pub id: String,
}