#[derive(Clone, Debug, PartialEq)]
pub struct CrateVersion(String);

impl CrateVersion {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn parse(value: &str) -> Result<Self, String> {
        Ok(Self(value.to_owned()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fake::{Fake, Faker};

    #[test]
    fn as_str() {
        let value = Faker.fake::<String>();
        let result = CrateVersion::parse(&value).unwrap();
        assert_eq!(value, result.as_str());
    }

    #[test]
    fn parse() {
        let result = CrateVersion::parse(&Faker.fake::<String>());
        assert!(result.is_ok());
    }
}
