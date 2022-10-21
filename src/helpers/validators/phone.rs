use regex::Regex;

pub fn validate_phone_number(phone: &str) -> bool {
    lazy_static! {
        static ref PHONE_RE: Regex = Regex::new(r"^\+[0-9]{10,15}$").unwrap();
    }
    PHONE_RE.is_match(phone)
}