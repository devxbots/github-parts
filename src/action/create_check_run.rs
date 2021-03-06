use anyhow::Context;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use derive_new::new;
use serde::Serialize;

use crate::account::Login;
use crate::action::Action;
use crate::check_run::{
    CheckRun, CheckRunConclusion, CheckRunName, CheckRunOutput, CheckRunStatus,
};
use crate::git::HeadSha;
use crate::github::client::GitHubClient;
use crate::repository::RepositoryName;

#[derive(Debug, new)]
pub struct CreateCheckRun<'a> {
    github_client: &'a GitHubClient,
    owner: &'a Login,
    repository: &'a RepositoryName,
}

#[async_trait]
impl<'a> Action<CreateCheckRunInput, CheckRun, CreateCheckRunError> for CreateCheckRun<'a> {
    #[tracing::instrument]
    async fn execute(&self, input: CreateCheckRunInput) -> Result<CheckRun, CreateCheckRunError> {
        let url = format!(
            "/repos/{}/{}/check-runs",
            self.owner.get(),
            self.repository.get(),
        );

        let check_run = self
            .github_client
            .post(&url, Some(input))
            .await
            .context("failed to create check run")?;

        Ok(check_run)
    }
}

// TODO: Pass by reference, not by value (e.g. &HeadSha)
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, new)]
pub struct CreateCheckRunInput {
    name: CheckRunName,
    head_sha: HeadSha,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<CheckRunStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    conclusion: Option<CheckRunConclusion>,
    #[serde(skip_serializing_if = "Option::is_none")]
    completed_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output: Option<CheckRunOutput>,
}

#[derive(Debug, thiserror::Error)]
pub enum CreateCheckRunError {
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

#[cfg(test)]
mod tests {
    use crate::account::Login;
    use crate::action::Action;
    use crate::check_run::CheckRunStatus;
    use crate::repository::RepositoryName;
    use crate::testing::check_run::mock_create_check_run;
    use crate::testing::client::github_client;
    use crate::testing::token::mock_installation_access_tokens;

    use super::{CreateCheckRun, CreateCheckRunInput};

    #[tokio::test]
    async fn create_check_run_returns_check_run() {
        let _token_mock = mock_installation_access_tokens();
        let _content_mock = mock_create_check_run();

        let github_client = github_client();
        let owner = Login::new("github");
        let repository = RepositoryName::new("hello-world");

        let input = CreateCheckRunInput {
            name: "github-parts".into(),
            head_sha: "ce587453ced02b1526dfb4cb910479d431683101".into(),
            status: Some(CheckRunStatus::InProgress),
            conclusion: None,
            completed_at: None,
            output: None,
        };

        let check_run = CreateCheckRun::new(&github_client, &owner, &repository)
            .execute(input)
            .await
            .unwrap();

        assert_eq!(4, check_run.id().get());
    }
}
