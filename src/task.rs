use anyhow::{bail, Context, Result};
use selectel_mks::task;
use selectel_mks::Client;
use term_table::row::Row;
use term_table::table_cell::{Alignment, TableCell};
use term_table::{Table, TableStyle};

pub(crate) fn list(client: &Client, cluster_id: &str, output: &str) -> Result<()> {
    let tasks = client
        .list_tasks(cluster_id)
        .context("Failed to list cluster tasks")?;

    match output {
        "table" => list_print_table(tasks),
        "json" => list_print_json(tasks)?,
        _ => bail!("Unknown output format"),
    };

    Ok(())
}

fn list_print_table(tasks: Vec<task::schemas::Task>) {
    let mut table = Table::new();
    table.style = TableStyle::simple();

    table.add_row(Row::new(vec![
        TableCell::new_with_alignment("id", 1, Alignment::Center),
        TableCell::new_with_alignment("started_at", 1, Alignment::Center),
        TableCell::new_with_alignment("updated_at", 1, Alignment::Center),
        TableCell::new_with_alignment("type", 1, Alignment::Center),
        TableCell::new_with_alignment("status", 1, Alignment::Center),
    ]));

    for task in tasks.iter() {
        let updated_at = match &task.updated_at {
            Some(time) => time.to_rfc3339(),
            None => String::new(),
        };

        table.add_row(Row::new(vec![
            TableCell::new(&task.id),
            TableCell::new(&task.started_at.to_rfc3339()),
            TableCell::new(updated_at),
            TableCell::new(&task.task_type),
            TableCell::new(&task.status),
        ]));
    }

    println!("{}", table.render());
}

fn list_print_json(tasks: Vec<task::schemas::Task>) -> Result<()> {
    let serialized =
        serde_json::to_string(&tasks).map_err(selectel_mks::error::Error::SerializeError)?;

    println!("{}", serialized);

    Ok(())
}
