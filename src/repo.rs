use std::collections::HashMap;
use std::string::ToString;
use std::fs;
use std::io::Read;

use crate::commit::Commit;
use crate::staged::Staged;
use crate::serialize::Serialize;

pub struct Repo<'a>{
    head: String,
    staged: Staged,
    parent: Option<&'a Commit<'a>>,
    file_hash_dict: HashMap<String, String>,
}

impl Repo<'_> {
    pub fn new() -> Repo<'static>  {
        Repo {
            head: "master".to_string(),
            staged: Staged::new(),
            parent: None,
            file_hash_dict: HashMap::new(),
        }
    }


    pub fn init(&mut self) {
        fs::create_dir(".gitlet").unwrap();
        fs::create_dir(".gitlet/blobs").unwrap();
        fs::create_dir(".gitlet/branches").unwrap();
        fs::create_dir(".gitlet/commits").unwrap();
        fs::create_dir(".gitlet/staged").unwrap();
        // fs::create_dir(".gitlet/refs/tags").unwrap();

        println!("init");
        self.copy_files_and_hash("code/", ".gitlet/blobs/");

        let master = Commit::commit("initial commit".to_string(), Vec::new(), None);
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
                            let hash = Serialize::sha1_hash(&file_content);
                            fs::copy(&file_path, format!(".gitlet/blobs/{}", hash)).unwrap();
                            self.file_hash_dict.insert(file_str.to_string(), hash);
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
        let hash = Serialize::sha1_hash(&file);
        if !self.staged.get_files().contains(&hash) {
            fs::copy(&file, format!(".gitlet/staged/{}", file)).unwrap();
            self.staged.add(hash);
        }
    }

    pub fn get_staged(&self) -> &Vec<String> {
        self.staged.get_files()
    }

    pub fn commit<'a>(&mut self, message: String) {
        let paths = fs::read_dir(".gitlet/staged").unwrap();
        let mut blobs: Vec<String> = Vec::new();
        for path in paths {
            let file = path.unwrap().path();
            let file = file.to_str().unwrap();
            let hash = Serialize::sha1_hash(file);
            fs::copy(file, format!(".gitlet/blobs/{}", hash)).unwrap();
            blobs.push(hash);
            fs::remove_file(file).unwrap();
        }
        let commit = Commit::commit(message, blobs, None);
        println!("commit id: {}", commit);
        // self.prev_commit = Option::from(commit);
        // let commit_hash = Serialize::sha1_hash(&commit.id);
    }

}