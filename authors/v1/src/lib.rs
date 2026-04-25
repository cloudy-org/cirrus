pub mod error;
pub mod contributor;

mod authors;
pub use authors::*;

#[cfg(test)]
mod tests {
    use cirrus_git_tag::{GitTag, platform::Platform};

    use crate::{contributor::Contributor, error::Result};

    #[test]
    fn test_contributors() -> Result<()> {
        let contributor = Contributor::parse_string(
            &String::from("Goldy <goldy@devgoldy.xyz> (THEGOLDENPRO @ gh)")
        )?;

        assert_eq!(
            contributor,
            Contributor {
                name: String::from("Goldy"),
                email: Some(String::from("goldy@devgoldy.xyz")),
                git_tag: Some(
                    GitTag {
                        owner: String::from("THEGOLDENPRO"),
                        repo: None,
                        platform: Platform::GitHub,
                    }
                )
            }
        );

        assert_eq!(
            Contributor::parse_string(&String::from("John Doe"))?,
            Contributor {
                name: String::from("John Doe"),
                email: None,
                git_tag: None,
            }
        );

        assert_eq!(
            Contributor::parse_string(&String::from("Goldy V2 (goldyv2 @ gh) "))?,
            Contributor {
                name: String::from("Goldy V2"),
                email: None,
                git_tag: Some(
                    GitTag {
                        owner: String::from("goldyv2"),
                        repo: None,
                        platform: Platform::GitHub
                    }
                ),
            }
        );

        Ok(())
    }
}
