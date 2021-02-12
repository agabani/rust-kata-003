use std::convert::TryFrom;

#[derive(Debug, PartialEq)]
pub enum CrateDependencyType {
    Build,
    Dev,
    Normal,
}

impl CrateDependencyType {
    pub fn as_str(&self) -> &str {
        match &self {
            CrateDependencyType::Build => "build",
            CrateDependencyType::Dev => "dev",
            CrateDependencyType::Normal => "normal",
        }
    }
}

impl TryFrom<&str> for CrateDependencyType {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "build" => Ok(Self::Build),
            "dev" => Ok(Self::Dev),
            "normal" => Ok(Self::Normal),
            other => Err(format!(
                "{} is not a supported relationship. Use either `build`, `dev` or `normal`.",
                other
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fake::{Fake, Faker};

    #[test]
    fn as_str() {
        assert_eq!("build", CrateDependencyType::Build.as_str());
        assert_eq!("dev", CrateDependencyType::Dev.as_str());
        assert_eq!("normal", CrateDependencyType::Normal.as_str());
    }

    #[test]
    fn try_from() {
        assert_eq!(
            Ok(CrateDependencyType::Build),
            CrateDependencyType::try_from("build")
        );
        assert_eq!(
            Ok(CrateDependencyType::Dev),
            CrateDependencyType::try_from("dev")
        );
        assert_eq!(
            Ok(CrateDependencyType::Normal),
            CrateDependencyType::try_from("normal")
        );

        let other = Faker.fake::<String>();
        assert_eq!(
            Err(format!(
                "{} is not a supported relationship. Use either `build`, `dev` or `normal`.",
                other
            )),
            CrateDependencyType::try_from(other.as_str())
        );
    }
}
