use crate::conf::vars::INSTALL_PROGRAMS;

// TODO: Calculate length via future config files / scripts (scanning directory + configs)
pub fn length() -> Result<u64, std::io::Error> {
    let programs_length: u8 = INSTALL_PROGRAMS.len() as u8;
    let number_of_tweaks: u8 = 0;

    let final_length: u64 = (programs_length + number_of_tweaks) as u64;
    return Ok(final_length);
}
