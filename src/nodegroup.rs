use anyhow::{bail, Context, Result};
use selectel_mks::nodegroup;
use selectel_mks::Client;
use term_table::row::Row;
use term_table::table_cell::{Alignment, TableCell};
use term_table::{Table, TableStyle};

use crate::json;

pub(crate) fn list(client: &Client, output: &str, cluster_id: &str) -> Result<()> {
    let nodegroups = client
        .list_nodegroups(cluster_id)
        .context("Failed to list nodegroups")?;

    match output {
        "table" => list_print_table(&nodegroups),
        "json" => json::print_json(nodegroups)?,
        _ => bail!("Unknown output format"),
    };

    Ok(())
}

fn list_print_table(nodegroups: &[nodegroup::schemas::Nodegroup]) {
    let mut table = Table::new();
    table.style = TableStyle::simple();

    table.add_row(Row::new(vec![
        TableCell::new_with_alignment("id", 1, Alignment::Center),
        TableCell::new_with_alignment("nodes_count", 1, Alignment::Center),
        TableCell::new_with_alignment("availability_zone", 1, Alignment::Center),
        TableCell::new_with_alignment("flavor_id", 1, Alignment::Center),
        TableCell::new_with_alignment("volume_gb", 1, Alignment::Center),
        TableCell::new_with_alignment("volume_type", 1, Alignment::Center),
        TableCell::new_with_alignment("local_volume", 1, Alignment::Center),
    ]));

    for nodegroup in nodegroups.iter() {
        table.add_row(Row::new(vec![
            TableCell::new(&nodegroup.id),
            TableCell::new(nodegroup.nodes.len()),
            TableCell::new(&nodegroup.availability_zone),
            TableCell::new(&nodegroup.flavor_id),
            TableCell::new(&nodegroup.volume_gb),
            TableCell::new(&nodegroup.volume_type),
            TableCell::new(&nodegroup.local_volume),
        ]));
    }

    println!("{}", table.render());
}
