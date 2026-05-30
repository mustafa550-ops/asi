use std::thread;
use adler_asi_lib::agents::ApprovalLevel;

#[test]
fn approval_level_values_are_distinct() {
    let levels = vec![
        ApprovalLevel::Observer,
        ApprovalLevel::SemiAutonomous,
        ApprovalLevel::Strategic,
    ];
    assert_eq!(levels.len(), 3);
    assert_ne!(levels[0], levels[1]);
    assert_ne!(levels[1], levels[2]);
}

#[test]
fn approval_level_debug_representation() {
    let observer = format!("{:?}", ApprovalLevel::Observer);
    assert!(!observer.is_empty());
}

#[test]
fn concurrent_string_operations() {
    let handles: Vec<_> = (0..10).map(|i| {
        thread::spawn(move || format!("agent_{}", i))
    }).collect();

    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    assert_eq!(results.len(), 10);
    assert!(results.iter().all(|r| r.starts_with("agent_")));
}
