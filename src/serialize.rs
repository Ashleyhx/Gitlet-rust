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

    pub fn sha1_hash_commit(message: &str, blobs: &Vec<String>, timestamp: &str) -> String {
        let mut hasher = Sha1::new();
        let mut data = message.to_string();
        for blob in blobs {
            data.push_str(blob);
        }
        data.push_str(timestamp);
        hasher.update(data);
        let result = hasher.finalize();

        // Convert the output to a byte array
        let bytes: [u8; 20] = result.into();

        // Format the byte array as a hexadecimal string
        let hex_string = bytes.iter().map(|b| format!("{:02x}", b)).collect::<String>();

        hex_string
    }


}
