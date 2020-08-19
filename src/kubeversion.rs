use anyhow::{bail, Context, Result};
use selectel_mks::kubeversion;
use selectel_mks::Client;
use term_table::row::Row;
use term_table::table_cell::{Alignment, TableCell};
use term_table::{Table, TableStyle};

pub(crate) fn list(client: &Client, output: &str) -> Result<()> {
    let kube_versions = client
        .list_kube_versions()
        .context("Failed to list Kubernetes versions")?;

    match output {
        "table" => list_print_table(kube_versions),
        "json" => list_print_json(kube_versions)?,
        _ => bail!("Unknown output format"),
    };

    Ok(())
}

fn list_print_table(kube_versions: Vec<kubeversion::schemas::KubeVersion>) {
    let mut table = Table::new();
    table.style = TableStyle::simple();

    table.add_row(Row::new(vec![
        TableCell::new_with_alignment("version", 1, Alignment::Center),
        TableCell::new_with_alignment("is_default", 1, Alignment::Center),
    ]));

    for kube_version in kube_versions.iter() {
        table.add_row(Row::new(vec![
            TableCell::new(&kube_version.version),
            TableCell::new(&kube_version.is_default),
        ]));
    }

    println!("{}", table.render());
}

fn list_print_json(kube_versions: Vec<kubeversion::schemas::KubeVersion>) -> Result<()> {
    let serialized = serde_json::to_string(&kube_versions)
        .map_err(selectel_mks::error::Error::SerializeError)?;

    println!("{}", serialized);

    Ok(())
}
