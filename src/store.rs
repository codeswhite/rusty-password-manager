use serde::{Deserialize, Serialize};

use bincode::{deserialize, serialize};
use chacha20poly1305::aead::{rand_core::RngCore, OsRng};
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use crate::crypto;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Store {
    pub name: String,
    pub entries: Vec<Entry>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Entry {
    pub name: String,
    pub username: Option<String>,
    pub password: Option<String>,
}

impl Store {
    pub fn save_store(
        &self,
        store_path: &Path,
        password: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Serializing
        let packed_store = serialize(self)?;

        // Generate 12 bytes salt
        let mut salt = [0u8; 12];
        OsRng.fill_bytes(&mut salt);

        // Hash the password
        let pwd_hash = crypto::argon2_hash(password.as_bytes(), &salt);

        // Encrypt data
        println!("Encrypting..");
        let encrypted = crypto::encrypt(&pwd_hash, &packed_store).expect("Failed encrypting!");

        // Prepend salt to data
        let mut combined_data = salt.to_vec();
        combined_data.extend(encrypted);

        // Write to file
        println!("Saving to '{}'..", store_path.to_str().unwrap());
        File::create(store_path)
            .unwrap()
            .write_all(&combined_data)?;

        Ok(())
    }

    pub fn load_store(store_path: &Path, password: &str) -> Self {
        // Try to open file
        let data = fs::read(store_path)
            .unwrap_or_else(|err| panic!("Could not read file, with error: {}", err));

        // Extract the 12 bytes salt
        let salt: &[u8; 12] = data[..12]
            .try_into()
            .unwrap_or_else(|_| panic!("Invalid salt inside file"));

        // Hash the password
        let pwd_hash = crypto::argon2_hash(password.as_bytes(), salt);

        // Decrypt data
        let encrypted_data = &data[12..];
        let decrypted =
            crypto::decrypt(&pwd_hash, encrypted_data).expect("Failed decrypting, wrong key?");

        // Parse store
        deserialize(&decrypted).unwrap()
    }
}

#[allow(unused_imports)]
mod tests {
    use super::*;
    use std::{fs::create_dir, path::PathBuf};

    #[test]
    fn test_save_load() {
        let created_store = Store {
            name: "testvault".to_string(),
            entries: vec![Entry {
                name: "testent".to_string(),
                username: Some("sigma".to_string()),
                password: None,
            }],
        };

        // Change this on Windows..
        let tmp_file = PathBuf::from("/tmp/testing-rusty-pm");
        let arbitrary_pwd = "123";

        created_store.save_store(&tmp_file, arbitrary_pwd).unwrap();
        let loaded_store = Store::load_store(&tmp_file, arbitrary_pwd);

        assert_eq!(created_store, loaded_store)
    }
}
