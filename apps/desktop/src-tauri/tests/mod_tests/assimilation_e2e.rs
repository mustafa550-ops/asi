use adler_asi_lib::assimilation::registry::ModuleRegistry;
use super::helpers::create_test_db_arc;

fn create_registry() -> ModuleRegistry {
    ModuleRegistry::new(create_test_db_arc())
}

#[test]
fn assimilation_module_register() {
    let registry = create_registry();
    registry.register("test_module", "/tmp/test", &["dep1".into()]).unwrap();
    let modules = registry.list_all();
    assert!(!modules.is_empty());
    assert!(modules.iter().any(|m| m.name == "test_module"));
}

#[test]
fn assimilation_module_remove() {
    let registry = create_registry();
    registry.register("removable", "/tmp/rem", &[] as &[String]).unwrap();
    registry.remove("removable").unwrap();
    let modules = registry.list_all();
    assert!(!modules.iter().any(|m| m.name == "removable"));
}

#[test]
fn assimilation_module_list_empty() {
    let registry = create_registry();
    let modules = registry.list_all();
    assert!(modules.is_empty());
}
