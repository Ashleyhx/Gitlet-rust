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
}

impl Repo{
    pub fn new() -> Repo  {
        Repo {
            head: "master".to_string(),
            staged: Staged::new(),
            // parent: None,
            file_hash_dict: HashMap::new(),
            file_hash_staged: HashMap::new(),
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

        let master = Commit::commit("initial commit".to_string(), Vec::new());
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
                            println!("1111hash: {}", hash);
                            println!("1111file: {} ", file_path.to_str().unwrap());
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
            println!("added {}", file);
            println!("hash {}", hash);
            self.file_hash_staged.insert(file, hash.clone());

            self.staged.add(hash);

        }
    }

    pub fn get_staged(&self) -> &Vec<String> {
        self.staged.get_files()
    }

    pub fn commit(&mut self, message: String) {
        let paths = fs::read_dir(".gitlet/staged").unwrap();

        let mut blobs: Vec<String> = Vec::new();
        for path in paths {
            let file = path.unwrap().path();
            let file_name = file.file_name().unwrap().to_string_lossy().into_owned();

            let dest_path = format!(".gitlet/blobs/{}", &file_name);
            fs::rename(file, &dest_path).unwrap();
            blobs.push(file_name);
        }
        for (file, hash) in &self.file_hash_dict{
            println!("AAAAAfile: {}", file);
            println!("AAAAAhash: {}", hash);
        }

        for (file, hash) in &self.file_hash_staged {
            self.file_hash_dict.insert(file.clone(), hash.clone());
        }

        self.file_hash_staged.clear();

        let commit = Commit::commit(message, blobs);
        println!("commit id: {}", commit);
    }
}