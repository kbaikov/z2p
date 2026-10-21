#[derive(Debug)]
pub struct SubscriberEmail(String);
use validator::ValidateEmail;

impl SubscriberEmail {
    pub fn parse(s: String) -> Result<Self, String> {
        if ValidateEmail::validate_email(&s) {
            Ok(Self(s))
        } else {
            Err("invalid email".to_string())
        }
    }
}

impl AsRef<str> for SubscriberEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for SubscriberEmail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // We just forward to the Display implementation of
        // the wrapped String.
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fake::Fake;
    use fake::faker::internet::en::SafeEmail;

    #[derive(Debug, Clone)]
    struct ValidEmailFixture(pub String);

    impl quickcheck::Arbitrary for ValidEmailFixture {
        fn arbitrary(_g: &mut quickcheck::Gen) -> Self {
            let email = SafeEmail().fake();
            Self(email)
        }
    }

    #[test]
    fn empty_string_is_rejected() {
        let email = "".to_string();
        assert!(SubscriberEmail::parse(email).is_err());
    }

    #[test]
    fn email_missing_at_symbol_is_rejected() {
        let email = "ursuladomain.com".to_string();
        assert!(SubscriberEmail::parse(email).is_err());
    }

    #[test]
    fn email_missing_subject_is_rejected() {
        let email = "@domain.com".to_string();
        assert!(SubscriberEmail::parse(email).is_err());
    }

    #[quickcheck_macros::quickcheck]
    fn qc_valid_emails_are_parsed_successfully(valid_email: ValidEmailFixture) {
        assert!(SubscriberEmail::parse(valid_email.0).is_ok());
    }
}
