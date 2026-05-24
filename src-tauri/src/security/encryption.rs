use ring::aead::{Aad, LessSafeKey, Nonce, UnboundKey, AES_256_GCM};
use ring::rand::{SecureRandom, SystemRandom};

pub struct Encryption {
    key: LessSafeKey,
    rng: SystemRandom,
}

impl Encryption {
    pub fn new(key_bytes: &[u8]) -> Result<Self, String> {
        if key_bytes.len() != 32 {
            return Err("AES-256 requires 32-byte key".into());
        }
        let unbound_key =
            UnboundKey::new(&AES_256_GCM, key_bytes).map_err(|e| format!("Key error: {}", e))?;
        Ok(Self {
            key: LessSafeKey::new(unbound_key),
            rng: SystemRandom::new(),
        })
    }

    pub fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, String> {
        let mut nonce_bytes = [0u8; 12];
        self.rng
            .fill(&mut nonce_bytes)
            .map_err(|e| format!("RNG error: {}", e))?;
        let nonce = Nonce::assume_unique_for_key(nonce_bytes);

        let mut in_out = data.to_vec();
        self.key
            .seal_in_place_append_tag(nonce, Aad::empty(), &mut in_out)
            .map_err(|e| format!("Encrypt error: {}", e))?;

        let mut result = nonce_bytes.to_vec();
        result.extend_from_slice(&in_out);
        Ok(result)
    }

    pub fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>, String> {
        if data.len() < 12 {
            return Err("Data too short".into());
        }
        let (nonce_bytes, ciphertext) = data.split_at(12);
        let nonce = Nonce::assume_unique_for_key(
            nonce_bytes.try_into().map_err(|_| "Bad nonce length")?,
        );

        let mut in_out = ciphertext.to_vec();
        let plaintext = self
            .key
            .open_in_place(nonce, Aad::empty(), &mut in_out)
            .map_err(|e| format!("Decrypt error: {}", e))?;

        Ok(plaintext.to_vec())
    }
}
