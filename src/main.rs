use anyhow::bail;
use anyhow::{Context, Result};
use selectel_mks::cluster as mks_cluster;
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

        // cluster list
        conf::Resource::Cluster(conf::Cluster {
            command: conf::ClusterCommand::List { output },
        }) => cluster::list(&client, &output)?,

        // cluster create
        conf::Resource::Cluster(conf::Cluster {
            command:
                conf::ClusterCommand::Create {
                    output,
                    name,
                    kube_version,
                    region,
                    network_id,
                    subnet_id,
                    maintenance_window_start,
                    enable_autorepair,
                    enable_patch_version_auto_upgrade,
                    zonal,
                },
        }) => {
            let mut opts = mks_cluster::schemas::CreateOpts::new(&name, &kube_version, &region);
            if let Some(network_id) = network_id {
                opts = opts.with_network_id(&network_id);
            }
            if let Some(subnet_id) = subnet_id {
                opts = opts.with_subnet_id(&subnet_id);
            }
            if let Some(maintenance_window_start) = maintenance_window_start {
                opts = opts.with_maintenance_window_start(&maintenance_window_start);
            }
            if let Some(enable_autorepair) = enable_autorepair {
                opts = opts.with_enable_autorepair(enable_autorepair);
            }
            if let Some(enable_patch_version_auto_upgrade) = enable_patch_version_auto_upgrade {
                opts =
                    opts.with_enable_patch_version_auto_upgrade(enable_patch_version_auto_upgrade);
            }
            if let Some(zonal) = zonal {
                opts = opts.with_zonal(zonal);
            }

            cluster::create(&client, &output, opts)?
        }

        // cluster delete
        conf::Resource::Cluster(conf::Cluster {
            command: conf::ClusterCommand::Delete { cluster_id },
        }) => cluster::delete(&client, &cluster_id)?,

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
