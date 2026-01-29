use serde::Deserialize;

use crate::NAME_PATH_RESOLUTION;

pub fn find_selected_type(install_type: &str) -> String {
    let npr = NAME_PATH_RESOLUTION.lock().unwrap();
    let names = &npr.names;
    println!("{:#?}", names);
    let found_index = names.iter().position(|r| *r == install_type).unwrap();
    println!("Found Index: {}", found_index);
    let paths = &npr.paths;
    println!("{:#?}", paths);
    let path = paths[found_index].as_str();
    println!("Path: {}", format!("./menu_options/{}", path));
    return format!("./menu_options/{}", path);
}

pub fn handle_install_type(install_type: &str) {
    println!("{}", install_type);
    let config_path = find_selected_type(install_type);
    println!("Finding selected type... {}", config_path);
    let json_file = std::fs::read_to_string(&config_path)
        .unwrap_or_else(|e| panic!("Failed to read {}: {}", config_path, e));
    let json: ConfigStructure = serde_json::from_str(json_file.as_str())
        .unwrap_or_else(|e| panic!("Invalid JSON in {}: {}", config_path, e));
    println!("Converting json to Point...");
    start(json);
}

pub fn start(config: ConfigStructure) {
    println!("{:#?}", config);
}

#[allow(unused)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructure {
    pub display_name: String,
    pub verbose: bool,
    pub debug: bool,
    pub does_back_up: bool,
    pub branding: ConfigStructureBranding,
    pub programs: ConfigStructurePrograms,
    pub debloat: ConfigStructureDebloat,
    pub tweaks: ConfigStructureTweaks,
    pub post_configuration: ConfigStructurePostConfiguration,
    pub extra: ConfigStructureExtra,
}
#[allow(unused)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureBranding {
    pub enabled: bool,
    pub dark_mode_enabled: bool,
    pub logo: ConfigStructureBrandingLogo,
    pub windows: ConfigStructureBrandingWindows,
    pub chromium: ConfigStructureBrandingChromium,
    pub signal_rgb: ConfigStructureBrandingSignalRgb,
}
#[allow(unused)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureBrandingLogo {
    pub enabled: bool,
    pub get: Vec<ConfigStructureBrandingLogoGet>,
}
#[allow(unused)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureBrandingLogoGet {
    pub method: String,
    pub locator: String,
    pub size: String,
    pub file_type: String,
}
#[allow(unused)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureBrandingWindows {
    pub enabled: bool,
    pub wallpapers: ConfigStructureBrandingWindowsWallpapers,
    pub accent_color: ConfigStructureBrandingWindowsAccentColor,
}
#[allow(unused)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureBrandingWindowsWallpapers {
    pub enabled: bool,
    pub get: Vec<ConfigStructureBrandingWindowsWallpapersGet>,
}
#[allow(unused)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureBrandingWindowsWallpapersGet {
    pub method: String,
    pub locator: String,
    pub size: String,
    pub file_type: String,
}
#[allow(unused)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureBrandingWindowsAccentColor {
    pub enabled: bool,
    pub color: String,
}
#[allow(unused)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureBrandingChromium {
    pub enabled: bool,
    pub icons: Vec<ConfigStructureBrandingChromiumIcons>,
}
#[allow(unused)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureBrandingChromiumIcons {
    pub replaces: String,
    pub method: String,
    pub locator: String,
    pub size: String,
    pub file_type: String,
}
#[allow(unused)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureBrandingSignalRgb {
    pub enabled: bool,
    pub configuration: ConfigStructureBrandingSignalRgbConfiguration,
}
#[allow(unused)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureBrandingSignalRgbConfiguration {
    pub enabled: bool,
    pub full: ConfigStructureBrandingSignalRgbConfigurationFull,
}
#[allow(unused)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureBrandingSignalRgbConfigurationFull {
    pub enabled: bool,
    pub config_type: String,
    pub color: String,
}
#[allow(unused)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructurePrograms {
    pub enabled: bool,
    pub install: ConfigStructureProgramsInstall,
    pub remove: ConfigStructureProgramsRemove,
}
#[allow(unused)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureProgramsInstall {
    pub enabled: bool,
    pub winget: Vec<ConfigStructureProgramsInstallWinget>,
    pub powershell: Vec<ConfigStructureProgramsInstallPowershell>,
    pub appx_package: Vec<ConfigStructureProgramsInstallAppxPackage>,
}
#[allow(unused)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureProgramsInstallWinget {
    pub id: String,
}
#[allow(unused)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureProgramsInstallPowershell {
    pub id: String,
}
#[allow(unused)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureProgramsInstallAppxPackage {
    pub id: String,
}
#[allow(unused)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureProgramsRemove {
    pub enabled: bool,
    pub winget: Vec<ConfigStructureProgramsRemoveWinget>,
    pub powershell: Vec<ConfigStructureProgramsRemovePowershell>,
    pub appx_package: Vec<ConfigStructureProgramsRemoveAppxPackage>,
    pub bc_uninstaller: ConfigStructureProgramsRemoveBcUninstaller,
    pub manual: Vec<ConfigStructureProgramsRemoveManual>,
}
#[allow(unused)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureProgramsRemoveWinget {
    pub id: String,
}
#[allow(unused)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureProgramsRemovePowershell {
    pub id: String,
}
#[allow(unused)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureProgramsRemoveAppxPackage {
    pub id: String,
}
#[allow(unused)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureProgramsRemoveBcUninstaller {
    pub enabled: bool,
    pub store_helper: Vec<ConfigStructureProgramsRemoveBcUninstallerStoreHelper>,
    pub powershell: Vec<ConfigStructureProgramsRemoveBcUninstallerPowershell>,
}
#[allow(unused)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureProgramsRemoveBcUninstallerStoreHelper {
    pub id: String,
}
#[allow(unused)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureProgramsRemoveBcUninstallerPowershell {
    pub id: String,
}
#[allow(unused)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureProgramsRemoveManual {
    pub id: String,
}
#[allow(unused)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureDebloat {
    pub enabled: bool,
    pub bc_uninstaller: ConfigStructureDebloatBcUninstaller,
    pub powershell: ConfigStructureDebloatPowershell,
}
#[allow(unused)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureDebloatBcUninstaller {
    pub enabled: bool,
    pub store_helper: Vec<ConfigStructureDebloatBcUninstallerStoreHelper>,
}
#[allow(unused)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureDebloatBcUninstallerStoreHelper {
    pub id: String,
}
#[allow(unused)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureDebloatPowershell {
    pub enabled: bool,
    pub appx_package: Vec<ConfigStructureDebloatPowershellAppxPackage>,
}
#[allow(unused)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureDebloatPowershellAppxPackage {
    pub id: String,
}
#[allow(unused)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureTweaks {
    pub enabled: bool,
    pub powershell: Vec<ConfigStructureTweaksPowershell>,
    pub registry: Vec<ConfigStructureTweaksRegistry>,
    pub power: Vec<ConfigStructureTweaksPower>,
}
#[allow(unused)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureTweaksPowershell {
    pub id: String,
}
#[allow(unused)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureTweaksRegistry {
    pub id: String,
}
#[allow(unused)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureTweaksPower {
    pub id: String,
}
#[allow(unused)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructurePostConfiguration {
    pub enabled: bool,
    pub explorer_patcher: ConfigStructurePostConfigurationExplorerPatcher,
    pub chromium: ConfigStructurePostConfigurationChromium,
}
#[allow(unused)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructurePostConfigurationExplorerPatcher {
    pub enabled: bool,
    pub import_config: ConfigStructurePostConfigurationExplorerPatcherImportConfig,
}
#[allow(unused)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructurePostConfigurationExplorerPatcherImportConfig {
    pub enabled: bool,
    pub config_type: String,
}
#[allow(unused)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructurePostConfigurationChromium {
    pub enabled: bool,
    pub save_session_on_close: bool,
    pub extensions: Vec<ConfigStructurePostConfigurationChromiumExtensions>,
    pub security: ConfigStructurePostConfigurationChromiumSecurity,
    pub downloads: ConfigStructurePostConfigurationChromiumDownloads,
}
#[allow(unused)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructurePostConfigurationChromiumExtensions {
    pub enabled: bool,
    pub id: String,
}
#[allow(unused)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructurePostConfigurationChromiumSecurity {
    pub level: String,
}
#[allow(unused)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructurePostConfigurationChromiumDownloads {
    pub ask_to_download: bool,
    pub default_download_directory: String,
}
#[allow(unused)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureExtra {
    pub enabled: bool,
    pub scripts: Vec<ConfigStructureExtraScripts>,
    pub commands: Vec<ConfigStructureExtraCommands>,
}
#[allow(unused)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureExtraScripts {
    pub run_message: String,
    pub method: String,
    pub locator: String,
    pub exec_method: String,
    pub privileges: String,
    pub completion_message: String,
}
#[allow(unused)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureExtraCommands {
    pub run_message: String,
    pub privileges: String,
    pub command: String,
    pub completion_message: String,
}
