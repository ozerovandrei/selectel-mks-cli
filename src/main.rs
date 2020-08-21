use anyhow::bail;
use anyhow::{Context, Result};
use selectel_mks::Client;
use structopt::StructOpt;

mod conf;

mod cluster;
mod kubeversion;
mod task;

pub(crate) mod json;

fn main() -> Result<()> {
    let cli_opts = conf::CliOptions::from_args();

    let client = Client::new(&cli_opts.mks_endpoint, &cli_opts.mks_token)
        .context("Failed to initialize MKS client")?;

    match cli_opts.resource {
        // cluster get
        conf::Resource::Cluster(conf::Cluster {
            command: conf::ClusterCommand::Get { output, cluster_id },
        }) => cluster::get(&client, &output, &cluster_id)?,

        // kubeversion list
        conf::Resource::Kubeversion(conf::Kubeversion {
            command: conf::KubeversionCommand::List { output },
        }) => kubeversion::list(&client, &output)?,

        // task get
        conf::Resource::Task(conf::Task {
            command:
                conf::TaskCommand::Get {
                    output,
                    cluster_id,
                    task_id,
                },
        }) => task::get(&client, &output, &cluster_id, &task_id)?,

        // task list
        conf::Resource::Task(conf::Task {
            command: conf::TaskCommand::List { output, cluster_id },
        }) => task::list(&client, &output, &cluster_id)?,

        _ => bail!("Unknown command"),
    };

    Ok(())
}
