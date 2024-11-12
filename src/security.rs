use ic_cdk::export::Principal;
use crypto_hash::{Algorithm, hex_digest};

pub fn validate_principal(user: Principal) -> bool {
    // Validasi apakah user memiliki izin (dapat dikembangkan lebih lanjut)
    true
}

pub fn hash_content(content: &[u8]) -> String {
    // Hash data sebelum disimpan untuk keamanan
    hex_digest(Algorithm::SHA256, content)
}
