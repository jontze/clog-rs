mod common;

#[test]
fn create_project_human_snapshot() {
    let db_path = common::setup_test_db("create_project", 0);
    // Run the CLI application
    let stdout = common::run_command(
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
        &db_path,
    );

    common::assert_snapshot("create_project_human_snapshot", &stdout);
    // Clean up the test database
    common::reset_sqlite_db(&db_path);
}

#[test]
fn create_project_json_snapshot() {
    let db_path = common::setup_test_db("create_project_json", 1);
    let stdout = common::run_command(
        &[
            "run",
            "--",
            "-o",
            "json",
            "project",
            "create",
            "-n",
            "demo_project",
            "-d",
            "A demo project",
        ],
        &db_path,
    );
    common::assert_snapshot("create_project_json_snapshot", &stdout);
    common::reset_sqlite_db(&db_path);
}

#[test]
fn create_project_yaml_snapshot() {
    let db_path = common::setup_test_db("create_project_yaml", 2);
    let stdout = common::run_command(
        &[
            "run",
            "--",
            "-o",
            "yaml",
            "project",
            "create",
            "-n",
            "demo_project",
            "-d",
            "A demo project",
        ],
        &db_path,
    );
    common::assert_snapshot("create_project_yaml_snapshot", &stdout);
    common::reset_sqlite_db(&db_path);
}

#[test]
fn create_project_toml_snapshot() {
    let db_path = common::setup_test_db("create_project_toml", 3);
    let stdout = common::run_command(
        &[
            "run",
            "--",
            "-o",
            "toml",
            "project",
            "create",
            "-n",
            "demo_project",
            "-d",
            "A demo project",
        ],
        &db_path,
    );
    common::assert_snapshot("create_project_toml_snapshot", &stdout);
    common::reset_sqlite_db(&db_path);
}
