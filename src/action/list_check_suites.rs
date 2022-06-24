use anyhow::Context;
use async_trait::async_trait;
use derive_new::new;
use reqwest::Method;

use crate::account::Login;
use crate::action::Action;
use crate::check_suite::CheckSuite;
use crate::git::HeadSha;
use crate::github::client::GitHubClient;
use crate::repository::RepositoryName;

#[derive(Debug, new)]
pub struct ListCheckSuites<'a> {
    github_client: &'a GitHubClient,
    owner: &'a Login,
    repository: &'a RepositoryName,
}

#[async_trait]
impl<'a> Action<HeadSha, Vec<CheckSuite>, ListCheckRunsError> for ListCheckSuites<'a> {
    #[tracing::instrument]
    async fn execute(&self, head_sha: &HeadSha) -> Result<Vec<CheckSuite>, ListCheckRunsError> {
        let url = format!(
            "/repos/{}/{}/commits/{}/check-suites",
            self.owner.get(),
            self.repository.get(),
            head_sha
        );

        let check_suites = self
            .github_client
            .paginate(Method::GET, &url, "check_suites")
            .await
            .context("failed to query check suites")?;

        Ok(check_suites)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ListCheckRunsError {
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

#[cfg(test)]
mod tests {
    use crate::account::Login;
    use crate::action::Action;
    use crate::git::HeadSha;
    use crate::repository::RepositoryName;
    use crate::testing::check_suite::mock_list_check_suites;
    use crate::testing::client::github_client;
    use crate::testing::token::mock_installation_access_tokens;

    use super::ListCheckSuites;

    #[tokio::test]
    async fn list_check_suites_returns_all_check_suite() {
        let _token_mock = mock_installation_access_tokens();
        let _content_mock = mock_list_check_suites();

        let github_client = github_client();
        let owner = Login::new("github");
        let repository = RepositoryName::new("hello-world");

        let check_runs = ListCheckSuites::new(&github_client, &owner, &repository)
            .execute(&HeadSha::new("d6fde92930d4715a2b49857d24b940956b26d2d3"))
            .await
            .unwrap();

        assert_eq!(1, check_runs.len());
    }
}
