use std::collections::HashMap;
use std::string::ToString;
use std::fs;
use std::io::Read;
use std::path::Path;

use crate::commit::Commit;
use crate::staged::Staged;
use crate::serialize::Serializing;

pub struct Repo{
    head: String,
    staged: Staged,
    file_hash_dict: HashMap<String, String>,
    file_hash_staged: HashMap<String, String>,
    current_commit: Option<String>,
    branches: HashMap<String, String>,
}

impl Repo{
    pub fn new() -> Repo  {
        Repo {
            head: "master".to_string(),
            staged: Staged::new(),
            file_hash_dict: HashMap::new(),
            file_hash_staged: HashMap::new(),
            current_commit: None,
            branches: HashMap::new(),
        }
    }


    pub fn init(&mut self) {
        fs::create_dir(".gitlet").unwrap();
        fs::create_dir(".gitlet/blobs").unwrap();
        fs::create_dir(".gitlet/branches").unwrap();
        fs::create_dir(".gitlet/commits").unwrap();
        fs::create_dir(".gitlet/staged").unwrap();

        self.copy_files_and_hash("code/", ".gitlet/blobs/");

        let cur_commit =
            Commit::commit("initial commit".to_string(), HashMap::new(), None);
        self.current_commit = Some(cur_commit);
    }


    fn copy_files_and_hash(&mut self, src_dir: &str, dest_dir: &str) {
        for entry in fs::read_dir(src_dir).unwrap() {
            let entry = entry.unwrap();
            let file_path = entry.path();
            if let Some(file_name) = file_path.file_name() {
                if let Some(file_str) = file_name.to_str() {
                    if file_str != ".gitlet" {
                        let dest_path = format!("{}{}", dest_dir, file_str);
                        if file_path.is_dir() {
                            self.copy_files_and_hash(&file_path.to_str().unwrap(), &dest_path);
                        } else {
                            let mut file_content = String::new();
                            let mut file = fs::File::open(&file_path).unwrap();
                            file.read_to_string(&mut file_content).unwrap();
                            let hash = Serializing::sha1_hash(&file_content);
                            fs::copy(&file_path, format!(".gitlet/blobs/{}", hash)).unwrap();
                            self.file_hash_dict.insert(file_path.to_str().unwrap().to_string(), hash);
                        }
                    }
                }
            }
        }
    }

    pub fn get_head(&self) -> &str {
        &self.head
    }

    pub fn add(&mut self, file: String) {
        let file_path = Path::new(&file);
        if !file_path.exists() {
            eprintln!("File not found: {}", file);
            return;
        }
        let hash = Serializing::sha1_hash(&file);
        if !self.staged.get_files().contains(&hash) {
            let dest_path = format!(".gitlet/staged/{}", hash);
            if let Err(err) = fs::copy(&file, &dest_path) {
                eprintln!("Error copying file: {}", err);
                return;
            }
            self.file_hash_staged.insert(file, hash.clone());
            self.staged.add(hash);
        }
    }

    pub fn get_staged(&self) -> &Vec<String> {
        self.staged.get_files()
    }

    pub fn commit(&mut self, message: String) {
        let paths = fs::read_dir(".gitlet/staged").unwrap();
        let file_hash_staged_clone = self.file_hash_staged.clone();
        let current_commit = self.current_commit.clone();
        for path in paths {
            let file = path.unwrap().path();
            let file_name = file.file_name().unwrap().to_string_lossy().into_owned();

            let dest_path = format!(".gitlet/blobs/{}", &file_name);
            fs::rename(file, &dest_path).unwrap();
        }
        let commit =
            Commit::commit(message, file_hash_staged_clone, current_commit);
        self.current_commit = Some(commit);
        for (file, hash) in &self.file_hash_staged {
            self.file_hash_dict.insert(file.clone(), hash.clone());
        }
        self.file_hash_staged.clear();
    }

    pub fn log(&self) {
        let commit_dir = fs::read_dir(".gitlet/commits").unwrap();
        for entry in commit_dir {
            println!("=====================");
            let entry = entry.unwrap();
            let file = fs::File::open(entry.path()).unwrap();
            let commit: Commit = serde_json::from_reader(file).unwrap();
            println!("commit {}", commit.get_id());
            println!("Date: {}", commit.get_timestamp());
            println!("{}", commit.get_message());
        }
    }

    pub fn checkout(&self, commit_id: &str) {
        let commit_path = format!(".gitlet/commits/{}.json", commit_id);
        let commit_file = fs::File::open(commit_path).unwrap();
        let commit: Commit = serde_json::from_reader(commit_file).unwrap();
        for (file, hash) in &commit.blobs {
            let dest_path = format!(".gitlet/blobs/{}", hash);
            fs::copy(&dest_path, file).unwrap();
        }
    }
}