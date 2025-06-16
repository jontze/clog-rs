use clap::{Parser, Subcommand};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, IntoActiveModel, TransactionTrait, TryIntoModel, prelude::*,
};
use serde::Serialize;
use tabled::Tabled;

use super::CommandExecutorTrait;
use crate::{
    Context,
    commands::{
        OutputFormat,
        command_output::{CommandOutput, NoTable},
    },
    entity::{projects, tasks, time_entries},
};

const TASK_STATUS_PENDING: &str = "pending";
const TASK_STATUS_IN_PROGRESS: &str = "in_progress";
const TASK_STATUS_COMPLETED: &str = "completed";
const TASK_STATUS_CANCELLED: &str = "cancelled";
const _TASK_STATES: [&str; 4] = [
    TASK_STATUS_PENDING,
    TASK_STATUS_IN_PROGRESS,
    TASK_STATUS_COMPLETED,
    TASK_STATUS_CANCELLED,
];

#[derive(Subcommand)]
pub(super) enum TaskCommand {
    /// Create a new task
    Create(CreateTaskCommand),
    /// Remove an existing task
    Remove(RemoveTaskCommand),
    /// List all tasks
    List(ListTasksCommand),
    /// Edit an existing task
    Edit(EditTaskCommand),
    /// Start a new task
    Start(StartTaskCommand),
    /// Stop an existing task
    Stop(StopTaskCommand),
}

#[derive(Parser)]
pub(super) struct CreateTaskCommand {
    /// Name of the task to create
    #[clap(short, long)]
    name: String,
    /// Description of the task
    #[clap(short, long)]
    description: Option<String>,
    /// Project name to associate with the task
    #[clap(short, long)]
    project_name: String,
    /// Immediately start the task after creation
    #[clap(short, long)]
    start: bool,
}

#[derive(Parser)]
pub(super) struct RemoveTaskCommand {
    /// Name of the task to remove
    #[clap(short, long)]
    name: String,
}

#[derive(Parser)]
pub(super) struct ListTasksCommand {
    /// Filter tasks by project name
    #[clap(short, long)]
    project_name: String,
}

#[derive(Parser)]
pub(super) struct StartTaskCommand {
    /// Name of the task to start
    #[clap(short, long)]
    name: String,
}

#[derive(Parser)]
pub(super) struct StopTaskCommand {
    /// Name of the task to stop
    #[clap(short, long)]
    name: String,
    /// Is the task finished?
    #[clap(short, long)]
    finished: bool,
    /// Is the task cancelled?
    #[clap(short, long)]
    cancelled: bool,
}

#[derive(Parser)]
pub(super) struct EditTaskCommand {
    /// Name of the task to edit
    #[clap(short, long)]
    name: String,
    /// New name for the task
    #[clap(short = 'N', long)]
    new_name: Option<String>,
    /// New description for the task
    #[clap(short = 'd', long)]
    new_description: Option<String>,
}

impl CommandExecutorTrait for TaskCommand {
    async fn execute(&self, ctx: Context, output_format: OutputFormat) -> miette::Result<()> {
        match self {
            TaskCommand::Create(cmd) => {
                create(
                    &ctx,
                    &cmd.name,
                    cmd.description.as_deref(),
                    &cmd.project_name,
                    cmd.start,
                    output_format,
                )
                .await
            }
            TaskCommand::Remove(cmd) => remove(&ctx, &cmd.name, output_format).await,
            TaskCommand::List(cmd) => list(&ctx, &cmd.project_name, output_format).await,
            TaskCommand::Start(cmd) => start(&ctx, &cmd.name, output_format).await,
            TaskCommand::Stop(cmd) => {
                stop(&ctx, &cmd.name, cmd.finished, cmd.cancelled, output_format).await
            }
            TaskCommand::Edit(cmd) => {
                edit(
                    &ctx,
                    output_format,
                    &cmd.name,
                    cmd.new_name.as_deref(),
                    cmd.new_description.as_deref(),
                )
                .await
            }
        }
    }
}

