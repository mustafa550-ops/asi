use std::collections::HashMap;
use crate::security::Encryption;

pub struct Keyring {
    keys: HashMap<String, Vec<u8>>,
    encryption: Encryption,
}

impl Keyring {
    pub fn new(master_key: &[u8]) -> Result<Self, String> {
        Ok(Self {
            keys: HashMap::new(),
            encryption: Encryption::new(master_key)?,
        })
    }

    pub fn set(&mut self, name: &str, value: &str) -> Result<(), String> {
        let encrypted = self.encryption.encrypt(value.as_bytes())?;
        self.keys.insert(name.to_string(), encrypted);
        Ok(())
    }

    pub fn get(&self, name: &str) -> Option<String> {
        self.keys.get(name).and_then(|enc| {
            self.encryption.decrypt(enc).ok().and_then(|bytes| {
                String::from_utf8(bytes).ok()
            })
        })
    }

    pub fn delete(&mut self, name: &str) {
        self.keys.remove(name);
    }

    pub fn list(&self) -> Vec<&String> {
        self.keys.keys().collect()
    }
}
