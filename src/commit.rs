use std::collections::HashMap;
use std::time::SystemTime;
use std::string::ToString;
use std::ptr;

pub struct Commit<'a> {
    id: String,
    commit_message: String,
    timestamp: String,
    mapping: HashMap<String, String>,
    parent: Option<&'a Commit<'a>>,
}

impl Commit<'_> {
    pub fn commit(message: String) -> Commit<'static> {
        let timestamp = "2021-01-01 00:00:00 +0000".to_string();
        let id = "1234".to_string();
        let parent = None;
        Commit {
            id,
            commit_message: message,
            timestamp,
            mapping: HashMap::new(),
            parent,
        }
    }
}