use reqwest::{blocking, header::USER_AGENT, blocking::Response};
use semver::Version;

// NOTE: Breaking changes may come to this in the future as we shift to using other methods.

type ActualError = String;

#[derive(Debug)]
pub enum Error {
    RequestFailed(ActualError),
    ResponseError(Response),
    FailedDataDeserialization(ActualError),
    FailedToParseVersion(ActualError),
}

#[derive(serde::Deserialize)]
struct GitHubData {
    tag_name: String,
}

pub fn get_latest_version(username: &str, repository: &str) -> Result<Option<Version>, Error> {
    let github_api = format!("https://api.github.com/repos/{}/{}/releases", username, repository);

    let client = blocking::Client::new();

    match client.get(github_api).header(USER_AGENT, "cirrus").send() {
        Ok(resp) => {
            if !resp.status().is_success() {
                return Err(Error::ResponseError(resp));
            }

            let data: Vec<GitHubData> = match resp.json() {
                Ok(data) => data,
                Err(error) => {
                    return Err(
                        Error::FailedDataDeserialization(error.to_string())
                    );
                }
            };

            match data.first() {
                Some(newest_release) => {
                    let mut github_tag = newest_release.tag_name.clone();

                    if github_tag.starts_with("v") {
                        github_tag.remove(0);
                    }

                    match Version::parse(&github_tag) {
                        Ok(version) => Ok(Some(version)),
                        Err(error) => Err(
                            Error::FailedToParseVersion(error.to_string())
                        ),
                    }
                }
                None => Ok(None)
            }
        },
        Err(err) => {
            Err(Error::RequestFailed(err.to_string()))
        }
    }
}