
pub struct Staged {
    files: Vec<String>,
}

impl Staged {
    pub fn new() -> Staged {
        Staged {
            files: Vec::new(),
        }
    }

    pub fn add(&mut self, file_hash: String) {
        // let hash = Serialize::sha1_hash(&file);
        self.files.push(file_hash);
    }

    pub fn get_files(&self) -> &Vec<String> {
        &self.files
    }
}