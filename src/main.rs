use anyhow::{Context, Result};
use selectel_mks::Client;
use structopt::StructOpt;

mod conf;
mod kubeversion;

fn main() -> Result<()> {
    let cli_opts = conf::CliOptions::from_args();

    let client = Client::new(&cli_opts.mks_endpoint, &cli_opts.mks_token)
        .context("Failed to initialize MKS client")?;

    if let conf::Resource::Kubeversion(conf::Kubeversion {
        command: conf::KubeversionCommand::List { output },
    }) = cli_opts.resource
    {
        kubeversion::list(&client, &output)?;
    }

    Ok(())
}
