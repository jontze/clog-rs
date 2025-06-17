use std::path::Path;

mod utils;

fn setup(db_path: &Path) {
    utils::run_command(
        &[
            "run",
            "--",
            "project",
            "create",
            "-n",
            "demo",
            "-d",
            "A demo project",
        ],
        db_path,
    );
    utils::run_command(
        &[
            "run",
            "--",
            "task",
            "create",
            "-n",
            "demo_task",
            "-d",
            "A demo task",
            "-p",
            "demo",
        ],
        db_path,
    );
    utils::run_command(
        &[
            "run",
            "--",
            "task",
            "create",
            "-n",
            "demo_task_2",
            "-d",
            "Another demo task",
            "-p",
            "demo",
        ],
        db_path,
    );
}

#[test]
fn list_tasks_human_snapshot() {
    let db_path = utils::setup_test_db("list_tasks_human", 0);
    setup(&db_path);
    let stdout = utils::run_command(&["run", "--", "task", "list", "-p", "demo"], &db_path);
    let snapshot_name = "list_tasks_human_snapshot";
    if std::path::Path::new(&utils::get_snapshot_path(snapshot_name)).exists() {
        let expected_output = utils::read_snapshot(snapshot_name);
        assert_eq!(
            stdout, expected_output,
            "Output does not match the snapshot"
        );
    } else {
        utils::write_snapshot(snapshot_name, &stdout);
        println!("Snapshot created: {}", snapshot_name);
    }
    utils::reset_sqlite_db(&db_path);
}

#[test]
fn list_tasks_json_snapshot() {
    let db_path = utils::setup_test_db("list_tasks_json", 1);
    setup(&db_path);
    let stdout = utils::run_command(
        &["run", "--", "-o", "json", "task", "list", "-p", "demo"],
        &db_path,
    );
    let snapshot_name = "list_tasks_json_snapshot";
    if std::path::Path::new(&utils::get_snapshot_path(snapshot_name)).exists() {
        let expected_output = utils::read_snapshot(snapshot_name);
        assert_eq!(
            stdout, expected_output,
            "Output does not match the snapshot"
        );
    } else {
        utils::write_snapshot(snapshot_name, &stdout);
        println!("Snapshot created: {}", snapshot_name);
    }
    utils::reset_sqlite_db(&db_path);
}

#[test]
fn list_tasks_yaml_snapshot() {
    let db_path = utils::setup_test_db("list_tasks_yaml", 2);
    setup(&db_path);
    let stdout = utils::run_command(
        &["run", "--", "-o", "yaml", "task", "list", "-p", "demo"],
        &db_path,
    );
    let snapshot_name = "list_tasks_yaml_snapshot";
    if std::path::Path::new(&utils::get_snapshot_path(snapshot_name)).exists() {
        let expected_output = utils::read_snapshot(snapshot_name);
        assert_eq!(
            stdout, expected_output,
            "Output does not match the snapshot"
        );
    } else {
        utils::write_snapshot(snapshot_name, &stdout);
        println!("Snapshot created: {}", snapshot_name);
    }
    utils::reset_sqlite_db(&db_path);
}

#[test]
fn list_task_toml_snapshot() {
    let db_path = utils::setup_test_db("list_tasks_toml", 3);
    setup(&db_path);
    let stdout = utils::run_command(
        &["run", "--", "-o", "toml", "task", "list", "-p", "demo"],
        &db_path,
    );
    let snapshot_name = "list_tasks_toml_snapshot";
    if std::path::Path::new(&utils::get_snapshot_path(snapshot_name)).exists() {
        let expected_output = utils::read_snapshot(snapshot_name);
        assert_eq!(
            stdout, expected_output,
            "Output does not match the snapshot"
        );
    } else {
        utils::write_snapshot(snapshot_name, &stdout);
        println!("Snapshot created: {}", snapshot_name);
    }
    utils::reset_sqlite_db(&db_path);
}
