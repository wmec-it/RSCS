use rscs::core::api::parse_config;

use crate::{
    conf::enums::{DelimiterType, MessageType},
    message,
};

pub async fn pre_run_checks() -> bool {
    let response = match reqwest::get("https://rscsapi.cadenf.com/latest-version").await {
        Ok(val) => match val.text().await {
            Ok(val) => val.to_string(),
            Err(_) => return false,
        },
        Err(_) => return false,
    };
    let response_content: &str = response.as_str();
    if env!("CARGO_PKG_VERSION")
        != parse_config(response_content.to_string(), "version")
            .expect("Error reading version from api...")
    {
        if parse_config(response_content.to_string(), "needs_update")
            .expect("Failed to read 'needs_update' value...")
            == "true"
        {
            message::error(
                MessageType::Print,
                message::add_delimiter(
                    DelimiterType::Layer1Error,
                    "You need to update this program!!!".to_string(),
                    Some(true),
                    None,
                    Some(true),
                )
                .unwrap()
                .as_str(),
            );
            return false;
        }
    }
    return true;
}
