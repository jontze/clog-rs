use serde::Serialize;
use tabled::{Table, Tabled, settings::Style};

use crate::commands::OutputFormat;

pub(super) struct CommandOutput<I, TOut>
where
    I: IntoIterator<Item = TOut> + Clone + Serialize,
    TOut: Tabled + Serialize + Clone,
{
    prefix_messages: Vec<String>,
    suffix_messages: Vec<String>,
    error_messages: Vec<String>,
    table_rows: Option<I>,
    mode: OutputFormat,
}

impl<I, TOut> CommandOutput<I, TOut>
where
    I: IntoIterator<Item = TOut> + Clone + Serialize,
    TOut: Tabled + Serialize + Clone,
{
    pub fn builder() -> CommandOutputBuilder<I, TOut> {
        CommandOutputBuilder {
            table_rows: None,
            prefix_messages: Vec::new(),
            suffix_messages: Vec::new(),
            error_messages: Vec::new(),
            mode: None,
        }
    }

    pub fn print(&self) {
        match self.mode {
            OutputFormat::Json => self.json(),
            OutputFormat::Yaml => self.yaml(),
            OutputFormat::Toml => self.toml(),
            OutputFormat::Human => self.human(),
        }
    }

    fn human(&self) {
        // Only print error message and then abort
        if !self.error_messages.is_empty() {
            eprintln!("Errors:");
            for error in &self.error_messages {
                eprintln!("{}", error);
            }
            return;
        }

        // Print prefix messages
        for message in &self.prefix_messages {
            println!("{}", message);
        }

        // Print the table if it exists
        if let Some(table_rows) = &self.table_rows {
            let mut table = Table::new(table_rows.clone());
            table.with(Style::modern_rounded());
            println!("{table}");
        }

        // Print suffix messages
        for message in &self.suffix_messages {
            println!("{}", message);
        }
    }

    fn json(&self) {
        let json_string = serde_json::to_string(&self.output_structure())
            .expect("Failed to serialize output structure to JSON");

        // Print the JSON string
        println!("{json_string}");
    }

    fn yaml(&self) {
        let yaml_string = serde_yaml::to_string(&self.output_structure())
            .expect("Failed to serialize output structure to YAML");

        // Print the YAML string
        println!("{yaml_string}");
    }

    fn toml(&self) {
        let toml_string = toml::to_string(&self.output_structure())
            .expect("Failed to serialize output structure to TOML");
        // Print the TOML string
        println!("{toml_string}");
    }

    fn output_structure(&self) -> OutputStructure<I> {
        OutputStructure {
            prefix_messages: Some(self.prefix_messages.clone()),
            suffix_messages: Some(self.suffix_messages.clone()),
            error_messages: Some(self.error_messages.clone()),
            table_rows: self.table_rows.clone(),
        }
    }
}

pub(super) struct CommandOutputBuilder<I, TOut>
where
    I: IntoIterator<Item = TOut> + Clone + Serialize,
    TOut: Tabled + Serialize + Clone,
{
    table_rows: Option<I>,
    prefix_messages: Vec<String>,
    suffix_messages: Vec<String>,
    error_messages: Vec<String>,
    mode: Option<OutputFormat>,
}

impl<I, TOut> CommandOutputBuilder<I, TOut>
where
    I: IntoIterator<Item = TOut> + Clone + Serialize,
    TOut: Tabled + Serialize + Clone,
{
    pub fn with_table_rows(mut self, table_rows: I) -> Self {
        self.table_rows = Some(table_rows);
        self
    }

    pub fn with_prefix_message(mut self, message: String) -> Self {
        self.prefix_messages.push(message);
        self
    }

    #[allow(unused)]
    pub fn with_suffix_message(mut self, message: String) -> Self {
        self.suffix_messages.push(message);
        self
    }

    #[allow(unused)]
    pub fn with_error_message(mut self, message: String) -> Self {
        self.error_messages.push(message);
        self
    }

    pub fn with_mode(mut self, mode: OutputFormat) -> Self {
        self.mode = Some(mode);
        self
    }

    pub fn build(self) -> CommandOutput<I, TOut> {
        CommandOutput {
            table_rows: self.table_rows,
            prefix_messages: self.prefix_messages,
            suffix_messages: self.suffix_messages,
            error_messages: self.error_messages,
            mode: self.mode.expect("Output format must be set"),
        }
    }
}

#[derive(Clone, Tabled, Serialize)]
pub(super) struct NoTable;

#[derive(Serialize)]
struct OutputStructure<TRows> {
    prefix_messages: Option<Vec<String>>,
    suffix_messages: Option<Vec<String>>,
    error_messages: Option<Vec<String>>,
    table_rows: Option<TRows>,
}
