use crate::system::utils::commands::templates;

pub fn storehelper(program_id: &str) {
    templates::admin("run", "success", "error", format!("$env:Path = [System.Environment]::GetEnvironmentVariable(\"Path\",\"Machine\") + \";\" + [System.Environment]::GetEnvironmentVariable(\"Path\",\"User\") || StoreAppHelper.exe /uninstall {}", program_id).as_str());
}
