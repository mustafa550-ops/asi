use adler_asi_lib::skill::registry::SkillRegistry;
use adler_asi_lib::skill::SkillStep;
use super::helpers::create_test_db_arc;

fn create_registry() -> SkillRegistry {
    SkillRegistry::new(create_test_db_arc())
}

#[test]
fn skill_register_and_list() {
    let registry = create_registry();
    let steps = vec![SkillStep { order: 1, description: "test adimi".into() }];
    let id = registry.register(
        "test_skill", "test aciklamasi",
        &["tetikleyici".into()], "auto",
        &steps, None, &[],
    ).unwrap();
    assert!(id > 0);
    let skills = registry.list().unwrap();
    assert!(!skills.is_empty());
    assert!(skills.iter().any(|s| s.id == id));
}

#[test]
fn skill_activate_deactivate() {
    let registry = create_registry();
    let steps = vec![SkillStep { order: 1, description: "adim".into() }];
    registry.register("toggle_test", "test", &["x".into()], "auto", &steps, None, &[]).unwrap();
    registry.deactivate("toggle_test").unwrap();
    let skill = registry.get_by_name("toggle_test").unwrap().unwrap();
    assert!(!skill.active);
    registry.activate("toggle_test").unwrap();
    let skill = registry.get_by_name("toggle_test").unwrap().unwrap();
    assert!(skill.active);
}

#[test]
fn skill_remove() {
    let registry = create_registry();
    let steps = vec![SkillStep { order: 1, description: "adim".into() }];
    registry.register("silinecek", "test", &["sil".into()], "auto", &steps, None, &[]).unwrap();
    registry.remove("silinecek").unwrap();
    let skill = registry.get_by_name("silinecek").unwrap();
    assert!(skill.is_none());
}

#[test]
fn skill_register_duplicate_name_upserts() {
    let registry = create_registry();
    let steps = vec![SkillStep { order: 1, description: "adim".into() }];
    registry.register("dup", "first", &["a".into()], "auto", &steps, None, &[]).unwrap();
    let result = registry.register("dup", "second", &["b".into()], "auto", &steps, None, &[]);
    assert!(result.is_ok());
}

#[test]
fn skill_get_nonexistent_returns_none() {
    let registry = create_registry();
    let result = registry.get_by_name("yok_boyle_bir_skill").unwrap();
    assert!(result.is_none());
}

#[test]
fn skill_increment_run_count() {
    let registry = create_registry();
    let steps = vec![SkillStep { order: 1, description: "adim".into() }];
    registry.register("counter_test", "test", &["c".into()], "auto", &steps, None, &[]).unwrap();
    for _ in 0..3 { registry.increment_run_count("counter_test").unwrap(); }
    let skill = registry.get_by_name("counter_test").unwrap().unwrap();
    assert_eq!(skill.run_count, 3);
}
