use egui_notify::ToastLevel;
use reqwest::{blocking, header::USER_AGENT};
use semver::{Version, VersionReq};

#[derive(serde::Deserialize)]
struct GitHubData {
    tag_name: String,
}
pub struct UpdateCheck {
    username: String,
    repository: String,
    local_version: String
}

impl UpdateCheck {
    pub fn new(username: &str, repository: &str, local_version: &str) -> Self {
        Self {
            username: username.to_string(),
            repository: repository.to_string(),
            local_version: local_version.to_string()
        }
    }

    pub fn check(&self) -> Option<(String, ToastLevel)> {
        let github_api = format!("https://api.github.com/repos/{}/{}/releases", self.username, self.repository);

        let client = blocking::Client::new();
        match client.get(github_api).header(USER_AGENT, "Reqwest").send() {
            Ok(resp) => {
                if !resp.status().is_success() {
                    return Some(
                        (format!("GitHub API responded with: {:?}", resp.status()), ToastLevel::Error)
                    )
                }

                let data: Vec<GitHubData> = match resp.json() {
                    Ok(data) => data,
                    Err(err) => {
                        return Some(
                            (format!("Failed to deserialize data: {}", err), ToastLevel::Error)
                        );
                    }
                };

                match data.first() {
                    Some(newest_release) => {
                        let mut tag = newest_release.tag_name.clone();
                        if tag.starts_with("v") {
                            tag.remove(0);
                        }

                        let req_string = format!(">{}", self.local_version).to_string();
                        let req = VersionReq::parse(&req_string).unwrap();
                        let remote = Version::parse(&tag).unwrap();

                        if req.matches(&remote) {
                            return Some((format!("New version available: {}", &newest_release.tag_name), ToastLevel::Info))
                        } else {
                            return None
                        }
                    }
                    None => return None
                };
            },
            Err(err) => {
                return Some(
                    (format!("Failed to send request to github api: {}", err.to_string()), ToastLevel::Error)
                )
            }
        };
    }
}