use ring::signature::{Ed25519KeyPair, KeyPair, UnparsedPublicKey, ED25519};
use ring::rand::SystemRandom;

pub struct ModuleSigner {
    key_pair: Ed25519KeyPair,
}

impl ModuleSigner {
    pub fn new() -> Result<Self, String> {
        let rng = SystemRandom::new();
        let pkcs8_bytes = Ed25519KeyPair::generate_pkcs8(&rng)
            .map_err(|e| format!("Anahtar uretimi basarisiz: {}", e))?;
        let key_pair = Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref())
            .map_err(|e| format!("Anahtar yukleme basarisiz: {}", e))?;
        Ok(Self { key_pair })
    }

    pub fn from_private_key(pkcs8: &[u8]) -> Result<Self, String> {
        let key_pair = Ed25519KeyPair::from_pkcs8(pkcs8)
            .map_err(|e| format!("Anahtar yukleme basarisiz: {}", e))?;
        Ok(Self { key_pair })
    }

    pub fn sign(&self, data: &[u8]) -> Vec<u8> {
        self.key_pair.sign(data).as_ref().to_vec()
    }

    pub fn public_key(&self) -> Vec<u8> {
        self.key_pair.public_key().as_ref().to_vec()
    }

    pub fn verify(public_key: &[u8], data: &[u8], signature: &[u8]) -> Result<(), String> {
        let public_key = UnparsedPublicKey::new(&ED25519, public_key);
        public_key.verify(data, signature)
            .map_err(|_| "Imza dogrulamasi basarisiz".to_string())
    }

    pub fn sign_module(module_data: &[u8]) -> Result<(Vec<u8>, Vec<u8>, Vec<u8>), String> {
        let signer = Self::new()?;
        let signature = signer.sign(module_data);
        Ok((signer.public_key(), signature, module_data.to_vec()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign_and_verify() {
        let data = b"test module data";
        let (pk, sig, _) = ModuleSigner::sign_module(data).unwrap();
        assert!(ModuleSigner::verify(&pk, data, &sig).is_ok());
    }

    #[test]
    fn test_verify_wrong_data() {
        let data = b"test module data";
        let (pk, sig, _) = ModuleSigner::sign_module(data).unwrap();
        let result = ModuleSigner::verify(&pk, b"wrong data", &sig);
        assert!(result.is_err());
    }

    #[test]
    fn test_public_key_length() {
        let signer = ModuleSigner::new().unwrap();
        assert_eq!(signer.public_key().len(), 32);
    }

    #[test]
    fn test_signature_length() {
        let signer = ModuleSigner::new().unwrap();
        let sig = signer.sign(b"test");
        assert_eq!(sig.len(), 64);
    }
}
