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
            "demo_project",
            "-d",
            "A demo project",
        ],
        db_path,
    );
    common::run_command(
        &[
            "run",
            "--",
            "project",
            "create",
            "-n",
            "demo_project_2",
            "-d",
            "Another demo project",
        ],
        db_path,
    );
}

#[test]
fn list_projects_human_snapshot() {
    let db_path = common::setup_test_db("list_projects_human", 0);
    setup(&db_path);
    let stdout = common::run_command(&["run", "--", "project", "list"], &db_path);
    common::assert_snapshot("list_projects_human_snapshot", &stdout);
    common::reset_sqlite_db(&db_path);
}

#[test]
fn list_projects_json_snapshot() {
    let db_path = common::setup_test_db("list_projects_json", 1);
    setup(&db_path);
    let stdout = common::run_command(&["run", "--", "-o", "json", "project", "list"], &db_path);
    common::assert_snapshot("list_projects_json_snapshot", &stdout);
    common::reset_sqlite_db(&db_path);
}

#[test]
fn list_projects_yaml_snapshot() {
    let db_path = common::setup_test_db("list_projects_yaml", 2);
    setup(&db_path);
    let stdout = common::run_command(&["run", "--", "-o", "yaml", "project", "list"], &db_path);
    common::assert_snapshot("list_projects_yaml_snapshot", &stdout);
    common::reset_sqlite_db(&db_path);
}

#[test]
fn list_projects_toml_snapshot() {
    let db_path = common::setup_test_db("list_projects_toml", 3);
    setup(&db_path);
    let stdout = common::run_command(&["run", "--", "-o", "toml", "project", "list"], &db_path);
    common::assert_snapshot("list_projects_toml_snapshot", &stdout);
    common::reset_sqlite_db(&db_path);
}
