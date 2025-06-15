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
    /// Whether to gennerate the report for today
    #[clap(short, long)]
    today: bool,
    /// The date to generate the report for (format: YYYY-MM-DD)
    #[clap(short, long)]
    date: Option<String>,
}

#[derive(Parser)]
pub(super) struct ReportTaskCommand {
    /// The name of the project the task belongs to
    #[clap(short, long)]
    project_name: String,
    /// Whether to gennerate the report for today
    #[clap(short, long)]
    today: bool,
    /// The date to generate the report for (format: YYYY-MM-DD)
    #[clap(short, long)]
    date: Option<String>,
}

impl CommandExecutorTrait for ReportCommand {
    async fn execute(&self, ctx: Context, output_format: OutputFormat) -> miette::Result<()> {
        match self {
            ReportCommand::Project(cmd) => {
                report_project(ctx, output_format, cmd.today, &cmd.date).await
            }
            ReportCommand::Task(cmd) => {
                report_task(ctx, output_format, &cmd.project_name, cmd.today, &cmd.date).await
            }
        }
    }
}

async fn report_project(
    ctx: Context,
    output_format: OutputFormat,
    check_today: bool,
    date_string: &Option<String>,
) -> miette::Result<()> {
    //Fetch all projects from the database
    let all_projects = projects::Entity::find()
        .all(&ctx.db)
        .await
        .map_err(|e| miette::miette!("Failed to fetch projects: {}", e))?;

    let mut project_table: Vec<ReportProjectTable> = vec![];
    // Iterate through each project and fetch related tasks and time entries
    for project in all_projects {
        let mut project_query = project
            .find_related(tasks::Entity)
            .find_with_related(time_entries::Entity);
        // If the `check_today` flag is set, filter tasks by today's time entries
        if check_today {
            let today = chrono::Utc::now().date_naive();
            let tomorrow = today
                .succ_opt()
                .ok_or_else(|| miette::miette!("Failed to calculate tomorrow's date"))?;
            project_query = project_query.filter(
                time_entries::Column::StartTime
                    .gt(today)
                    .and(time_entries::Column::StartTime.lt(tomorrow)),
            );
        } else if let Some(date_str) = date_string {
            // If a specific date is provided, filter tasks by that date and exclude the following days
            let selected_day = chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
                .map_err(|e| miette::miette!("Invalid date format: {}", e))?;
            let next_day = selected_day.succ_opt().ok_or_else(|| {
                miette::miette!("Failed to calculate the next day for the provided date")
            })?;
            project_query = project_query.filter(
                time_entries::Column::StartTime
                    .gt(selected_day)
                    .and(time_entries::Column::StartTime.lt(next_day)),
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

    CommandOutput::builder()
        .with_table_rows(project_table)
        .with_prefix_message(format!("Time spent per project ({})", {
            if check_today {
                "today"
            } else if let Some(date_str) = date_string {
                date_str
            } else {
                "all time"
            }
        }))
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
    ctx: Context,
    output_format: OutputFormat,
    project_name: &str,
    check_today: bool,
    date_string: &Option<String>,
) -> miette::Result<()> {
    // Fetch the project tasks by project name
    let project = projects::Entity::find()
        .filter(projects::Column::Name.eq(project_name))
        .one(&ctx.db)
        .await
        .map_err(|e| miette::miette!("Failed to fetch tasks for project: {}", e))?
        .ok_or(miette::miette!(
            "Project with name '{}' not found",
            project_name
        ))?;
    // Start the query to find all tasks with related time entries
    let mut tasks_with_time_entries = project
        .find_related(tasks::Entity)
        .find_with_related(time_entries::Entity);
    // If the `check_today` flag is set, filter tasks by today's time entries
    if check_today {
        let today_date = chrono::Utc::now().date_naive();
        let tomorrow_date = today_date
            .succ_opt()
            .ok_or_else(|| miette::miette!("Failed to calculate tomorrow's date"))?;
        tasks_with_time_entries = tasks_with_time_entries.filter(
            time_entries::Column::StartTime
                .gt(today_date)
                .and(time_entries::Column::StartTime.lt(tomorrow_date)),
        );
    } else if let Some(date_str) = date_string {
        // If a specific date is provided, filter tasks by that date and exclude the following days
        let selected_day = chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
            .map_err(|e| miette::miette!("Invalid date format: {}", e))?;
        let next_day = selected_day.succ_opt().ok_or_else(|| {
            miette::miette!("Failed to calculate the next day for the provided date")
        })?;
        tasks_with_time_entries = tasks_with_time_entries.filter(
            time_entries::Column::StartTime
                .gt(selected_day)
                .and(time_entries::Column::StartTime.lt(next_day)),
        );
    }
    let tasks_with_time_entries = tasks_with_time_entries.all(&ctx.db).await.map_err(|e| {
        miette::miette!(
            "Failed to fetch tasks for project '{}': {}",
            project_name,
            e
        )
    })?;

    let mut task_table: Vec<ReportTaskTable> = vec![];
    for (task, time_entry_items) in tasks_with_time_entries {
        let time_spent_secs = time_entry_items
            .iter()
            .map(|entry| entry.duration)
            .sum::<i32>();
        let time_entries_count = time_entry_items.len();
        let open_time_entries = time_entry_items
            .iter()
            .any(|entry| entry.end_time.is_none());
        let spend_mins = time_spent_secs as f32 / 60.00_f32;
        let spend_hours = spend_mins / 60.00_f32;
        task_table.push(ReportTaskTable {
            id: task.id,
            name: task.name,
            description: task.description.unwrap_or("".to_string()),
            time_spent_min: format!("{spend_mins:.2} mins"),
            time_spent_hours: format!("{spend_hours:.2} hours"),
            time_entries: time_entries_count,
            open_time_entries,
        });
    }

    CommandOutput::builder()
        .with_table_rows(task_table)
        .with_prefix_message(format!(
            "Time spent per task in project '{project_name}' ({})",
            {
                if check_today {
                    "today"
                } else if let Some(date_str) = date_string {
                    date_str
                } else {
                    "all time"
                }
            }
        ))
        .with_mode(output_format)
        .build()
        .print();
    Ok(())
}

#[derive(Tabled, Serialize, Clone)]
struct ReportTaskTable {
    id: i32,
    name: String,
    description: String,
    time_spent_min: String,
    time_spent_hours: String,
    time_entries: usize,
    open_time_entries: bool,
}
