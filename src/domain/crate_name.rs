pub struct CrateName(String);

impl CrateName {
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
        let result = CrateName::parse(&value).unwrap();
        assert_eq!(value, result.as_str());
    }

    #[test]
    fn parse() {
        let result = CrateName::parse(&Faker.fake::<String>());
        assert!(result.is_ok());
    }
}
