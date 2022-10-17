use regex::Regex;

pub fn validate_email(email: &str) -> bool {
    lazy_static! {
        static ref EMAIL_RE: Regex = Regex::new(r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$").unwrap();
    }
    EMAIL_RE.is_match(email)
}

