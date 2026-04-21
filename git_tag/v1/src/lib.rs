pub mod error;
pub mod platform;

mod git_tag;
pub use git_tag::*;

#[cfg(test)]
mod tests {
    use crate::{GitTag, error::Result, platform::Platform};

    #[test]
    fn test_platform() {
        assert_eq!(Platform::parse_platform_tag("GH"), Some(Platform::GitHub));
        assert_eq!(Platform::parse_platform_tag(" gh "), Some(Platform::GitHub));
        assert_eq!(Platform::parse_platform_tag("meow 🐈"), None);
    }

    #[test]
    fn test_git_tag() -> Result<()> {
        let git_tag = GitTag::parse_string(String::from("THEGOLDENPRO @ GH"))?;

        assert_eq!(git_tag.owner, String::from("THEGOLDENPRO"));
        assert_eq!(git_tag.repo, None);
        assert_eq!(git_tag.platform, Platform::GitHub);

        // TODO: test errors

        Ok(())
    }
}
