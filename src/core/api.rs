use regex::Regex;

pub fn parse_config(response: String, key: &str) -> Result<String, std::io::Error> {
    let regex = Regex::new(r"(?<key>\w+)=(?<value>[a-zA-Z0-9._-]+)").unwrap();
    for cap in regex.captures_iter(&response) {
        if let Some(captured_key) = cap.name("key") {
            if captured_key.as_str() == key {
                if let Some(captured_value) = cap.name("value") {
                    return Ok(captured_value.as_str().to_string());
                }
            }
        }
    }
    Err(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "Key not found",
    ))
}
