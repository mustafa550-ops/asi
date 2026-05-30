use std::time::{Duration, Instant};

pub struct QuarantineEntry {
    pub module_name: String,
    pub added_at: Instant,
    pub restrictions: Vec<String>,
}

pub struct Quarantine {
    entries: Vec<QuarantineEntry>,
    quarantine_duration: Duration,
}

impl Quarantine {
    pub fn new(duration_hours: u64) -> Self {
        Self {
            entries: Vec::new(),
            quarantine_duration: Duration::from_secs(duration_hours * 3600),
        }
    }

    pub fn add(&mut self, module_name: &str) {
        self.entries.push(QuarantineEntry {
            module_name: module_name.to_string(),
            added_at: Instant::now(),
            restrictions: vec![
                "sadece log yazabilir".into(),
                "dis ag erisimi yok".into(),
                "dosya sistemi salt-okunur".into(),
            ],
        });
        log::info!("Module '{}' 24 saat karantinaya alindi", module_name);
    }

    pub fn release(&mut self, module_name: &str) -> Result<(), String> {
        if let Some(pos) = self.entries.iter().position(|e| e.module_name == module_name) {
            let _entry = self.entries.remove(pos);
            log::info!("Module '{}' karantinadan cikarildi", module_name);
            Ok(())
        } else {
            Err(format!("Module '{}' karantinada bulunamadi", module_name))
        }
    }

    pub fn check_release(&mut self, module_name: &str) -> bool {
        if let Some(entry) = self.entries.iter().find(|e| e.module_name == module_name) {
            entry.added_at.elapsed() >= self.quarantine_duration
        } else {
            true
        }
    }

    pub fn is_quarantined(&self, module_name: &str) -> bool {
        self.entries.iter().any(|e| e.module_name == module_name)
    }

    pub fn list(&self) -> Vec<&QuarantineEntry> {
        self.entries.iter().filter(|e| e.added_at.elapsed() < self.quarantine_duration).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_check() {
        let mut q = Quarantine::new(24);
        q.add("test_module");
        assert!(q.is_quarantined("test_module"));
        assert!(!q.is_quarantined("other"));
    }

    #[test]
    fn test_release() {
        let mut q = Quarantine::new(24);
        q.add("test_module");
        assert!(q.release("test_module").is_ok());
        assert!(!q.is_quarantined("test_module"));
    }

    #[test]
    fn test_release_nonexistent() {
        let mut q = Quarantine::new(24);
        assert!(q.release("nonexistent").is_err());
    }

    #[test]
    fn test_list_quarantined() {
        let mut q = Quarantine::new(24);
        q.add("mod1");
        q.add("mod2");
        assert_eq!(q.list().len(), 2);
    }
}
