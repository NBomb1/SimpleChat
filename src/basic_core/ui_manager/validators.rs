/// Fast validators for ui.
use std::net::IpAddr;

/// Checks IP. supports IPv4, IPv6.
pub fn validate_ip(text: &str) -> bool {
    let text = text.trim();
    if text == "localhost" || text == "ip6-localhost" {
        return true;
    }
    text.parse::<IpAddr>().is_ok()
}

/// Port check.Range 1-65535.
pub fn validate_port(text: &str) -> bool {
    let text = text.trim();
    if let Ok(port) = text.parse::<u16>() {
        port > 0
    } else {
        false
    }
}

/// username check
/// length 3-20 symbols
pub fn validate_username(text: &str) -> bool {
    let t = text.trim();
    t.len() >= 3 && t.len() <= 20 && t.chars().all(char::is_alphanumeric)
}

