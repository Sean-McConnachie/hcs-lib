use std::path;

pub fn make_blank_file(program_dir: &path::PathBuf, change_count: i64) {
    let path = program_dir
        .join("changes")
        .join(format!("{}.tmp", change_count));
    std::fs::write(path, "").unwrap();
}
