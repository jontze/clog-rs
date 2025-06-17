use std::path::Path;

mod common;

fn setup(db_path: &Path) {
    // Create projects
    let projects = [
        ("demo_project", "A demo project"),
        ("another_project", "Another project"),
    ];
    for (name, description) in projects.into_iter() {
        common::run_command(
            &[
                "run",
                "--",
                "project",
                "create",
                "-n",
                name,
                "-d",
                description,
            ],
            db_path,
        );
    }

    // Add tasks to projects
    let tasks = [
        ("demo_task", "A demo task", "demo_project"),
        ("another_task", "Another task", "another_project"),
        ("demo_task_2", "A second demo task", "demo_project"),
        (
            "another_task_2",
            "A second task for another project",
            "another_project",
        ),
    ];

    for (name, description, project) in tasks.into_iter() {
        common::run_command(
            &[
                "run",
                "--",
                "task",
                "create",
                "-n",
                name,
                "-d",
                description,
                "-p",
                project,
            ],
            db_path,
        );
    }
}

#[test]
fn report_projects_human_snapshot() {
    let db_path = common::setup_test_db("report_projects_human", 0);
    setup(&db_path);
    let stdout = common::run_command(&["run", "--", "report", "project"], &db_path);
    common::assert_snapshot("report_projects_human_snapshot", &stdout);
    common::reset_sqlite_db(&db_path);
}

#[test]
fn report_projects_json_snapshot() {
    let db_path = common::setup_test_db("report_projects_json", 1);
    setup(&db_path);
    let stdout = common::run_command(&["run", "--", "-o", "json", "report", "project"], &db_path);
    common::assert_snapshot("report_projects_json_snapshot", &stdout);
    common::reset_sqlite_db(&db_path);
}

#[test]
fn report_projects_yaml_snapshot() {
    let db_path = common::setup_test_db("report_projects_yaml", 2);
    setup(&db_path);
    let stdout = common::run_command(&["run", "--", "-o", "yaml", "report", "project"], &db_path);
    common::assert_snapshot("report_projects_yaml_snapshot", &stdout);
    common::reset_sqlite_db(&db_path);
}
