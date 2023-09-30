use std::time::Instant;

use argon2::{hash_raw, Config};
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305, Nonce,
};

pub fn argon2_hash(key: &[u8], salt: &[u8]) -> Vec<u8> {
    println!("Hashing password..");
    let start = Instant::now();
    let res = hash_raw(
        key,
        salt,
        &Config {
            mem_cost: 32 * 1024, // 32 MB
            lanes: 6,
            time_cost: 5,
            variant: argon2::Variant::Argon2id,
            ..argon2::Config::default()
        },
    );
    println!("Hashing took: {:?}", start.elapsed());
    res.unwrap()
}

pub fn decrypt(key: &[u8], data: &[u8]) -> Result<Vec<u8>, chacha20poly1305::Error> {
    let cipher = ChaCha20Poly1305::new(key.into());

    // Extract 12 bytes nonce from data
    let nonce_length = 12;
    let nonce: &Nonce = data[..nonce_length].into();
    let data_part = &data[nonce_length..];

    cipher.decrypt(nonce, data_part)
}

pub fn encrypt(key: &[u8], data: &[u8]) -> Result<Vec<u8>, chacha20poly1305::Error> {
    let cipher = ChaCha20Poly1305::new(key.into());
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);

    // Concat the nonce + encrypted data
    let mut combined_data = nonce.to_vec();

    let encrypted_data = cipher.encrypt(&nonce, data).to_owned()?;

    combined_data.extend(encrypted_data);
    Ok(combined_data)
}

#[allow(unused_imports)]
mod tests {
    // Those lines appear to me as unused even though they are in use, my misconfiguration or a bug
    use super::*;
    use chacha20poly1305::aead::rand_core::RngCore;

    #[test]
    fn test_enc_dec() {
        // Generate random 32-byte key
        let mut rand_key = [0u8; 32];
        OsRng.fill_bytes(&mut rand_key);

        // Random data
        let some_data = [0x42; 100];

        let encrypted = encrypt(&rand_key, &some_data).unwrap();
        let decrypted = decrypt(&rand_key, &encrypted).unwrap();

        // Make sure we are not losing stuff on the way (it would be funny)
        assert_eq!(decrypted, some_data.to_vec());
    }

    #[test]
    fn test_hash() {
        // Generate random 12-byte salt
        let mut rand_salt = [0u8; 12];
        OsRng.fill_bytes(&mut rand_salt);

        let mega_passward = "1234";

        // Just make sure that two calls with the same params result in the same hash.
        assert_eq!(
            argon2_hash(mega_passward.as_bytes(), &rand_salt),
            argon2_hash(mega_passward.as_bytes(), &rand_salt)
        )
    }
}
