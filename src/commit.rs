use std::collections::HashMap;
use std::time::SystemTime;
use std::string::ToString;
use std::ptr;
use sha1::{Digest, Sha1};
use crate::serialize::Serialize;


pub struct Commit<'a> {
    id: String,
    commit_message: String,
    timestamp: String,
    pub(crate) blobs: Vec<String>,
    pub(crate) parent: Option<&'a Commit<'a>>,
}

impl Commit<'_> {
    pub fn commit<'a>(message: String, blobs: Vec<String>, parent: Option<&'a Commit<'a>>)
        -> String {
        let timestamp = "2021-01-01 00:00:00 +0000".to_string();


        let hash_result = Serialize::sha1_hash_commit(&message, &blobs, &timestamp);

        let commit = Commit {
            id: hash_result.clone(),
            commit_message: message,
            timestamp,
            blobs,
            parent,
        };

        hash_result
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_message(&self) -> &str {
        &self.commit_message
    }

}