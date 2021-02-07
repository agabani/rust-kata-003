use std::convert::TryFrom;

#[derive(Debug, PartialEq)]
pub enum CrateRelationship {
    Normal,
    Dev,
}

impl CrateRelationship {
    pub fn as_str(&self) -> &str {
        match &self {
            CrateRelationship::Normal => "normal",
            CrateRelationship::Dev => "dev",
        }
    }
}

impl TryFrom<&str> for CrateRelationship {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "normal" => Ok(Self::Normal),
            "dev" => Ok(Self::Dev),
            other => Err(format!(
                "{} is not a supported relationship. Use either `normal` or `dev`.",
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
        assert_eq!("normal", CrateRelationship::Normal.as_str());
        assert_eq!("dev", CrateRelationship::Dev.as_str());
    }

    #[test]
    fn try_from() {
        assert_eq!(
            Ok(CrateRelationship::Normal),
            CrateRelationship::try_from("normal")
        );
        assert_eq!(
            Ok(CrateRelationship::Dev),
            CrateRelationship::try_from("dev")
        );

        let other = Faker.fake::<String>();
        assert_eq!(
            Err(format!(
                "{} is not a supported relationship. Use either `normal` or `dev`.",
                other
            )),
            CrateRelationship::try_from(other.as_str())
        );
    }
}
