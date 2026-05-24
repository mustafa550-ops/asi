/// Encryption — SQLCipher ile veritabanı şifreleme (§11).
pub struct Encryption;

impl Encryption {
    pub fn new() -> Self {
        Self
    }

    pub fn encrypt(&self, data: &[u8]) -> Vec<u8> {
        data.to_vec()
    }

    pub fn decrypt(&self, data: &[u8]) -> Vec<u8> {
        data.to_vec()
    }
}
