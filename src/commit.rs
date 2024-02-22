use std::string::ToString;
use std::{fs};
use std::collections::HashMap;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use crate::serialize::Serializing;

#[derive(Serialize, Deserialize)]
pub struct Commit {
    id: String,
    commit_message: String,
    timestamp: String,
    pub(crate) blobs: HashMap<String, String>,
}

impl Commit {
    pub fn commit(message: String, blobs: HashMap<String, String>)
        -> String {
        let dt = Utc::now().naive_utc();
        let timestamp = dt.format("%Y-%m-%d %H:%M:%S").to_string();

        let hash_result = Serializing::sha1_hash_commit(&message, &blobs, &timestamp.to_string());

        let commit = Commit {
            id: hash_result.clone(),
            commit_message: message,
            timestamp: timestamp.to_string(),
            blobs,
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

    pub fn get_timestamp(&self) -> &str {
        &self.timestamp
    }

}