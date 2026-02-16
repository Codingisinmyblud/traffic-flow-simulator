use multi_agent_traffic_sim::coordination::deadlock::WaitForGraph;

#[test]
fn test_cyclic_deadlock() {
    let mut wfg = WaitForGraph::new();
    wfg.add_wait(1, 2);
    wfg.add_wait(2, 3);
    wfg.add_wait(3, 1);

    let deadlock = wfg.detect_deadlock();
    assert!(deadlock.is_some());
}

#[test]
fn test_no_deadlock() {
    let mut wfg = WaitForGraph::new();
    wfg.add_wait(1, 2);
    wfg.add_wait(2, 3);
    wfg.add_wait(4, 5);

    let deadlock = wfg.detect_deadlock();
    assert!(deadlock.is_none());
}
