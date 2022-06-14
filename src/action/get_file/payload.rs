use anyhow::Context;
use base64::decode;
use serde::Deserialize;
use serde_json::Value;

use crate::action::get_file::{GetFileError, GetFileResult};

#[derive(Clone, Eq, PartialEq, Debug, Deserialize)]
#[serde(untagged)]
pub enum GetFileResponse {
    Error(GetFileErrorPayload),
    Success(Value),
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum GetFilePayload {
    Directory,
    File(FilePayload),
    Submodule,
    Symlink,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize)]
pub struct FilePayload {
    encoding: FileEncoding,
    size: u64,
    name: String,
    path: String,
    content: String,
    sha: String,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FileEncoding {
    Base64,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize)]
pub struct GetFileErrorPayload {
    message: String,
}

impl TryFrom<GetFilePayload> for GetFileResult {
    type Error = GetFileError;

    #[tracing::instrument]
    fn try_from(value: GetFilePayload) -> Result<Self, Self::Error> {
        let payload = match value {
            GetFilePayload::Directory => Err(GetFileError::Directory),
            GetFilePayload::File(payload) => Ok(payload),
            GetFilePayload::Submodule => Err(GetFileError::Submodule),
            GetFilePayload::Symlink => Err(GetFileError::Symlink),
        }?;

        let sanitized_content = &payload.content.replace('\n', "");
        let content =
            decode(sanitized_content).context("failed to decode Base64 encoded file content")?;

        Ok(GetFileResult {
            size: payload.size,
            name: payload.name,
            path: payload.path.into(),
            sha: payload.sha,
            content,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::action::get_file::GetFileResult;

    use super::{FileEncoding, FilePayload, GetFilePayload};

    const PAYLOAD: &str = r#"
        {
          "name": "README.md",
          "path": "README.md",
          "sha": "3928ebd3c7db689f5ea5b11db6bfa89b132139c3",
          "size": 1012,
          "url": "https://api.github.com/repos/devxbots/github-parts/contents/README.md?ref=main",
          "html_url": "https://github.com/devxbots/github-parts/blob/main/README.md",
          "git_url": "https://api.github.com/repos/devxbots/github-parts/git/blobs/3928ebd3c7db689f5ea5b11db6bfa89b132139c3",
          "download_url": "https://raw.githubusercontent.com/devxbots/github-parts/main/README.md",
          "type": "file",
          "content": "IyDwn5SpIGdpdGh1Yi1wYXJ0cwoKYGdpdGh1Yi1wYXJ0c2AgaXMgYW4gb3Bp\nbmlvbmF0ZWQgaW50ZWdyYXRpb24gd2l0aCB0aGUgR2l0SHViIHBsYXRmb3Jt\nLCB1c2VkIGJ5CltERVYgeCBCT1RTXSBmb3IgaXRzIGF1dG9tYXRpb25zLiBJ\ndCBjb250YWlucyBfdHlwZXNfIGZvciB0aGUgcmVzb3VyY2VzIHByb3ZpZGVk\nCmJ5IEdpdEh1YiB0aHJvdWdoIGl0cyBBUEksIGFuZCBfYWN0aW9uc18gdGhh\ndCBjYW4gaW50ZXJhY3Qgd2l0aCB0aGUgcGxhdGZvcm0gaW4KdmFyaW91cyB3\nYXlzLgoKIyMgU3RhdHVzCgpXZSBhcmUgYWN0aXZlbHkgZGV2ZWxvcGluZyBg\nZ2l0aHViLXBhcnRzYC4gSXRzIEFQSSBhbmQgZnVuY3Rpb25hbGl0eSBhcmUg\nbm90CnN0YWJsZSBhbmQgY2FuIGNoYW5nZSBhdCBhbnkgdGltZS4gRHVyaW5n\nIHRoaXMgcGVyaW9kLCB3ZSBkb24ndCBhY2NlcHQgY29kZQpjb250cmlidXRp\nb25zIHRvIHRoZSBwcm9qZWN0LgoKIyMgTGljZW5zZQoKTGljZW5zZWQgdW5k\nZXIgZWl0aGVyIG9mCgotIEFwYWNoZSBMaWNlbnNlLCBWZXJzaW9uIDIuMCAo\nW0xJQ0VOU0UtQVBBQ0hFXShMSUNFTlNFLUFQQUNIRSkgb3IgPGh0dHA6Ly93\nd3cuYXBhY2hlLm9yZy9saWNlbnNlcy9MSUNFTlNFLTIuMD4pCi0gTUlUIGxp\nY2Vuc2UgKFtMSUNFTlNFLU1JVF0oTElDRU5TRS1NSVQpIG9yIDxodHRwOi8v\nb3BlbnNvdXJjZS5vcmcvbGljZW5zZXMvTUlUPikKCmF0IHlvdXIgb3B0aW9u\nLgoKIyMgQ29udHJpYnV0aW9uCgpVbmxlc3MgeW91IGV4cGxpY2l0bHkgc3Rh\ndGUgb3RoZXJ3aXNlLCBhbnkgY29udHJpYnV0aW9uIGludGVudGlvbmFsbHkg\nc3VibWl0dGVkCmZvciBpbmNsdXNpb24gaW4gdGhlIHdvcmsgYnkgeW91LCBh\ncyBkZWZpbmVkIGluIHRoZSBBcGFjaGUtMi4wIGxpY2Vuc2UsIHNoYWxsIGJl\nCmR1YWwgbGljZW5zZWQgYXMgYWJvdmUsIHdpdGhvdXQgYW55IGFkZGl0aW9u\nYWwgdGVybXMgb3IgY29uZGl0aW9ucy4KCltkZXYgeCBib3RzXTogaHR0cHM6\nLy9naXRodWIuY29tL2Rldnhib3RzCg==\n",
          "encoding": "base64",
          "_links": {
            "self": "https://api.github.com/repos/devxbots/github-parts/contents/README.md?ref=main",
            "git": "https://api.github.com/repos/devxbots/github-parts/git/blobs/3928ebd3c7db689f5ea5b11db6bfa89b132139c3",
            "html": "https://github.com/devxbots/github-parts/blob/main/README.md"
          }
        }
    "#;

    #[test]
    fn trait_deserialize() {
        let payload: GetFilePayload = serde_json::from_str(PAYLOAD).unwrap();

        assert!(matches!(payload, GetFilePayload::File(_)));
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<GetFilePayload>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<GetFilePayload>();
    }

    #[test]
    fn trait_try_from() {
        let payload = GetFilePayload::File(FilePayload {
            encoding: FileEncoding::Base64,
            size: 1012,
            name: "README.md".to_string(),
            path: "README.md".to_string(),
            content: "IyDwn5SpIGdpdGh1Yi1wYXJ0cwoKYGdpdGh1Yi1wYXJ0c2AgaXMgYW4gb3Bp\nbmlvbmF0ZWQgaW50ZWdyYXRpb24gd2l0aCB0aGUgR2l0SHViIHBsYXRmb3Jt\nLCB1c2VkIGJ5CltERVYgeCBCT1RTXSBmb3IgaXRzIGF1dG9tYXRpb25zLiBJ\ndCBjb250YWlucyBfdHlwZXNfIGZvciB0aGUgcmVzb3VyY2VzIHByb3ZpZGVk\nCmJ5IEdpdEh1YiB0aHJvdWdoIGl0cyBBUEksIGFuZCBfYWN0aW9uc18gdGhh\ndCBjYW4gaW50ZXJhY3Qgd2l0aCB0aGUgcGxhdGZvcm0gaW4KdmFyaW91cyB3\nYXlzLgoKIyMgU3RhdHVzCgpXZSBhcmUgYWN0aXZlbHkgZGV2ZWxvcGluZyBg\nZ2l0aHViLXBhcnRzYC4gSXRzIEFQSSBhbmQgZnVuY3Rpb25hbGl0eSBhcmUg\nbm90CnN0YWJsZSBhbmQgY2FuIGNoYW5nZSBhdCBhbnkgdGltZS4gRHVyaW5n\nIHRoaXMgcGVyaW9kLCB3ZSBkb24ndCBhY2NlcHQgY29kZQpjb250cmlidXRp\nb25zIHRvIHRoZSBwcm9qZWN0LgoKIyMgTGljZW5zZQoKTGljZW5zZWQgdW5k\nZXIgZWl0aGVyIG9mCgotIEFwYWNoZSBMaWNlbnNlLCBWZXJzaW9uIDIuMCAo\nW0xJQ0VOU0UtQVBBQ0hFXShMSUNFTlNFLUFQQUNIRSkgb3IgPGh0dHA6Ly93\nd3cuYXBhY2hlLm9yZy9saWNlbnNlcy9MSUNFTlNFLTIuMD4pCi0gTUlUIGxp\nY2Vuc2UgKFtMSUNFTlNFLU1JVF0oTElDRU5TRS1NSVQpIG9yIDxodHRwOi8v\nb3BlbnNvdXJjZS5vcmcvbGljZW5zZXMvTUlUPikKCmF0IHlvdXIgb3B0aW9u\nLgoKIyMgQ29udHJpYnV0aW9uCgpVbmxlc3MgeW91IGV4cGxpY2l0bHkgc3Rh\ndGUgb3RoZXJ3aXNlLCBhbnkgY29udHJpYnV0aW9uIGludGVudGlvbmFsbHkg\nc3VibWl0dGVkCmZvciBpbmNsdXNpb24gaW4gdGhlIHdvcmsgYnkgeW91LCBh\ncyBkZWZpbmVkIGluIHRoZSBBcGFjaGUtMi4wIGxpY2Vuc2UsIHNoYWxsIGJl\nCmR1YWwgbGljZW5zZWQgYXMgYWJvdmUsIHdpdGhvdXQgYW55IGFkZGl0aW9u\nYWwgdGVybXMgb3IgY29uZGl0aW9ucy4KCltkZXYgeCBib3RzXTogaHR0cHM6\nLy9naXRodWIuY29tL2Rldnhib3RzCg==\n".to_string(),
            sha: "3928ebd3c7db689f5ea5b11db6bfa89b132139c3".to_string()
        });

        let result =
            GetFileResult::try_from(payload).expect("failed to convert payload into a result");

        let content =
            String::from_utf8(result.content).expect("failed to convert content to string");

        assert!(content.starts_with("# 🔩 github-parts"))
    }
}
