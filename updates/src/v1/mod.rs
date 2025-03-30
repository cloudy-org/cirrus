use reqwest::{blocking, header::USER_AGENT, blocking::Response};
use semver::{Version, VersionReq};

// NOTE: Breaking changes may come to this in the future.

type ActualError = String;

pub enum Error {
    RequestFailed(ActualError),
    ResponseError(Response),
    FailedDataDeserialization(ActualError),
}

#[derive(serde::Deserialize)]
struct GitHubData {
    tag_name: String,
}

pub fn get_latest_version(username: String, repository: String, local_version: String) -> Result<Option<Version>, Error> {
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
                    let mut tag = newest_release.tag_name.clone();

                    if tag.starts_with("v") {
                        tag.remove(0);
                    }

                    let req_string = format!(">{}", local_version).to_string();
                    let req = VersionReq::parse(&req_string).unwrap();
                    let remote = Version::parse(&tag).unwrap();

                    match req.matches(&remote) {
                        true => Ok(Some(remote)),
                        false => Ok(None)
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