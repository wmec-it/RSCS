use serde::{Deserialize, Serialize};

#[allow(unused)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default)]
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
#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureBranding {
    pub enabled: bool,
    pub dark_mode_enabled: bool,
    pub custom_taskbar_selection: bool,
    pub clear_desktop: bool,
    pub logo: ConfigStructureBrandingLogo,
    pub windows: ConfigStructureBrandingWindows,
    pub chromium: ConfigStructureBrandingChromium,
    pub signal_rgb: ConfigStructureBrandingSignalRgb,
}
#[allow(unused)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureBrandingLogo {
    pub enabled: bool,
    pub get: Vec<ConfigStructureBrandingLogoGet>,
}
#[allow(unused)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureBrandingLogoGet {
    pub icon_path: String,
    pub size: String,
    pub file_type: String,
    pub replaces: String,
    pub executable_path: String,
    pub desktop_shortcut_path: String,
    pub taskbar_shortcut_path: String,
    pub args: String,
}
#[allow(unused)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureBrandingWindows {
    pub enabled: bool,
    pub wallpapers: ConfigStructureBrandingWindowsWallpapers,
    pub accent_color: ConfigStructureBrandingWindowsAccentColor,
}
#[allow(unused)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureBrandingWindowsWallpapers {
    pub enabled: bool,
    pub get: Vec<ConfigStructureBrandingWindowsWallpapersGet>,
}
#[allow(unused)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureBrandingWindowsWallpapersGet {
    pub method: String,
    pub locator: String,
    pub size: String,
    pub file_type: String,
}
#[allow(unused)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureBrandingWindowsAccentColor {
    pub enabled: bool,
    pub color: String,
}
#[allow(unused)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureBrandingChromium {
    pub enabled: bool,
    pub icons: Vec<ConfigStructureBrandingChromiumIcons>,
}
#[allow(unused)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureBrandingChromiumIcons {
    pub replaces: String,
    pub method: String,
    pub locator: String,
    pub size: String,
    pub file_type: String,
}
#[allow(unused)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureBrandingSignalRgb {
    pub enabled: bool,
    pub configuration: ConfigStructureBrandingSignalRgbConfiguration,
}
#[allow(unused)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureBrandingSignalRgbConfiguration {
    pub enabled: bool,
    pub full: ConfigStructureBrandingSignalRgbConfigurationFull,
}
#[allow(unused)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureBrandingSignalRgbConfigurationFull {
    pub enabled: bool,
    pub config_type: String,
    pub color: String,
}
#[allow(unused)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructurePrograms {
    pub enabled: bool,
    pub install: ConfigStructureProgramsInstall,
    pub remove: ConfigStructureProgramsRemove,
}
#[allow(unused)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureProgramsInstall {
    pub enabled: bool,
    pub winget: Vec<ConfigStructureProgramsInstallWinget>,
    pub powershell: Vec<ConfigStructureProgramsInstallPowershell>,
    pub appx_package: Vec<ConfigStructureProgramsInstallAppxPackage>,
}
#[allow(unused)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureProgramsInstallWinget {
    pub name: String,
    pub id: String,
    pub source: String,
}
#[allow(unused)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureProgramsInstallPowershell {
    pub name: String,
    pub id: String,
}
#[allow(unused)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureProgramsInstallAppxPackage {
    pub name: String,
    pub id: String,
}
#[allow(unused)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default)]
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
#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureProgramsRemoveWinget {
    pub name: String,
    pub id: String,
}
#[allow(unused)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureProgramsRemovePowershell {
    pub name: String,
    pub id: String,
}
#[allow(unused)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureProgramsRemoveAppxPackage {
    pub name: String,
    pub id: String,
}
#[allow(unused)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureProgramsRemoveBcUninstaller {
    pub enabled: bool,
    pub store_helper: Vec<ConfigStructureProgramsRemoveBcUninstallerStoreHelper>,
    pub powershell: Vec<ConfigStructureProgramsRemoveBcUninstallerPowershell>,
}
#[allow(unused)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureProgramsRemoveBcUninstallerStoreHelper {
    pub name: String,
    pub id: String,
}
#[allow(unused)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureProgramsRemoveBcUninstallerPowershell {
    pub name: String,
    pub id: String,
}
#[allow(unused)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureProgramsRemoveManual {
    pub official_uninstaller: bool,
    pub name: String,
    pub id: String,
    pub command: ConfigStructureProgramsRemoveManualCommand,
}
#[allow(unused)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureProgramsRemoveManualCommand {
    pub program: String,
    pub args: Vec<ConfigStructureProgramsRemoveManualCommandArgs>,
}
#[allow(unused)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureProgramsRemoveManualCommandArgs {
    pub arg: String,
}
#[allow(unused)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureDebloat {
    pub enabled: bool,
    pub bc_uninstaller: ConfigStructureDebloatBcUninstaller,
    pub powershell: ConfigStructureDebloatPowershell,
}
#[allow(unused)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureDebloatBcUninstaller {
    pub enabled: bool,
    pub store_helper: Vec<ConfigStructureDebloatBcUninstallerStoreHelper>,
}
#[allow(unused)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureDebloatBcUninstallerStoreHelper {
    pub name: String,
    pub id: String,
}
#[allow(unused)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureDebloatPowershell {
    pub enabled: bool,
    pub appx_package: Vec<ConfigStructureDebloatPowershellAppxPackage>,
}
#[allow(unused)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureDebloatPowershellAppxPackage {
    pub name: String,
    pub id: String,
}
#[allow(unused)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureTweaks {
    pub enabled: bool,
    pub powershell: Vec<ConfigStructureTweaksPowershell>,
    pub registry: Vec<ConfigStructureTweaksRegistry>,
    pub power: Vec<ConfigStructureTweaksPower>,
}
#[allow(unused)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureTweaksPowershell {
    pub name: String,
    pub id: String,
}
#[allow(unused)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureTweaksRegistry {
    pub name: String,
    pub id: String,
}
#[allow(unused)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureTweaksPower {
    pub name: String,
    pub id: String,
}
#[allow(unused)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructurePostConfiguration {
    pub enabled: bool,
    pub explorer_patcher: ConfigStructurePostConfigurationExplorerPatcher,
    pub chromium: ConfigStructurePostConfigurationChromium,
}
#[allow(unused)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructurePostConfigurationExplorerPatcher {
    pub enabled: bool,
    pub import_config: ConfigStructurePostConfigurationExplorerPatcherImportConfig,
}
#[allow(unused)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructurePostConfigurationExplorerPatcherImportConfig {
    pub enabled: bool,
    pub config_type: String,
}
#[allow(unused)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructurePostConfigurationChromium {
    pub enabled: bool,
    pub save_session_on_close: bool,
    pub extensions: Vec<ConfigStructurePostConfigurationChromiumExtensions>,
    pub security: ConfigStructurePostConfigurationChromiumSecurity,
    pub downloads: ConfigStructurePostConfigurationChromiumDownloads,
}
#[allow(unused)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructurePostConfigurationChromiumExtensions {
    pub enabled: bool,
    pub name: String,
    pub id: String,
}
#[allow(unused)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructurePostConfigurationChromiumSecurity {
    pub level: String,
}
#[allow(unused)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructurePostConfigurationChromiumDownloads {
    pub enabled: bool,
    pub ask_to_download: bool,
    pub default_download_directory: String,
}
#[allow(unused)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureExtra {
    pub enabled: bool,
    pub scripts: Vec<ConfigStructureExtraScripts>,
    pub commands: Vec<ConfigStructureExtraCommands>,
}
#[allow(unused)]
#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default)]
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
#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigStructureExtraCommands {
    pub run_message: String,
    pub privileges: String,
    pub command: String,
    pub completion_message: String,
}
