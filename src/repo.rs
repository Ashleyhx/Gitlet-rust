use std::string::ToString;
use std::fs;

use crate::commit::Commit;
use crate::staged::Staged;
use crate::serialize::Serialize;

pub struct Repo {
    head: String,
    staged: Staged,
}

impl Repo {
    pub fn new() -> Repo {
        Repo {
            head: "master".to_string(),
            staged: Staged::new(),
        }
    }

    pub fn init(&self) {
        fs::create_dir(".gitlet").unwrap();
        fs::create_dir(".gitlet/blobs").unwrap();
        fs::create_dir(".gitlet/branches").unwrap();
        fs::create_dir(".gitlet/commits").unwrap();
        fs::create_dir(".gitlet/staged").unwrap();
        // fs::create_dir(".gitlet/refs/tags").unwrap();

        let master = Commit::commit("initial commit".to_string());
    }


    pub fn get_head(&self) -> &str {
        &self.head
    }

    pub fn add(&mut self, file: String) {
        let hash = Serialize::sha1_hash(&file);
        if !self.staged.get_files().contains(&hash) {
            fs::copy(&file, format!(".gitlet/staged/{}", file)).unwrap();
            self.staged.add(hash);
        }
    }

    pub fn get_staged(&self) -> &Vec<String> {
        self.staged.get_files()
    }

    pub fn commit(&self, message: String) {
        let commit = Commit::commit(message);
        self.staged.get_files().iter().for_each(|file| {
            let contents = fs::read_to_string(file).unwrap();
            fs::write(format!(".gitlet/blobs/{}", file), contents).unwrap();
        });
    }

}