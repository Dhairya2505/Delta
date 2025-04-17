use rand::RngCore;
use base64::{engine::general_purpose, Engine};

pub fn generate_random_id() -> String {
    let mut bytes = [0u8; 64];
    rand::rng().fill_bytes(&mut bytes);

    // Encode as base64 for readability
    general_purpose::STANDARD.encode(&bytes)
}