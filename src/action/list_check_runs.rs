use anyhow::Context;
use async_trait::async_trait;
use derive_new::new;
use reqwest::Method;

use crate::account::Login;
use crate::action::Action;
use crate::check_run::CheckRun;
use crate::check_suite::CheckSuiteId;
use crate::github::client::GitHubClient;
use crate::repository::RepositoryName;

#[derive(Debug, new)]
pub struct ListCheckRuns<'a> {
    github_client: &'a GitHubClient,
    owner: &'a Login,
    repository: &'a RepositoryName,
}

#[async_trait]
impl<'a> Action<CheckSuiteId, Vec<CheckRun>, ListCheckRunsError> for ListCheckRuns<'a> {
    #[tracing::instrument]
    async fn execute(
        &self,
        check_suite_id: CheckSuiteId,
    ) -> Result<Vec<CheckRun>, ListCheckRunsError> {
        let url = format!(
            "/repos/{}/{}/check-suites/{}/check-runs",
            self.owner.get(),
            self.repository.get(),
            check_suite_id
        );

        let check_runs = self
            .github_client
            .paginate(Method::GET, &url, "check_runs")
            .await
            .context("failed to query check runs")?;

        Ok(check_runs)
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
    use crate::check_suite::CheckSuiteId;
    use crate::repository::RepositoryName;
    use crate::testing::check_run::mock_list_check_runs;
    use crate::testing::client::github_client;
    use crate::testing::token::mock_installation_access_tokens;

    use super::ListCheckRuns;

    #[tokio::test]
    async fn list_check_runs_returns_all_check_runs() {
        let _token_mock = mock_installation_access_tokens();
        let _content_mock = mock_list_check_runs();

        let github_client = github_client();
        let owner = Login::new("github");
        let repository = RepositoryName::new("hello-world");

        let check_runs = ListCheckRuns::new(&github_client, &owner, &repository)
            .execute(CheckSuiteId::new(5))
            .await
            .unwrap();

        assert_eq!(1, check_runs.len());
    }
}
