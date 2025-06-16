mod utils;

#[test]
fn create_project_human_snapshot() {
    let db_path = utils::setup_test_db("create_project", 0);
    // Run the CLI application
    let stdout = utils::run_command(
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

    // Define the snapshot name
    let snapshot_name = "create_project_human_snapshot";

    // Check if the snapshot exists
    if std::path::Path::new(&utils::get_snapshot_path(snapshot_name)).exists() {
        // Compare with the existing snapshot
        let expected_output = utils::read_snapshot(snapshot_name);
        assert_eq!(
            stdout, expected_output,
            "Output does not match the snapshot"
        );
    } else {
        // Write the snapshot if it doesn't exist
        utils::write_snapshot(snapshot_name, &stdout);
        println!("Snapshot created: {}", snapshot_name);
    }

    // Clean up the test database
    utils::reset_sqlite_db(&db_path);
}

#[test]
fn create_project_json_snapshot() {
    let db_path = utils::setup_test_db("create_project_json", 1);
    let stdout = utils::run_command(
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
    let snapshot_name = "create_project_json_snapshot";
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
fn create_project_yaml_snapshot() {
    let db_path = utils::setup_test_db("create_project_yaml", 2);
    let stdout = utils::run_command(
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
    let snapshot_name = "create_project_yaml_snapshot";
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
fn create_project_toml_snapshot() {
    let db_path = utils::setup_test_db("create_project_toml", 3);
    let stdout = utils::run_command(
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
    let snapshot_name = "create_project_toml_snapshot";
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
