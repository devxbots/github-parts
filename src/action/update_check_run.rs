use anyhow::Context;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::account::Login;
use crate::action::Action;
use crate::check_run::{CheckRun, CheckRunConclusion, CheckRunId, CheckRunStatus};
use crate::github::client::GitHubClient;
use crate::repository::RepositoryName;

#[derive(Debug)]
pub struct UpdateCheckRun<'a> {
    github_client: &'a GitHubClient,
    owner: &'a Login,
    repository: &'a RepositoryName,
    check_run_id: CheckRunId,
}

impl<'a> UpdateCheckRun<'a> {
    #[tracing::instrument]
    pub fn new(
        github_client: &'a GitHubClient,
        owner: &'a Login,
        repository: &'a RepositoryName,
        check_run_id: CheckRunId,
    ) -> Self {
        Self {
            github_client,
            owner,
            repository,
            check_run_id,
        }
    }
}

#[async_trait]
impl<'a> Action<UpdateCheckRunInput, CheckRun, UpdateCheckRunError> for UpdateCheckRun<'a> {
    #[tracing::instrument]
    async fn execute(
        &mut self,
        input: &UpdateCheckRunInput,
    ) -> Result<CheckRun, UpdateCheckRunError> {
        let url = format!(
            "/repos/{}/{}/check-runs/{}",
            self.owner.get(),
            self.repository.get(),
            self.check_run_id
        );

        let check_run = self
            .github_client
            .patch(&url, Some(input))
            .await
            .context("failed to update check run")?;

        Ok(check_run)
    }
}

// TODO: Pass by reference, not by value (e.g. &HeadSha)
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize)]
pub struct UpdateCheckRunInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<CheckRunStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conclusion: Option<CheckRunConclusion>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, thiserror::Error)]
pub enum UpdateCheckRunError {
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

#[cfg(test)]
mod tests {
    use crate::account::Login;
    use crate::action::Action;
    use crate::check_run::{CheckRunConclusion, CheckRunId, CheckRunStatus};
    use crate::repository::RepositoryName;
    use crate::testing::check_run::mock_update_check_run;
    use crate::testing::client::github_client;
    use crate::testing::token::mock_installation_access_tokens;

    use super::{UpdateCheckRun, UpdateCheckRunInput};

    #[tokio::test]
    async fn update_check_run_returns_check_run() {
        let _token_mock = mock_installation_access_tokens();
        let _content_mock = mock_update_check_run();

        let github_client = github_client();
        let owner = Login::new("github");
        let repository = RepositoryName::new("hello-world");
        let check_run_id = CheckRunId::new(4);

        let input = UpdateCheckRunInput {
            status: Some(CheckRunStatus::Completed),
            conclusion: Some(CheckRunConclusion::Neutral),
            completed_at: None,
        };

        let check_run = UpdateCheckRun::new(&github_client, &owner, &repository, check_run_id)
            .execute(&input)
            .await
            .unwrap();

        assert!(matches!(check_run.status(), CheckRunStatus::Completed));
    }
}
