#[allow(dead_code)]
pub fn official_uninstaller() {
    std::process::Command::new("\"C:\\Users\\admin\\AppData\\Local\\Microsoft\\OneDrive\\26.002.0105.0001\\OneDriveSetup.exe").arg("/uninstall").output().expect("");
}
