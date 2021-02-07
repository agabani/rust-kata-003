use std::convert::TryFrom;

#[derive(Debug, PartialEq)]
pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. Use either `local` or `production`.",
                other
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Environment;
    use std::convert::TryFrom;

    #[test]
    fn as_str() {
        assert_eq!("local", Environment::Local.as_str());
        assert_eq!("production", Environment::Production.as_str());
    }

    #[test]
    fn try_from() {
        assert_eq!(
            Ok(Environment::Local),
            Environment::try_from("local".to_owned())
        );
        assert_eq!(
            Ok(Environment::Production),
            Environment::try_from("production".to_owned())
        );
        assert_eq!(
            Err(
                "other is not a supported environment. Use either `local` or `production`."
                    .to_owned()
            ),
            Environment::try_from("other".to_owned())
        );
    }
}
