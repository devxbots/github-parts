use anyhow::Context;
use reqwest::Method;

use crate::account::Login;
use crate::check_run::CheckRun;
use crate::check_suite::CheckSuiteId;
use crate::github::client::GitHubClient;
use crate::github::{AppId, GitHubHost, PrivateKey};
use crate::installation::InstallationId;
use crate::repository::RepositoryName;

pub async fn list_check_runs(
    github_host: &GitHubHost,
    app_id: AppId,
    private_key: &PrivateKey,
    installation_id: InstallationId,
    owner: &Login,
    repository: &RepositoryName,
    check_suite_id: CheckSuiteId,
) -> Result<Vec<CheckRun>, ListCheckRunsError> {
    let client: GitHubClient<CheckRun> =
        GitHubClient::new(github_host, app_id, private_key, installation_id);

    let url = format!(
        "{}/repos/{}/{}/check-suites/{}/check-runs",
        github_host.get(),
        owner.get(),
        repository.get(),
        check_suite_id
    );

    let check_runs = client
        .paginate(Method::GET, &url, "check_runs")
        .await
        .context("failed to query check runs")?;

    Ok(check_runs)
}

#[derive(Debug, thiserror::Error)]
pub enum ListCheckRunsError {
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

#[cfg(test)]
mod tests {
    use mockito::mock;

    use crate::account::Login;
    use crate::check_suite::CheckSuiteId;
    use crate::github::{AppId, GitHubHost, PrivateKey};
    use crate::installation::InstallationId;
    use crate::repository::RepositoryName;

    use super::list_check_runs;

    #[tokio::test]
    async fn list_check_runs_returns_all_check_runs() {
        let _token_mock = mock("POST", "/app/installations/1/access_tokens")
            .with_status(200)
            .with_body(r#"{ "token": "ghs_16C7e42F292c6912E7710c838347Ae178B4a" }"#)
            .create();
        let _content_mock = mock("GET", "/repos/github/hello-world/check-suites/5/check-runs")
            .with_status(200)
            .with_body(r#"
                {
                  "total_count": 1,
                  "check_runs": [
                    {
                      "id": 4,
                      "head_sha": "ce587453ced02b1526dfb4cb910479d431683101",
                      "node_id": "MDg6Q2hlY2tSdW40",
                      "external_id": "",
                      "url": "https://api.github.com/repos/github/hello-world/check-runs/4",
                      "html_url": "https://github.com/github/hello-world/runs/4",
                      "details_url": "https://example.com",
                      "status": "completed",
                      "conclusion": "neutral",
                      "started_at": "2018-05-04T01:14:52Z",
                      "completed_at": "2018-05-04T01:14:52Z",
                      "output": {
                        "title": "Mighty Readme report",
                        "summary": "There are 0 failures, 2 warnings, and 1 notice.",
                        "text": "You may have some misspelled words on lines 2 and 4. You also may want to add a section in your README about how to install your app.",
                        "annotations_count": 2,
                        "annotations_url": "https://api.github.com/repos/github/hello-world/check-runs/4/annotations"
                      },
                      "name": "mighty_readme",
                      "check_suite": {
                        "id": 5
                      },
                      "app": {
                        "id": 1,
                        "slug": "octoapp",
                        "node_id": "MDExOkludGVncmF0aW9uMQ==",
                        "owner": {
                          "login": "github",
                          "id": 1,
                          "node_id": "MDEyOk9yZ2FuaXphdGlvbjE=",
                          "url": "https://api.github.com/orgs/github",
                          "repos_url": "https://api.github.com/orgs/github/repos",
                          "events_url": "https://api.github.com/orgs/github/events",
                          "avatar_url": "https://github.com/images/error/octocat_happy.gif",
                          "gravatar_id": "",
                          "html_url": "https://github.com/octocat",
                          "followers_url": "https://api.github.com/users/octocat/followers",
                          "following_url": "https://api.github.com/users/octocat/following{/other_user}",
                          "gists_url": "https://api.github.com/users/octocat/gists{/gist_id}",
                          "starred_url": "https://api.github.com/users/octocat/starred{/owner}{/repo}",
                          "subscriptions_url": "https://api.github.com/users/octocat/subscriptions",
                          "organizations_url": "https://api.github.com/users/octocat/orgs",
                          "received_events_url": "https://api.github.com/users/octocat/received_events",
                          "type": "User",
                          "site_admin": true
                        },
                        "name": "Octocat App",
                        "description": "",
                        "external_url": "https://example.com",
                        "html_url": "https://github.com/apps/octoapp",
                        "created_at": "2017-07-08T16:18:44-04:00",
                        "updated_at": "2017-07-08T16:18:44-04:00",
                        "permissions": {
                          "metadata": "read",
                          "contents": "read",
                          "issues": "write",
                          "single_file": "write"
                        },
                        "events": [
                          "push",
                          "pull_request"
                        ]
                      },
                      "pull_requests": [
                        {
                          "url": "https://api.github.com/repos/github/hello-world/pulls/1",
                          "id": 1934,
                          "number": 3956,
                          "head": {
                            "ref": "say-hello",
                            "sha": "3dca65fa3e8d4b3da3f3d056c59aee1c50f41390",
                            "repo": {
                              "id": 526,
                              "url": "https://api.github.com/repos/github/hello-world",
                              "name": "hello-world"
                            }
                          },
                          "base": {
                            "ref": "master",
                            "sha": "e7fdf7640066d71ad16a86fbcbb9c6a10a18af4f",
                            "repo": {
                              "id": 526,
                              "url": "https://api.github.com/repos/github/hello-world",
                              "name": "hello-world"
                            }
                          }
                        }
                      ]
                    }
                  ]
                }
            "#).create();

        let check_runs = list_check_runs(
            &GitHubHost::new(mockito::server_url()),
            AppId::new(1),
            &PrivateKey::new(include_str!("../../../tests/fixtures/private-key.pem").into()),
            InstallationId::new(1),
            &Login::new("github"),
            &RepositoryName::new("hello-world"),
            CheckSuiteId::new(5),
        )
        .await
        .unwrap();

        assert_eq!(1, check_runs.len());
    }
}
