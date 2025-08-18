use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};

/// Generate a SHA-256 hash of the input data
pub fn sha256_hash(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    format!("0x{:x}", result)
}

/// Generate a SHA-256 hash of a string
pub fn sha256_hash_string(data: &str) -> String {
    sha256_hash(data.as_bytes())
}

/// Get current timestamp in ISO 8601 format
pub fn get_current_timestamp() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    chrono::DateTime::from_timestamp(now as i64, 0)
        .unwrap()
        .to_rfc3339()
}

/// Get current timestamp as Unix timestamp
pub fn get_current_unix_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

/// Format timestamp for display
pub fn format_timestamp(timestamp: &str) -> String {
    // Try to parse ISO 8601 timestamp and format it nicely
    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(timestamp) {
        dt.format("%Y-%m-%d %H:%M:%S UTC").to_string()
    } else {
        timestamp.to_string()
    }
}

/// Validate email format
pub fn is_valid_email(email: &str) -> bool {
    use regex::Regex;
    lazy_static::lazy_static! {
        static ref EMAIL_REGEX: Regex = Regex::new(
            r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$"
        ).unwrap();
    }
    EMAIL_REGEX.is_match(email)
}

/// Validate phone number format (basic validation)
pub fn is_valid_phone(phone: &str) -> bool {
    use regex::Regex;
    lazy_static::lazy_static! {
        static ref PHONE_REGEX: Regex = Regex::new(r"^[\+]?[1-9][\d]{0,15}$").unwrap();
    }
    PHONE_REGEX.is_match(phone)
}

/// Generate a random ID
pub fn generate_random_id() -> String {
    use rand::{thread_rng, Rng};
    use rand::distributions::Alphanumeric;
    
    let mut rng = thread_rng();
    let id: String = (0..16)
        .map(|_| rng.sample(Alphanumeric) as char)
        .collect();
    
    format!("id-{}", id)
}

/// Convert bytes to hex string
pub fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>()
}

/// Convert hex string to bytes
pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    if hex.len() % 2 != 0 {
        return Err("Hex string must have even length".into());
    }
    
    let mut bytes = Vec::new();
    for i in (0..hex.len()).step_by(2) {
        let byte = u8::from_str_radix(&hex[i..i+2], 16)?;
        bytes.push(byte);
    }
    
    Ok(bytes)
}

/// Truncate string to specified length
pub fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len])
    }
}

/// Capitalize first letter of string
pub fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().chain(chars).collect(),
    }
}

/// Format file size in human readable format
pub fn format_file_size(bytes: u64) -> String {
    const UNITS: [&str; 4] = ["B", "KB", "MB", "GB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    format!("{:.1} {}", size, UNITS[unit_index])
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_sha256_hash() {
        let data = "Hello, World!";
        let hash = sha256_hash_string(data);
        assert_eq!(hash.len(), 66); // 0x + 64 hex chars
        assert!(hash.starts_with("0x"));
    }
    
    #[test]
    fn test_timestamp() {
        let timestamp = get_current_timestamp();
        assert!(timestamp.contains("T"));
        assert!(timestamp.contains("Z"));
    }
    
    #[test]
    fn test_email_validation() {
        assert!(is_valid_email("test@example.com"));
        assert!(is_valid_email("user.name+tag@domain.co.uk"));
        assert!(!is_valid_email("invalid-email"));
        assert!(!is_valid_email("@domain.com"));
    }
    
    #[test]
    fn test_phone_validation() {
        assert!(is_valid_phone("1234567890"));
        assert!(is_valid_phone("+1234567890"));
        assert!(!is_valid_phone("abc"));
        assert!(!is_valid_phone(""));
    }
    
    #[test]
    fn test_random_id() {
        let id1 = generate_random_id();
        let id2 = generate_random_id();
        
        assert!(id1.starts_with("id-"));
        assert!(id1.len() == 20); // "id-" + 16 chars
        assert_ne!(id1, id2);
    }
    
    #[test]
    fn test_hex_conversion() {
        let original = vec![0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF];
        let hex = bytes_to_hex(&original);
        let converted = hex_to_bytes(&hex).unwrap();
        
        assert_eq!(original, converted);
    }
    
    #[test]
    fn test_string_utils() {
        assert_eq!(truncate_string("Hello World", 5), "Hello...");
        assert_eq!(truncate_string("Short", 10), "Short");
        assert_eq!(capitalize_first("hello"), "Hello");
        assert_eq!(capitalize_first(""), "");
    }
    
    #[test]
    fn test_file_size_formatting() {
        assert_eq!(format_file_size(1024), "1.0 KB");
        assert_eq!(format_file_size(1048576), "1.0 MB");
        assert_eq!(format_file_size(500), "500.0 B");
    }
}
