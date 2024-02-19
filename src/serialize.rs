use sha1::{Sha1, Digest};

pub(crate) struct Serialize {}

impl Serialize {

    pub fn sha1_hash(data: &str) -> String {
        let mut hasher = Sha1::new();
        hasher.update(data);
        let result = hasher.finalize();

        // Convert the output to a byte array
        let bytes: [u8; 20] = result.into();

        // Format the byte array as a hexadecimal string
        let hex_string = bytes.iter().map(|b| format!("{:02x}", b)).collect::<String>();

        hex_string
    }


}

