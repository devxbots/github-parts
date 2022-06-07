//! Get a file from a repository on GitHub

use std::path::Path;

use reqwest::Client;

use crate::account::Login;
use crate::github::token::{AppToken, InstallationToken};
use crate::github::{AppId, GitHubHost, PrivateKey};
use crate::installation::InstallationId;
use crate::repository::RepositoryName;

pub use self::error::GetFileError;
use self::payload::GetFilePayload;
pub use self::result::GetFileResult;

mod error;
mod payload;
mod result;

/// Get a file
///
/// This action downloads a file from GitHub.
pub async fn get_file(
    github_host: &GitHubHost,
    app_id: &AppId,
    private_key: &PrivateKey,
    installation: &InstallationId,
    owner: &Login,
    repository: &RepositoryName,
    path: &Path,
) -> Result<GetFileResult, GetFileError> {
    let url = format!(
        "{}/repos/{}/{}/contents/{}",
        github_host.get(),
        owner.get(),
        repository.get(),
        path.to_str()
            .ok_or_else(|| GetFileError::Argument("failed to convert path to string".into()))?
    );

    let app_token = AppToken::new(app_id, private_key)?;
    let installation_token = InstallationToken::new(github_host, &app_token, installation).await?;

    let body = Client::new()
        .get(url)
        .header(
            "Authorization",
            format!("Bearer {}", installation_token.get()),
        )
        .header("Accept", "application/vnd.github.v3+json")
        .header("User-Agent", "devxbots/github-parts")
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    if body.is_array() {
        Err(GetFileError::Directory)
    } else {
        let payload: GetFilePayload = serde_json::from_value(body)?;
        GetFileResult::try_from(payload)
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use mockito::mock;

    use crate::account::Login;
    use crate::action::get_file::{get_file, GetFileError};
    use crate::github::{AppId, GitHubHost, PrivateKey};
    use crate::installation::InstallationId;
    use crate::repository::RepositoryName;

    #[tokio::test]
    async fn get_file_with_file() {
        let _token_mock = mock("POST", "/app/installations/1/access_tokens")
            .with_status(200)
            .with_body(r#"{ "token": "ghs_16C7e42F292c6912E7710c838347Ae178B4a" }"#)
            .create();
        let _content_mock = mock("GET", "/repos/octokit/octokit.rb/contents/README.md")
            .with_status(200)
            .with_body(r#"
                {
                  "type": "file",
                  "encoding": "base64",
                  "size": 5362,
                  "name": "README.md",
                  "path": "README.md",
                  "content": "ZW5jb2RlZCBjb250ZW50IC4uLg==",
                  "sha": "3d21ec53a331a6f037a91c368710b99387d012c1",
                  "url": "https://api.github.com/repos/octokit/octokit.rb/contents/README.md",
                  "git_url": "https://api.github.com/repos/octokit/octokit.rb/git/blobs/3d21ec53a331a6f037a91c368710b99387d012c1",
                  "html_url": "https://github.com/octokit/octokit.rb/blob/master/README.md",
                  "download_url": "https://raw.githubusercontent.com/octokit/octokit.rb/master/README.md",
                  "_links": {
                    "git": "https://api.github.com/repos/octokit/octokit.rb/git/blobs/3d21ec53a331a6f037a91c368710b99387d012c1",
                    "self": "https://api.github.com/repos/octokit/octokit.rb/contents/README.md",
                    "html": "https://github.com/octokit/octokit.rb/blob/master/README.md"
                  }
                }
            "#).create();

        let file = get_file(
            &GitHubHost::new(mockito::server_url()),
            &AppId::new(1),
            &PrivateKey::new(include_str!("../../../tests/fixtures/private-key.pem").into()),
            &InstallationId::new(1),
            &Login::new("octokit"),
            &RepositoryName::new("octokit.rb"),
            PathBuf::from("README.md").as_path(),
        )
        .await
        .unwrap();

        assert_eq!("README.md", file.name());
        assert_eq!(5362, file.size());
    }

    #[tokio::test]
    async fn get_file_with_directory() {
        let _token_mock = mock("POST", "/app/installations/1/access_tokens")
            .with_status(200)
            .with_body(r#"{ "token": "ghs_16C7e42F292c6912E7710c838347Ae178B4a" }"#)
            .create();
        let _content_mock = mock("GET", "/repos/octokit/octokit.rb/contents/lib/octokit")
            .with_status(200)
            .with_body(r#"
                [
                  {
                    "type": "file",
                    "size": 625,
                    "name": "octokit.rb",
                    "path": "lib/octokit.rb",
                    "sha": "fff6fe3a23bf1c8ea0692b4a883af99bee26fd3b",
                    "url": "https://api.github.com/repos/octokit/octokit.rb/contents/lib/octokit.rb",
                    "git_url": "https://api.github.com/repos/octokit/octokit.rb/git/blobs/fff6fe3a23bf1c8ea0692b4a883af99bee26fd3b",
                    "html_url": "https://github.com/octokit/octokit.rb/blob/master/lib/octokit.rb",
                    "download_url": "https://raw.githubusercontent.com/octokit/octokit.rb/master/lib/octokit.rb",
                    "_links": {
                      "self": "https://api.github.com/repos/octokit/octokit.rb/contents/lib/octokit.rb",
                      "git": "https://api.github.com/repos/octokit/octokit.rb/git/blobs/fff6fe3a23bf1c8ea0692b4a883af99bee26fd3b",
                      "html": "https://github.com/octokit/octokit.rb/blob/master/lib/octokit.rb"
                    }
                  },
                  {
                    "type": "dir",
                    "size": 0,
                    "name": "octokit",
                    "path": "lib/octokit",
                    "sha": "a84d88e7554fc1fa21bcbc4efae3c782a70d2b9d",
                    "url": "https://api.github.com/repos/octokit/octokit.rb/contents/lib/octokit",
                    "git_url": "https://api.github.com/repos/octokit/octokit.rb/git/trees/a84d88e7554fc1fa21bcbc4efae3c782a70d2b9d",
                    "html_url": "https://github.com/octokit/octokit.rb/tree/master/lib/octokit",
                    "download_url": null,
                    "_links": {
                      "self": "https://api.github.com/repos/octokit/octokit.rb/contents/lib/octokit",
                      "git": "https://api.github.com/repos/octokit/octokit.rb/git/trees/a84d88e7554fc1fa21bcbc4efae3c782a70d2b9d",
                      "html": "https://github.com/octokit/octokit.rb/tree/master/lib/octokit"
                    }
                  }
                ]
            "#).create();

        let error = get_file(
            &GitHubHost::new(mockito::server_url()),
            &AppId::new(1),
            &PrivateKey::new(include_str!("../../../tests/fixtures/private-key.pem").into()),
            &InstallationId::new(1),
            &Login::new("octokit"),
            &RepositoryName::new("octokit.rb"),
            PathBuf::from("lib/octokit").as_path(),
        )
        .await
        .unwrap_err();

        assert!(matches!(error, GetFileError::Directory));
    }

    #[tokio::test]
    async fn get_file_with_symlink() {
        let _token_mock = mock("POST", "/app/installations/1/access_tokens")
            .with_status(200)
            .with_body(r#"{ "token": "ghs_16C7e42F292c6912E7710c838347Ae178B4a" }"#)
            .create();
        let _content_mock = mock("GET", "/repos/octokit/octokit.rb/contents/bin/some-symlink")
            .with_status(200)
            .with_body(r#"
                {
                  "type": "symlink",
                  "target": "/path/to/symlink/target",
                  "size": 23,
                  "name": "some-symlink",
                  "path": "bin/some-symlink",
                  "sha": "452a98979c88e093d682cab404a3ec82babebb48",
                  "url": "https://api.github.com/repos/octokit/octokit.rb/contents/bin/some-symlink",
                  "git_url": "https://api.github.com/repos/octokit/octokit.rb/git/blobs/452a98979c88e093d682cab404a3ec82babebb48",
                  "html_url": "https://github.com/octokit/octokit.rb/blob/master/bin/some-symlink",
                  "download_url": "https://raw.githubusercontent.com/octokit/octokit.rb/master/bin/some-symlink",
                  "_links": {
                    "git": "https://api.github.com/repos/octokit/octokit.rb/git/blobs/452a98979c88e093d682cab404a3ec82babebb48",
                    "self": "https://api.github.com/repos/octokit/octokit.rb/contents/bin/some-symlink",
                    "html": "https://github.com/octokit/octokit.rb/blob/master/bin/some-symlink"
                  }
                }
            "#).create();

        let error = get_file(
            &GitHubHost::new(mockito::server_url()),
            &AppId::new(1),
            &PrivateKey::new(include_str!("../../../tests/fixtures/private-key.pem").into()),
            &InstallationId::new(1),
            &Login::new("octokit"),
            &RepositoryName::new("octokit.rb"),
            PathBuf::from("bin/some-symlink").as_path(),
        )
        .await
        .unwrap_err();

        assert!(matches!(error, GetFileError::Symlink));
    }

    #[tokio::test]
    async fn get_file_with_submodule() {
        let _token_mock = mock("POST", "/app/installations/1/access_tokens")
            .with_status(200)
            .with_body(r#"{ "token": "ghs_16C7e42F292c6912E7710c838347Ae178B4a" }"#)
            .create();
        let _content_mock = mock("GET", "/repos/jquery/jquery/contents/test/qunit")
            .with_status(200)
            .with_body(r#"
                {
                  "type": "submodule",
                  "submodule_git_url": "git://github.com/jquery/qunit.git",
                  "size": 0,
                  "name": "qunit",
                  "path": "test/qunit",
                  "sha": "6ca3721222109997540bd6d9ccd396902e0ad2f9",
                  "url": "https://api.github.com/repos/jquery/jquery/contents/test/qunit?ref=master",
                  "git_url": "https://api.github.com/repos/jquery/qunit/git/trees/6ca3721222109997540bd6d9ccd396902e0ad2f9",
                  "html_url": "https://github.com/jquery/qunit/tree/6ca3721222109997540bd6d9ccd396902e0ad2f9",
                  "download_url": null,
                  "_links": {
                    "git": "https://api.github.com/repos/jquery/qunit/git/trees/6ca3721222109997540bd6d9ccd396902e0ad2f9",
                    "self": "https://api.github.com/repos/jquery/jquery/contents/test/qunit?ref=master",
                    "html": "https://github.com/jquery/qunit/tree/6ca3721222109997540bd6d9ccd396902e0ad2f9"
                  }
                }
            "#).create();

        let error = get_file(
            &GitHubHost::new(mockito::server_url()),
            &AppId::new(1),
            &PrivateKey::new(include_str!("../../../tests/fixtures/private-key.pem").into()),
            &InstallationId::new(1),
            &Login::new("jquery"),
            &RepositoryName::new("jquery"),
            PathBuf::from("test/qunit").as_path(),
        )
        .await
        .unwrap_err();

        assert!(matches!(error, GetFileError::Submodule));
    }
}