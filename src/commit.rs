use std::string::ToString;
use std::{fs};
use serde::Serialize;
use crate::serialize::Serializing;


#[derive(Serialize)]
pub struct Commit {
    id: String,
    commit_message: String,
    timestamp: String,
    pub(crate) blobs: Vec<String>,
    // pub(crate) parent: Option<&'a Commit<'a>>,
}

impl Commit {
    pub fn commit(message: String, blobs: Vec<String>)
        -> String {
        let timestamp = "2021-01-01 00:00:00 +0000".to_string();

        let hash_result = Serializing::sha1_hash_commit(&message, &blobs, &timestamp);

        let commit = Commit {
            id: hash_result.clone(),
            commit_message: message,
            timestamp,
            blobs,
            // parent,
        };
        let commit_json = serde_json::to_string(&commit).unwrap();

        fs::write(format!(".gitlet/commits/{}.json", hash_result), &commit_json).unwrap();

        hash_result
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_message(&self) -> &str {
        &self.commit_message
    }

}