async fn create(
    ctx: &Context,
    name: &str,
    description: Option<&str>,
    project_name: &str,
    start: bool,
    output_format: OutputFormat,
) -> miette::Result<()> {
    let txn = ctx
        .db
        .begin()
        .await
        .map_err(|e| miette::miette!("Failed to begin transaction: {}", e))?;

    let project = projects::Entity::find()
        .filter(projects::Column::Name.eq(project_name))
        .one(&txn)
        .await
        .map_err(|e| miette::miette!("Failed to find project: {}", e))?
        .ok_or_else(|| miette::miette!("Project not found"))?;

    let status = if start {
        TASK_STATUS_IN_PROGRESS.to_string()
    } else {
        TASK_STATUS_PENDING.to_string()
    };
    let task = tasks::ActiveModel {
        name: Set(name.to_string()),
        description: Set(description.map(|d| d.to_string())),
        project_id: Set(project.id),
        status: Set(status),
        ..Default::default()
    }
    .save(&txn)
    .await
    .map_err(|e| miette::miette!("Failed to create task: {}", e))?
    .try_into_model()
    .expect("Failed to convert ActiveModel to Model");

    // Create a new time entry if the task is started
    if start {
        time_entries::ActiveModel {
            task_id: Set(task.id),
            ..Default::default()
        }
        .save(&txn)
        .await
        .map_err(|e| miette::miette!("Failed to create time entry: {}", e))?;
    }

    txn.commit()
        .await
        .map_err(|e| miette::miette!("Failed to commit transaction: {}", e))?;

    CommandOutput::<Vec<NoTable>, NoTable>::builder()
        .with_mode(output_format)
        .with_prefix_message(format!(
            "Task '{}' created successfully with status '{}'",
            task.name, task.status
        ))
        .build()
        .print();

    Ok(())
}

async fn remove(ctx: &Context, name: &str, output_format: OutputFormat) -> miette::Result<()> {
    let task = tasks::Entity::find()
        .filter(tasks::Column::Name.eq(name))
        .one(&ctx.db)
        .await
        .map_err(|e| miette::miette!("Failed to find task: {}", e))?
        .ok_or_else(|| miette::miette!("Task not found"))?;

    task.delete(&ctx.db)
        .await
        .map_err(|e| miette::miette!("Failed to remove task: {}", e))?;

    CommandOutput::<Vec<NoTable>, NoTable>::builder()
        .with_mode(output_format)
        .with_prefix_message("Task was removed successfully".to_string())
        .build()
        .print();
    Ok(())
}

async fn start(ctx: &Context, name: &str, _output_format: OutputFormat) -> miette::Result<()> {
    let txn = ctx
        .db
        .begin()
        .await
        .map_err(|e| miette::miette!("Failed to begin transaction: {}", e))?;

    let task = tasks::Entity::find()
        .filter(tasks::Column::Name.eq(name))
        .one(&txn)
        .await
        .map_err(|e| miette::miette!("Failed to find task: {}", e))?
        .ok_or_else(|| miette::miette!("Task not found"))?;

    if [TASK_STATUS_CANCELLED, TASK_STATUS_COMPLETED].contains(&task.status.as_str()) {
        return Err(miette::miette!(
            "Task is not in a state that can be started"
        ));
    }

    // Update task status to 'in_progress'
    let mut active_model: tasks::ActiveModel = task.clone().into();
    active_model.status = Set(TASK_STATUS_IN_PROGRESS.to_string());
    active_model
        .update(&txn)
        .await
        .map_err(|e| miette::miette!("Failed to start task: {}", e))?;

    // If there is an existing time entry for this task, that is not completed we should not create a new one
    let existing_unfinished_time_entry = time_entries::Entity::find()
        .filter(time_entries::Column::TaskId.eq(task.id))
        .filter(time_entries::Column::EndTime.is_null())
        .one(&txn)
        .await
        .map_err(|e| miette::miette!("Failed to find time entry: {}", e))?;
    if existing_unfinished_time_entry.is_some() {
        return Err(miette::miette!("Task already has an unfinished time entry"));
    }

    // Create a new time entry for the task
    time_entries::ActiveModel {
        task_id: Set(task.id),
        ..Default::default()
    }
    .save(&txn)
    .await
    .map_err(|e| miette::miette!("Failed to create time entry: {}", e))?;

    txn.commit()
        .await
        .map_err(|e| miette::miette!("Failed to commit transaction: {}", e))?;

    CommandOutput::<Vec<NoTable>, NoTable>::builder()
        .with_mode(_output_format)
        .with_prefix_message(format!("Started working on task '{}'", task.name))
        .build()
        .print();
    Ok(())
}

