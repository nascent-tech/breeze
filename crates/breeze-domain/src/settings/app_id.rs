use crate::settings::invalid_app_id::InvalidAppId;

const MAX_LEN: usize = 255;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct AppId(String);

impl AppId {
    pub fn parse(raw: &str) -> Result<AppId, InvalidAppId> {
        if raw.is_empty() {
            return Err(InvalidAppId::Empty);
        }
        if raw.len() > MAX_LEN {
            return Err(InvalidAppId::TooLong);
        }
        if !raw.bytes().all(is_allowed) {
            return Err(InvalidAppId::ForbiddenCharacter);
        }
        Ok(AppId(raw.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

fn is_allowed(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_bundle_identifier_round_trips() {
        let id = AppId::parse("com.apple.Safari").unwrap();
        assert_eq!(id.as_str(), "com.apple.Safari");
    }

    #[test]
    fn an_empty_identifier_is_refused() {
        assert_eq!(AppId::parse(""), Err(InvalidAppId::Empty));
    }

    #[test]
    fn an_identifier_beyond_the_length_cap_is_refused() {
        let raw = "a".repeat(MAX_LEN + 1);
        assert_eq!(AppId::parse(&raw), Err(InvalidAppId::TooLong));
    }

    #[test]
    fn a_hostile_identifier_with_a_path_separator_is_refused() {
        assert_eq!(
            AppId::parse("../../etc/passwd"),
            Err(InvalidAppId::ForbiddenCharacter)
        );
    }

    #[test]
    fn an_identifier_that_looks_like_a_command_flag_is_refused() {
        assert_eq!(
            AppId::parse("--output foo"),
            Err(InvalidAppId::ForbiddenCharacter)
        );
    }
}
