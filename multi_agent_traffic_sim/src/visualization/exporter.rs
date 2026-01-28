use super::timeline::Timeline;
use std::fs::File;
use std::io::Write;

pub fn export_timeline_to_json(timeline: &Timeline, filepath: &str) -> std::io::Result<()> {
    let json = serde_json::to_string_pretty(timeline)?;
    let mut file = File::create(filepath)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}
