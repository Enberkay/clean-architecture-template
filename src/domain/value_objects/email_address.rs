#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EmailAddress(String);

impl EmailAddress {
    pub fn new(value: &str) -> Result<Self, String> {
        let trimmed = value.trim();

        // sanity check: at least one '@' and '.' after '@'
        if let Some(at_index) = trimmed.find('@') {
            let domain_part = &trimmed[at_index + 1..];
            if !domain_part.contains('.') {
                return Err("Email domain must contain '.'".to_string());
            }
        } else {
            return Err("Email must contain '@'".to_string());
        }

        if trimmed.len() < 5 {
            return Err("Email too short".to_string());
        }

        Ok(Self(trimmed.to_lowercase()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
