use reqwest::{blocking, header::USER_AGENT};
use semver::{Version, VersionReq};

#[derive(serde::Deserialize)]
struct GitHubData {
    tag_name: String,
}

pub fn get_latest_version(username: String, repository: String, local_version: String) -> Result<Option<Version>, String> {
    let github_api = format!("https://api.github.com/repos/{}/{}/releases", username, repository);

    let client = blocking::Client::new();

    match client.get(github_api).header(USER_AGENT, "Reqwest").send() {
        Ok(resp) => {
            if !resp.status().is_success() {
                return Err(
                    format!("GitHub API responded with: {:?}", resp.status())
                )
            }

            let data: Vec<GitHubData> = match resp.json() {
                Ok(data) => data,
                Err(err) => {
                    return Err(
                        format!("Failed to deserialize data: {}", err)
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

                    if req.matches(&remote) {
                        return Ok(Some(remote))
                    } else {
                        return Ok(None)
                    }
                }
                None => return Err("No release found".to_string())
            };
        },
        Err(err) => {
            return Err(
                format!("Failed to send request to github api: {}", err.to_string())
            )
        }
    };
}
