use anyhow::{bail, Context, Result};
use selectel_mks::cluster;
use selectel_mks::Client;
use term_table::row::Row;
use term_table::table_cell::{Alignment, TableCell};
use term_table::{Table, TableStyle};

use crate::json;

pub(crate) fn get(client: &Client, output: &str, cluster_id: &str) -> Result<()> {
    let cluster = client
        .get_cluster(cluster_id)
        .context("Failed to get cluster")?;

    match output {
        "table" => get_print_table(&cluster),
        "json" => json::print_json(cluster)?,
        _ => bail!("Unknown output format"),
    };

    Ok(())
}

fn get_print_table(cluster: &cluster::schemas::Cluster) {
    let updated_at = match &cluster.updated_at {
        Some(time) => time.to_rfc3339(),
        None => String::new(),
    };
    let pki_tree_updated_at = match &cluster.pki_tree_updated_at {
        Some(time) => time.to_rfc3339(),
        None => String::new(),
    };
    let maintenance_window_start = match &cluster.maintenance_window_start {
        Some(time) => time.to_string(),
        None => String::new(),
    };
    let maintenance_window_end = match &cluster.maintenance_window_end {
        Some(time) => time.to_string(),
        None => String::new(),
    };
    let maintenance_last_start = match &cluster.maintenance_last_start {
        Some(time) => time.to_rfc3339(),
        None => String::new(),
    };

    let mut table = Table::new();
    table.style = TableStyle::simple();
    table.separate_rows = false;

    table.add_row(Row::new(vec![
        TableCell::new("id"),
        TableCell::new(&cluster.id),
    ]));

    table.add_row(Row::new(vec![
        TableCell::new("created_at"),
        TableCell::new(&cluster.created_at.to_rfc3339()),
    ]));

    table.add_row(Row::new(vec![
        TableCell::new("updated_at"),
        TableCell::new(&updated_at),
    ]));

    table.add_row(Row::new(vec![
        TableCell::new("name"),
        TableCell::new(&cluster.name),
    ]));

    table.add_row(Row::new(vec![
        TableCell::new("status"),
        TableCell::new(&cluster.status),
    ]));

    table.add_row(Row::new(vec![
        TableCell::new("project_id"),
        TableCell::new(&cluster.project_id),
    ]));

    table.add_row(Row::new(vec![
        TableCell::new("network_id"),
        TableCell::new(&cluster.network_id),
    ]));

    table.add_row(Row::new(vec![
        TableCell::new("subnet_id"),
        TableCell::new(&cluster.subnet_id),
    ]));

    table.add_row(Row::new(vec![
        TableCell::new("kube_api_ip"),
        TableCell::new(&cluster.kube_api_ip),
    ]));

    table.add_row(Row::new(vec![
        TableCell::new("kube_version"),
        TableCell::new(&cluster.kube_version),
    ]));

    table.add_row(Row::new(vec![
        TableCell::new("region"),
        TableCell::new(&cluster.region),
    ]));

    table.add_row(Row::new(vec![
        TableCell::new("pki_tree_updated_at"),
        TableCell::new(&pki_tree_updated_at),
    ]));

    table.add_row(Row::new(vec![
        TableCell::new("maintenance_window_start"),
        TableCell::new(&maintenance_window_start),
    ]));

    table.add_row(Row::new(vec![
        TableCell::new("maintenance_window_end"),
        TableCell::new(&maintenance_window_end),
    ]));

    table.add_row(Row::new(vec![
        TableCell::new("maintenance_last_start"),
        TableCell::new(&maintenance_last_start),
    ]));

    table.add_row(Row::new(vec![
        TableCell::new("enable_autorepair"),
        TableCell::new(&cluster.enable_autorepair),
    ]));

    table.add_row(Row::new(vec![
        TableCell::new("enable_patch_version_auto_upgrade"),
        TableCell::new(&cluster.enable_patch_version_auto_upgrade),
    ]));

    table.add_row(Row::new(vec![
        TableCell::new("zonal"),
        TableCell::new(&cluster.zonal),
    ]));

    println!("{}", table.render());
}

pub(crate) fn list(client: &Client, output: &str) -> Result<()> {
    let clusters = client.list_clusters().context("Failed to list clusters")?;

    match output {
        "table" => list_print_table(&clusters),
        "json" => json::print_json(clusters)?,
        _ => bail!("Unknown output format"),
    };

    Ok(())
}

fn list_print_table(clusters: &[cluster::schemas::Cluster]) {
    let mut table = Table::new();
    table.style = TableStyle::simple();

    table.add_row(Row::new(vec![
        TableCell::new_with_alignment("id", 1, Alignment::Center),
        TableCell::new_with_alignment("name", 1, Alignment::Center),
        TableCell::new_with_alignment("kube_version", 1, Alignment::Center),
        TableCell::new_with_alignment("kube_api_ip", 1, Alignment::Center),
        TableCell::new_with_alignment("status", 1, Alignment::Center),
    ]));

    for cluster in clusters.iter() {
        table.add_row(Row::new(vec![
            TableCell::new(&cluster.id),
            TableCell::new(&cluster.name),
            TableCell::new(&cluster.kube_version),
            TableCell::new(&cluster.kube_api_ip),
            TableCell::new(&cluster.status),
        ]));
    }

    println!("{}", table.render());
}

pub(crate) fn create(
    client: &Client,
    output: &str,
    opts: cluster::schemas::CreateOpts,
) -> Result<()> {
    let cluster = client
        .create_cluster(&opts)
        .context("Failed to create cluster")?;

    match output {
        "table" => get_print_table(&cluster),
        "json" => json::print_json(cluster)?,
        _ => bail!("Unknown output format"),
    };

    Ok(())
}

pub(crate) fn delete(client: &Client, cluster_id: &str) -> Result<()> {
    client
        .delete_cluster(cluster_id)
        .context("Failed to delete cluster")?;

    Ok(())
}
