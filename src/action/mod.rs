use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub mod get_file;
pub mod list_check_runs;

#[async_trait]
pub trait Action<Input, Output, Error>
where
    Input: Serialize,
    Output: DeserializeOwned,
    Error: std::error::Error,
{
    async fn execute(&self, input: &Input) -> Result<Output, Error>;
}
