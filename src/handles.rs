use serde::Deserialize;

use crate::{CONFIG, NAME_PATH_RESOLUTION, types};
use crate::utils::errors;

pub fn find_selected_type(install_type: &str) -> String {
    let paths = &NAME_PATH_RESOLUTION.lock().unwrap().paths;
    let names = &NAME_PATH_RESOLUTION.lock().unwrap().names;
    let found_index = names.iter().position(|r| *r == install_type).unwrap();
    let path = paths[found_index].as_str();
    return path.to_string()
}

pub fn handle_install_type(install_type: &str) {
    let config_path = find_selected_type(install_type);
    let json: ConfigStructure = serde_json::from_str(config_path.as_str()).unwrap();
    start(json);
}

pub fn start(config: ConfigStructure) {
    println!("{:#?}", config);
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructure {
    display_name: String,
    verbose: bool,
    debug: bool,
    does_back_up: bool,
    branding: ConfigStructureBranding,
    programs: ConfigStructurePrograms,
    debloat: ConfigStructureDebloat,
    tweaks: ConfigStructureTweaks,
    post_configuration: ConfigStructurePostConfiguration,
    extra: ConfigStructureExtra,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureBranding {
    enabled: bool,
    dark_mode_enabled: bool,
    logo: ConfigStructureBrandingLogo,
    windows: ConfigStructureBrandingWindows,
    chromium: ConfigStructureBrandingChromium,
    signal_rgb: ConfigStructureBrandingSignalRgb,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureBrandingLogo {
    enabled: bool,
    get: Vec<ConfigStructureBrandingLogoGet>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureBrandingLogoGet {
    method: String,
    locator: String,
    size: String,
    file_type: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureBrandingWindows {
    enabled: bool,
    wallpapers: ConfigStructureBrandingWindowsWallpapers,
    accent_color: ConfigStructureBrandingWindowsAccentColor,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureBrandingWindowsWallpapers {
    enabled: bool,
    get: Vec<ConfigStructureBrandingWindowsWallpapersGet>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureBrandingWindowsWallpapersGet {
    method: String,
    locator: String,
    size: String,
    file_type: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureBrandingWindowsAccentColor {
    enabled: bool,
    color: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureBrandingChromium {
    enabled: bool,
    icons: Vec<ConfigStructureBrandingChromiumIcons>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureBrandingChromiumIcons {
    replaces: String,
    method: String,
    locator: String,
    size: String,
    file_type: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureBrandingSignalRgb {
    enabled: bool,
    configuration: ConfigStructureBrandingSignalRgbConfiguration,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureBrandingSignalRgbConfiguration {
    enabled: bool,
    full: ConfigStructureBrandingSignalRgbConfigurationFull,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureBrandingSignalRgbConfigurationFull {
    enabled: bool,
    config_type: String,
    color: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructurePrograms {
    enabled: bool,
    install: ConfigStructureProgramsInstall,
    remove: ConfigStructureProgramsRemove,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureProgramsInstall {
    enabled: bool,
    winget: Vec<ConfigStructureProgramsInstallWinget>,
    powershell: Vec<ConfigStructureProgramsInstallPowershell>,
    appx_package: Vec<ConfigStructureProgramsInstallAppxPackage>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureProgramsInstallWinget {
    id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureProgramsInstallPowershell {
    id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureProgramsInstallAppxPackage {
    id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureProgramsRemove {
    enabled: bool,
    winget: Vec<ConfigStructureProgramsRemoveWinget>,
    powershell: Vec<ConfigStructureProgramsRemovePowershell>,
    appx_package: Vec<ConfigStructureProgramsRemoveAppxPackage>,
    bc_uninstaller: ConfigStructureProgramsRemoveBcUninstaller,
    manual: Vec<ConfigStructureProgramsRemoveManual>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureProgramsRemoveWinget {
    id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureProgramsRemovePowershell {
    id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureProgramsRemoveAppxPackage {
    id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureProgramsRemoveBcUninstaller {
    enabled: bool,
    store_helper: Vec<ConfigStructureProgramsRemoveBcUninstallerStoreHelper>,
    powershell: Vec<ConfigStructureProgramsRemoveBcUninstallerPowershell>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureProgramsRemoveBcUninstallerStoreHelper {
    id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureProgramsRemoveBcUninstallerPowershell {
    id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureProgramsRemoveManual {
    id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureDebloat {
    enabled: bool,
    bc_uninstaller: ConfigStructureDebloatBcUninstaller,
    powershell: ConfigStructureDebloatPowershell,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureDebloatBcUninstaller {
    enabled: bool,
    store_helper: Vec<ConfigStructureDebloatBcUninstallerStoreHelper>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureDebloatBcUninstallerStoreHelper {
    id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureDebloatPowershell {
    enabled: bool,
    appx_package: Vec<ConfigStructureDebloatPowershellAppxPackage>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureDebloatPowershellAppxPackage {
    id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureTweaks {
    enabled: bool,
    powershell: Vec<ConfigStructureTweaksPowershell>,
    registry: Vec<ConfigStructureTweaksRegistry>,
    power: Vec<ConfigStructureTweaksPower>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureTweaksPowershell {
    id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureTweaksRegistry {
    id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureTweaksPower {
    id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructurePostConfiguration {
    enabled: bool,
    explorer_patcher: ConfigStructurePostConfigurationExplorerPatcher,
    chromium: ConfigStructurePostConfigurationChromium,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructurePostConfigurationExplorerPatcher {
    enabled: bool,
    import_config: ConfigStructurePostConfigurationExplorerPatcherImportConfig,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructurePostConfigurationExplorerPatcherImportConfig {
    enabled: bool,
    config_type: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructurePostConfigurationChromium {
    enabled: bool,
    save_session_on_close: bool,
    extensions: Vec<ConfigStructurePostConfigurationChromiumExtensions>,
    security: ConfigStructurePostConfigurationChromiumSecurity,
    downloads: ConfigStructurePostConfigurationChromiumDownloads,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructurePostConfigurationChromiumExtensions {
    enabled: bool,
    id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructurePostConfigurationChromiumSecurity {
    level: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructurePostConfigurationChromiumDownloads {
    ask_to_download: bool,
    default_download_directory: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureExtra {
    enabled: bool,
    scripts: Vec<ConfigStructureExtraScripts>,
    commands: Vec<ConfigStructureExtraCommands>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureExtraScripts {
    run_message: String,
    method: String,
    locator: String,
    exec_method: String,
    privileges: String,
    completion_message: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureExtraCommands {
    run_message: String,
    privileges: String,
    command: String,
    completion_message: String,
}