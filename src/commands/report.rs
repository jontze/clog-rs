use clap::{Parser, Subcommand};
use sea_orm::prelude::*;
use serde::Serialize;
use tabled::Tabled;

use crate::{
    Context,
    commands::{CommandExecutorTrait, OutputFormat, command_output::CommandOutput},
    entity::{projects, tasks, time_entries},
};

#[derive(Subcommand)]
pub(super) enum ReportCommand {
    /// Generate a report for the current project
    Project(ReportProjectCommand),
    /// Generate a report for a task in a project
    Task(ReportTaskCommand),
}

#[derive(Parser)]
pub(super) struct ReportProjectCommand {
    /// Whether to check today's time entries
    #[clap(short, long)]
    today: bool,
}

#[derive(Parser)]
pub(super) struct ReportTaskCommand {
    /// The name of the task to generate a report for
    #[clap(short, long)]
    task_name: String,
    /// The name of the project the task belongs to
    #[clap(short, long)]
    project_name: String,
}

impl CommandExecutorTrait for ReportCommand {
    async fn execute(&self, ctx: Context, output_format: OutputFormat) -> miette::Result<()> {
        match self {
            ReportCommand::Project(cmd) => report_project(ctx, output_format, cmd.today).await,
            ReportCommand::Task(cmd) => {
                report_task(ctx, output_format, &cmd.task_name, &cmd.project_name).await
            }
        }
    }
}

async fn report_project(
    ctx: Context,
    output_format: OutputFormat,
    check_today: bool,
) -> miette::Result<()> {
    let all_projects = projects::Entity::find()
        .all(&ctx.db)
        .await
        .map_err(|e| miette::miette!("Failed to fetch projects: {}", e))?;

    let mut project_table: Vec<ReportProjectTable> = vec![];
    for project in all_projects {
        let mut project_query = project
            .find_related(tasks::Entity)
            .find_with_related(time_entries::Entity);
        if check_today {
            // If the `check_today` flag is set, filter tasks by today's time entries
            project_query = project_query.filter(
                time_entries::Column::StartTime
                    .gt(chrono::Utc::now().date_naive().and_hms_opt(0, 0, 0)),
            );
        }
        let project_task = project_query
            .all(&ctx.db)
            .await
            .map_err(|e| miette::miette!("Failed to find tasks for project: {}", e))?;
        let amount_tasks = project_task.len();
        let mut time_spent_secs = 0;
        let mut time_entries_count = 0;
        let mut open_time_entries = false; // Placeholder for open time entries logic
        for (_, entries) in project_task {
            time_entries_count += entries.len();
            if entries.iter().any(|entry| entry.end_time.is_none()) {
                open_time_entries = true;
            }
            time_spent_secs += entries.iter().map(|entry| entry.duration).sum::<i32>();
        }
        let spend_mins = time_spent_secs as f32 / 60.00_f32;
        let spend_hours = spend_mins / 60.00_f32;
        project_table.push(ReportProjectTable {
            id: project.id,
            name: project.name,
            description: project.description.unwrap_or("".to_string()),
            time_spent_min: format!("{spend_mins:.2} mins"),
            time_spent_hours: format!("{spend_hours:.2} hours"),
            tasks: amount_tasks,
            time_entries: time_entries_count,
            open_time_entries,
        });
    }

    // Placeholder for project report logic

    CommandOutput::builder()
        .with_table_rows(project_table)
        .with_prefix_message("Time spent per project".to_string())
        .with_mode(output_format)
        .build()
        .print();
    Ok(())
}

#[derive(Tabled, Serialize, Clone)]
struct ReportProjectTable {
    id: i32,
    name: String,
    description: String,
    time_spent_min: String,
    time_spent_hours: String,
    tasks: usize,
    time_entries: usize,
    open_time_entries: bool,
}

async fn report_task(
    _ctx: Context,
    _output_format: OutputFormat,
    task_name: &str,
    project_name: &str,
) -> miette::Result<()> {
    // Placeholder for task report logic
    unimplemented!(
        "Generating report for task '{}' in project '{}'",
        task_name,
        project_name
    )
}
