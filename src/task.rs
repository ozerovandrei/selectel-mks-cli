use anyhow::{bail, Context, Result};
use selectel_mks::task;
use selectel_mks::Client;
use term_table::row::Row;
use term_table::table_cell::{Alignment, TableCell};
use term_table::{Table, TableStyle};

use crate::json;

pub(crate) fn get(client: &Client, output: &str, cluster_id: &str, task_id: &str) -> Result<()> {
    let task = client
        .get_task(cluster_id, task_id)
        .context("Failed to get cluster task")?;

    match output {
        "table" => get_print_table(&task),
        "json" => json::print_json(task)?,
        _ => bail!("Unknown output format"),
    };

    Ok(())
}

fn new_table_with_headers<'a>() -> Table<'a> {
    let mut table = Table::new();
    table.style = TableStyle::simple();

    table.add_row(Row::new(vec![
        TableCell::new_with_alignment("id", 1, Alignment::Center),
        TableCell::new_with_alignment("started_at", 1, Alignment::Center),
        TableCell::new_with_alignment("updated_at", 1, Alignment::Center),
        TableCell::new_with_alignment("type", 1, Alignment::Center),
        TableCell::new_with_alignment("status", 1, Alignment::Center),
    ]));

    table
}

fn add_task_table_row(table: &mut Table, task: &task::schemas::Task) {
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

fn get_print_table(task: &task::schemas::Task) {
    let mut table = new_table_with_headers();

    add_task_table_row(&mut table, task);

    println!("{}", table.render());
}

pub(crate) fn list(client: &Client, output: &str, cluster_id: &str) -> Result<()> {
    let tasks = client
        .list_tasks(cluster_id)
        .context("Failed to list cluster tasks")?;

    match output {
        "table" => list_print_table(&tasks),
        "json" => json::print_json(tasks)?,
        _ => bail!("Unknown output format"),
    };

    Ok(())
}

fn list_print_table(tasks: &[task::schemas::Task]) {
    let mut table = new_table_with_headers();

    for task in tasks.iter() {
        add_task_table_row(&mut table, task);
    }

    println!("{}", table.render());
}
