use serde::de::DeserializeOwned;
use tabled::{Table, Tabled, settings::Style};

pub(super) fn output<I, TOut>(table_rows: I)
where
    I: IntoIterator<Item = TOut>,
    TOut: Tabled + DeserializeOwned,
{
    let mut table = Table::new(table_rows);
    table.with(Style::modern_rounded());
    println!("{table}");
}
