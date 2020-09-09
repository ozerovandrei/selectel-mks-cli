use anyhow::{bail, Context, Result};
use selectel_mks::node;
use selectel_mks::Client;
use term_table::row::Row;
use term_table::table_cell::TableCell;
use term_table::{Table, TableStyle};

use crate::json;

pub(crate) fn get(
    client: &Client,
    output: &str,
    cluster_id: &str,
    nodegroup_id: &str,
    node_id: &str,
) -> Result<()> {
    let node = client
        .get_node(cluster_id, nodegroup_id, node_id)
        .context("Failed to get node")?;

    match output {
        "table" => get_print_table(&node),
        "json" => json::print_json(node)?,
        _ => bail!("Unknown output format"),
    };

    Ok(())
}

fn get_print_table(node: &node::schemas::Node) {
    let updated_at = match &node.updated_at {
        Some(time) => time.to_rfc3339(),
        None => String::new(),
    };

    let mut table = Table::new();
    table.style = TableStyle::simple();
    table.separate_rows = false;

    table.add_row(Row::new(vec![
        TableCell::new("id"),
        TableCell::new(&node.id),
    ]));

    table.add_row(Row::new(vec![
        TableCell::new("created_at"),
        TableCell::new(&node.created_at.to_rfc3339()),
    ]));

    table.add_row(Row::new(vec![
        TableCell::new("updated_at"),
        TableCell::new(&updated_at),
    ]));

    table.add_row(Row::new(vec![
        TableCell::new("hostname"),
        TableCell::new(&node.hostname),
    ]));

    table.add_row(Row::new(vec![
        TableCell::new("ip"),
        TableCell::new(&node.ip),
    ]));

    table.add_row(Row::new(vec![
        TableCell::new("nodegroup_id"),
        TableCell::new(&node.nodegroup_id),
    ]));

    println!("{}", table.render());
}

pub(crate) fn reinstall(
    client: &Client,
    cluster_id: &str,
    nodegroup_id: &str,
    node_id: &str,
) -> Result<()> {
    client
        .reinstall_node(cluster_id, nodegroup_id, node_id)
        .context("Failed to reinstall node")?;

    Ok(())
}
