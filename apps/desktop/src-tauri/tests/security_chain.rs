use adler_asi_lib::security::quarantine::Quarantine;
use adler_asi_lib::security::signing::ModuleSigner;

#[test]
fn security_quarantine_add_and_check() {
    let mut q = Quarantine::new(24);
    q.add("mod_a");
    assert!(q.is_quarantined("mod_a"));
    assert!(!q.is_quarantined("other"));
}

#[test]
fn security_quarantine_release() {
    let mut q = Quarantine::new(24);
    q.add("mod_b");
    assert!(q.release("mod_b").is_ok());
    assert!(!q.is_quarantined("mod_b"));
}

#[test]
fn security_quarantine_release_nonexistent() {
    let mut q = Quarantine::new(24);
    assert!(q.release("nonexistent").is_err());
}

#[test]
fn security_quarantine_list() {
    let mut q = Quarantine::new(24);
    q.add("mod1");
    q.add("mod2");
    assert_eq!(q.list().len(), 2);
}

#[test]
fn security_module_signer_creation() {
    let signer = ModuleSigner::new();
    assert!(signer.is_ok(), "ModuleSigner should be creatable");
}
