use {
    std::{env, io},
    winresource::WindowsResource,
};

fn main() -> io::Result<()> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=assets/punchdown_paul.ico");

    let version = env::var("CARGO_PKG_VERSION").unwrap();

    if env::var_os("CARGO_CFG_WINDOWS").is_some() {
        WindowsResource::new()
            .set_icon("assets/punchdown_paul.ico")
            .set("FileDescription", "Repair Shop Configuration Superintendant CLI program.")
            .set("ProductName", "Repair Shop Configuration Superintendant")
            .set("CompanyName", "West-Mec Southwest, IT Security")
            .set("LegalCopyright", "© 2025 West-Mec Southwest, IT Security")
            .set("OriginalFilename", "rscs.exe")
            .set("InternalName", "rscs")
            .set("FileVersion",  &format!("{}.0", version))
            .set("ProductVersion", &version)
            .compile()?;
    }
    Ok(())
}
