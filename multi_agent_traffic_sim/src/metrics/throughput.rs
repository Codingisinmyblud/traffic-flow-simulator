pub fn calculate_throughput(completed_tasks: usize, total_time: u64) -> f64 {
    if total_time == 0 {
        return 0.0;
    }
    completed_tasks as f64 / total_time as f64
}
