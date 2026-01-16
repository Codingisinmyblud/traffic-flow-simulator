pub fn calculate_utilization(active_time: u64, total_time: u64) -> f64 {
    if total_time == 0 {
        return 0.0;
    }
    active_time as f64 / total_time as f64
}
