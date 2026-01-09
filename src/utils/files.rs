use std::env;
use std::fs::File;
use std::io::Write;

pub fn create_temp_file(
    local_file_path_bytes: &[u8],
    output_file_name: &str,
) -> std::io::Result<String> {
    let temp_path = env::temp_dir().join(&output_file_name);
    let mut file = File::create(&temp_path)?;
    println!(
        "Writing file: {} to {}",
        output_file_name,
        &temp_path.to_string_lossy().into_owned()
    );
    file.write_all(local_file_path_bytes)?;
    println!(
        "Successfully wrote file: {} to {}",
        output_file_name,
        &temp_path.to_string_lossy().into_owned()
    );
    Ok(temp_path.to_string_lossy().into_owned())
}