async fn stop(
    ctx: &Context,
    name: &str,
    finished: bool,
    cancelled: bool,
    _output_format: OutputFormat,
) -> miette::Result<()> {
    let txn = ctx
        .db
        .begin()
        .await
        .map_err(|e| miette::miette!("Failed to begin transaction: {}", e))?;

    let task = tasks::Entity::find()
        .filter(tasks::Column::Name.eq(name))
        .one(&txn)
        .await
        .map_err(|e| miette::miette!("Failed to find task: {}", e))?
        .ok_or_else(|| miette::miette!("Task not found"))?;

    let new_task_status = if finished {
        TASK_STATUS_COMPLETED.to_string()
    } else if cancelled {
        TASK_STATUS_CANCELLED.to_string()
    } else {
        task.status.clone()
    };

    // Check if the task is in progress
    if task.status != TASK_STATUS_IN_PROGRESS {
        return Err(miette::miette!(
            "Task is not in progress and cannot be stopped"
        ));
    }

    // Check if there is an active time entry for the task
    for time_entry in time_entries::Entity::find()
        .filter(time_entries::Column::TaskId.eq(task.id))
        .filter(time_entries::Column::EndTime.is_null())
        .all(&txn)
        .await
        .map_err(|e| miette::miette!("Failed to find time entry: {}", e))?
    {
        let (duration, end_time) = calc_duration_to_now(time_entry.start_time);
        // If there is an active time entry, we need to stop it before stopping the task
        let mut time_entry_active_model: time_entries::ActiveModel = time_entry.into();
        // Update the end time of the time entry to now
        time_entry_active_model.end_time = Set(Some(end_time));
        time_entry_active_model.duration = Set(duration);
        time_entry_active_model
            .update(&txn)
            .await
            .map_err(|e| miette::miette!("Failed to stop time entry: {}", e))?;
    }

    // Update task status to potential new status
    let mut active_model: tasks::ActiveModel = task.into();
    active_model.status = Set(new_task_status);
    let task = active_model
        .update(&txn)
        .await
        .map_err(|e| miette::miette!("Failed to stop task: {}", e))?
        .try_into_model()
        .map_err(|e| miette::miette!("Failed to convert ActiveModel to Model: {}", e))?;

    txn.commit()
        .await
        .map_err(|e| miette::miette!("Failed to commit transaction: {}", e))?;

    CommandOutput::<Vec<NoTable>, NoTable>::builder()
        .with_mode(_output_format)
        .with_prefix_message(format!(
            "Stopped working on task '{}'. New status: '{}'",
            task.name, task.status
        ))
        .build()
        .print();
    Ok(())
}

fn calc_duration_to_now(start_time: DateTimeWithTimeZone) -> (i32, DateTimeWithTimeZone) {
    let offset = start_time.offset();
    let start_time = start_time.to_utc();
    let end_time_utc = chrono::Utc::now();
    (
        (end_time_utc - start_time).num_seconds() as i32,
        end_time_utc.with_timezone(offset),
    )
}

async fn list(
    ctx: &Context,
    project_name: &str,
    output_format: OutputFormat,
) -> miette::Result<()> {
    let project = projects::Entity::find()
        .filter(projects::Column::Name.eq(project_name))
        .one(&ctx.db)
        .await
        .map_err(|e| miette::miette!("Failed to find project: {}", e))?
        .ok_or_else(|| miette::miette!("Project not found"))?;

    let project_tasks = project
        .find_related(tasks::Entity)
        .all(&ctx.db)
        .await
        .map_err(|e| miette::miette!("Failed to find tasks for project: {}", e))?;

    let mut tasks_table: Vec<TaskTable> = vec![];

    for task in &project_tasks {
        let amount_of_time_entries = time_entries::Entity::find()
            .filter(time_entries::Column::TaskId.eq(task.id))
            .count(&ctx.db)
            .await
            .map_err(|e| miette::miette!("Failed to count time entries: {}", e))?;
        tasks_table.push(TaskTable {
            id: task.id,
            name: task.name.clone(),
            description: task.description.clone().unwrap_or_default(),
            status: task.status.clone(),
            time_entries: amount_of_time_entries,
        });
    }

    CommandOutput::<Vec<TaskTable>, TaskTable>::builder()
        .with_table_rows(tasks_table)
        .with_mode(output_format)
        .with_prefix_message(format!("Tasks for project '{}':", project.name))
        .build()
        .print();
    Ok(())
}

#[derive(Tabled, Serialize, Clone)]
struct TaskTable {
    id: i32,
    name: String,
    description: String,
    status: String,
    time_entries: u64,
}

async fn edit(
    ctx: &Context,
    output_format: OutputFormat,
    name: &str,
    new_name: Option<&str>,
    new_description: Option<&str>,
) -> miette::Result<()> {
    let mut task = tasks::Entity::find()
        .filter(tasks::Column::Name.eq(name))
        .one(&ctx.db)
        .await
        .map_err(|e| miette::miette!("Failed to find task: {}", e))?
        .ok_or_else(|| miette::miette!("Task not found"))?
        .into_active_model();

    if let Some(new_name) = new_name {
        task.name = Set(new_name.to_string());
    }
    if let Some(new_description) = new_description {
        task.description = Set(Some(new_description.to_string()));
    }

    let task = task
        .update(&ctx.db)
        .await
        .map_err(|e| miette::miette!("Failed to update task: {}", e))?;

    CommandOutput::<Vec<NoTable>, NoTable>::builder()
        .with_mode(output_format)
        .with_prefix_message(format!("Task '{}' updated successfully", task.name))
        .build()
        .print();
    Ok(())
}
