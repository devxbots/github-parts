use mockito::{mock, Mock};

pub fn mock_installation_access_tokens() -> Mock {
    mock("POST", "/app/installations/1/access_tokens")
        .with_status(200)
        .with_body(r#"{ "token": "ghs_16C7e42F292c6912E7710c838347Ae178B4a" }"#)
        .create()
}
