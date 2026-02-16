use multi_agent_traffic_sim::coordination::scheduler::Scheduler;

#[test]
fn test_schedule_path_success() {
    let mut scheduler = Scheduler::new();
    let path = vec![1, 2, 3];
    
    // First time should succeed
    let success = scheduler.schedule_path(&path, 0);
    assert!(success);
}

#[test]
fn test_schedule_path_conflict() {
    let mut scheduler = Scheduler::new();
    let path = vec![1, 2, 3];
    
    scheduler.schedule_path(&path, 0);
    // Second time with exact same path at same time should fail
    let success = scheduler.schedule_path(&path, 0);
    assert!(!success);
}
