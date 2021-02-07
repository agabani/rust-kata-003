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
    fn environment_as_str() {
        assert_eq!(Environment::Local.as_str(), "local");
        assert_eq!(Environment::Production.as_str(), "production");
    }

    #[test]
    fn environment_try_from() {
        assert_eq!(
            Environment::try_from("local".to_owned()),
            Ok(Environment::Local)
        );
        assert_eq!(
            Environment::try_from("production".to_owned()),
            Ok(Environment::Production)
        );
        assert_eq!(
            Environment::try_from("other".to_owned()),
            Err(
                "other is not a supported environment. Use either `local` or `production`."
                    .to_owned()
            )
        );
    }
}
