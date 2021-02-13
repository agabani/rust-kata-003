use crate::domain::CrateVersion;

#[derive(Debug, PartialEq)]
pub struct CrateRequirement(String);

impl CrateRequirement {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn parse(value: &str) -> Result<Self, String> {
        Ok(Self(value.to_owned()))
    }

    pub fn minimum_version(&self) -> CrateVersion {
        let range_set =
            semver_parser::RangeSet::parse(&self.0, semver_parser::Compat::Cargo).unwrap();

        let comparator = &range_set.ranges[0].comparator_set[0];

        CrateVersion::parse(&format!(
            "{}.{}.{}",
            comparator.major, comparator.minor, comparator.patch
        ))
        .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fake::{Fake, Faker};

    #[test]
    fn as_str() {
        let value = Faker.fake::<String>();
        let result = CrateRequirement::parse(&value).unwrap();
        assert_eq!(value, result.as_str());
    }

    #[test]
    fn parse() {
        let result = CrateRequirement::parse(&Faker.fake::<String>());
        assert!(result.is_ok());
    }

    #[test]
    fn minimum_version() {
        let requirement = CrateRequirement::parse("^2.2.2").unwrap();
        let version = requirement.minimum_version();
        assert_eq!("2.2.2", version.as_str())
    }
}
