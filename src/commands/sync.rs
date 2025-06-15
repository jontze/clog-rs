use bytes::Bytes;
use clap::Subcommand;
use object_store::azure::MicrosoftAzureBuilder;

use super::CommandExecutorTrait;
use crate::{
    Context,
    commands::{
        OutputFormat,
        command_output::{CommandOutput, NoTable},
    },
};

const REMOTE_FILE_NAME: &str = "clog-db.sqlite";

#[derive(Subcommand)]
pub(super) enum SyncCommand {
    /// Push local changes to the remote storage
    Push,
    /// Pull changes from the remote storage
    Pull,
}

impl CommandExecutorTrait for SyncCommand {
    async fn execute(&self, ctx: Context, output_format: OutputFormat) -> miette::Result<()> {
        let access_key = std::env::var("CLOG_STORAGE_ACCESS_KEY").map_err(|_| {
            miette::miette!(
                "CLOG_STORAGE_ACCESS_KEY environment variable is not set. Please set it to use sync commands."
            )
        })?;
        let account_url = std::env::var("CLOG_STORAGE_ACCOUNT_NAME").map_err(|_| {
            miette::miette!(
                "CLOG_STORAGE_ACCOUNT_NAME environment variable is not set. Please set it to use sync commands."
            )
        })?;
        let container_name = std::env::var("CLOG_CONTAINER_NAME").unwrap_or("sync".to_string());
        let s3_client = setup_s3_client(&access_key, &container_name, &account_url)
            .map_err(|e| miette::miette!("Failed to set up S3 client: {}", e))?;

        match self {
            SyncCommand::Push => push(ctx, output_format, &s3_client).await,
            SyncCommand::Pull => pull(ctx, output_format, &s3_client).await,
        }
    }
}

async fn push(
    _ctx: Context,
    output_format: OutputFormat,
    s3_client: &dyn object_store::ObjectStore,
) -> miette::Result<()> {
    let db_path =
        crate::db::db_path().map_err(|e| miette::miette!("Failed to get database path: {}", e))?;

    let file_bytes = tokio::fs::read(&db_path)
        .await
        .map_err(|e| {
            miette::miette!(
                "Failed to read database file at {}: {}",
                db_path.display(),
                e
            )
        })
        .map(Bytes::from)?;

    let remote_file_path = object_store::path::Path::from(REMOTE_FILE_NAME);
    let put_payload = object_store::PutPayload::from_bytes(file_bytes);

    s3_client
        .put(&remote_file_path, put_payload)
        .await
        .map_err(|e| {
            miette::miette!(
                "Failed to upload file to remote storage at {}: {}",
                remote_file_path,
                e
            )
        })?;

    CommandOutput::<Vec<NoTable>, NoTable>::builder()
        .with_mode(output_format)
        .with_prefix_message("Local changes pushed to remote storage successfully.".to_string())
        .build()
        .print();
    Ok(())
}

async fn pull(
    _ctx: Context,
    output_format: OutputFormat,
    s3_client: &dyn object_store::ObjectStore,
) -> miette::Result<()> {
    let remote_file_path = object_store::path::Path::from(REMOTE_FILE_NAME);
    let get_result = s3_client
        .get(&remote_file_path)
        .await
        .map_err(|e| miette::miette!("Failed to download file from remote storage: {}", e))?;
    let file_bytes = get_result
        .bytes()
        .await
        .map_err(|e| miette::miette!("Failed to read bytes from downloaded file: {}", e))?;

    let db_path =
        crate::db::db_path().map_err(|e| miette::miette!("Failed to get database path: {}", e))?;

    tokio::fs::write(&db_path, file_bytes).await.map_err(|e| {
        miette::miette!(
            "Failed to write database file at {}: {}",
            db_path.display(),
            e
        )
    })?;

    CommandOutput::<Vec<NoTable>, NoTable>::builder()
        .with_mode(output_format)
        .with_prefix_message("Changes pulled from remote storage successfully.".to_string())
        .build()
        .print();
    Ok(())
}

fn setup_s3_client(
    access_key: &str,
    container_name: &str,
    account_url: &str,
) -> miette::Result<Box<dyn object_store::ObjectStore>> {
    let azure_builder = MicrosoftAzureBuilder::new()
        .with_account(account_url)
        .with_access_key(access_key)
        .with_container_name(container_name);

    let azure = azure_builder
        .build()
        .expect("Error creating Azure Blob client");
    Ok(Box::new(azure))
}
