use anyhow::bail;
use anyhow::{Context, Result};
use selectel_mks::Client;
use structopt::StructOpt;

mod conf;
mod kubeversion;
mod task;

fn main() -> Result<()> {
    let cli_opts = conf::CliOptions::from_args();

    let client = Client::new(&cli_opts.mks_endpoint, &cli_opts.mks_token)
        .context("Failed to initialize MKS client")?;

    match cli_opts.resource {
        conf::Resource::Kubeversion(conf::Kubeversion {
            command: conf::KubeversionCommand::List { output },
        }) => kubeversion::list(&client, &output)?,

        conf::Resource::Task(conf::Task {
            command: conf::TaskCommand::List { cluster_id, output },
        }) => task::list(&client, &cluster_id, &output)?,
        _ => bail!("Unknown command"),
    };

    Ok(())
}
