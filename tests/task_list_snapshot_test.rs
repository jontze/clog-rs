use std::path::Path;

mod common;

fn setup(db_path: &Path) {
    common::run_command(
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
    common::run_command(
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
    common::run_command(
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
    let db_path = common::setup_test_db("list_tasks_human", 0);
    setup(&db_path);
    let stdout = common::run_command(&["run", "--", "task", "list", "-p", "demo"], &db_path);
    common::assert_snapshot("list_tasks_human_snapshot", &stdout);
    common::reset_sqlite_db(&db_path);
}

#[test]
fn list_tasks_json_snapshot() {
    let db_path = common::setup_test_db("list_tasks_json", 1);
    setup(&db_path);
    let stdout = common::run_command(
        &["run", "--", "-o", "json", "task", "list", "-p", "demo"],
        &db_path,
    );
    common::assert_snapshot("list_tasks_json_snapshot", &stdout);
    common::reset_sqlite_db(&db_path);
}

#[test]
fn list_tasks_yaml_snapshot() {
    let db_path = common::setup_test_db("list_tasks_yaml", 2);
    setup(&db_path);
    let stdout = common::run_command(
        &["run", "--", "-o", "yaml", "task", "list", "-p", "demo"],
        &db_path,
    );
    common::assert_snapshot("list_tasks_yaml_snapshot", &stdout);
    common::reset_sqlite_db(&db_path);
}

#[test]
fn list_task_toml_snapshot() {
    let db_path = common::setup_test_db("list_tasks_toml", 3);
    setup(&db_path);
    let stdout = common::run_command(
        &["run", "--", "-o", "toml", "task", "list", "-p", "demo"],
        &db_path,
    );
    common::assert_snapshot("list_tasks_toml_snapshot", &stdout);
    common::reset_sqlite_db(&db_path);
}
