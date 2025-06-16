use std::path::{Path, PathBuf};

pub fn get_snapshot_path(name: &str) -> String {
    format!("tests/snapshots/{}.txt", name)
}

pub fn read_snapshot(name: &str) -> String {
    let path = get_snapshot_path(name);
    std::fs::read_to_string(path).expect("Failed to read snapshot file")
}

pub fn write_snapshot(name: &str, content: &str) {
    let path = get_snapshot_path(name);
    std::fs::write(path, content).expect("Failed to write snapshot file");
}

pub fn reset_sqlite_db(db_path: &Path) {
    if db_path.exists() {
        std::fs::remove_file(&db_path).expect("Failed to remove database file");
    }
    println!("SQLite database reset at: {}", db_path.display());
}

pub fn run_command(args: &[&str], db_path: &Path) -> String {
    let output = std::process::Command::new("cargo")
        .args(args)
        .env("CLOG_DATABASE_PATH", db_path)
        .output()
        .expect("Failed to execute command");

    String::from_utf8_lossy(&output.stdout).to_string()
}

pub fn setup_test_db(test_name: &str, index: usize) -> PathBuf {
    let db_path = format!("tests/tmp/{test_name}_{index}.sqlite");
    let db_path = std::path::Path::new(&db_path).to_path_buf();
    reset_sqlite_db(&db_path);
    std::fs::create_dir_all(db_path.clone().parent().unwrap())
        .expect("Failed to create database directory");
    std::fs::File::create(&db_path).expect("Failed to create database file");
    db_path
}
