use chrono::NaiveDate;
use regex::Regex;
use validator::ValidateEmail;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct User {
    pub name: String,
    pub surname: String,
    pub birthdate: String,
    pub mail: String,
    pub status: String,
    pub phone: String,
}

pub fn check_name(name: &str) -> bool {
    let re = Regex::new(r"^[a-zA-Z]+$").unwrap();
    re.is_match(name)
}

pub fn check_birthdate(date: &str) -> bool {
    NaiveDate::parse_from_str(date, "%Y-%m-%d").is_ok()
}

pub fn check_email(email: &str) -> bool {
    email.validate_email()
}

pub fn check_phone(phone: &str) -> bool {
    let re = Regex::new(r"^\+?\d{11}$").unwrap();
    re.is_match(phone)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        assert!(check_name("Anton"));
        assert!(check_name("aBoBus"));
        assert!(!check_name("9bobs"));
        assert!(!check_name("bob_s_s"));
        assert!(!check_name(""));
    }
    #[test]
    fn test_birthdate() {
        assert!(check_birthdate("2025-03-08"));
        assert!(check_birthdate("1990-12-31"));
        assert!(!check_birthdate("2025-02-29"));
        assert!(!check_birthdate("2025-03-00"));
        assert!(!check_birthdate("aBobus"));
    }
    #[test]
    fn test_email() {
        assert!(check_email("ailutsenko@edu.hse.ru"));
        assert!(!check_email("boba(("));
    }
    #[test]
    fn test_phone() {
        assert!(check_phone("88005553535"));
        assert!(check_phone("+78005553535"));
        assert!(!check_phone("boba(("));
    }
}
