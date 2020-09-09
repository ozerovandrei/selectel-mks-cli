use anyhow::bail;
use anyhow::{Context, Result};
use selectel_mks::cluster as mks_cluster;
use selectel_mks::nodegroup as mks_nodegroup;
use selectel_mks::Client;
use structopt::StructOpt;

mod conf;

mod cluster;
mod kubeversion;
mod node;
mod nodegroup;
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

        // node get
        conf::Resource::Node(conf::Node {
            command:
                conf::NodeCommand::Get {
                    output,
                    cluster_id,
                    nodegroup_id,
                    node_id,
                },
        }) => node::get(&client, &output, &cluster_id, &nodegroup_id, &node_id)?,

        // node reinstall
        conf::Resource::Node(conf::Node {
            command:
                conf::NodeCommand::Reinstall {
                    cluster_id,
                    nodegroup_id,
                    node_id,
                },
        }) => node::reinstall(&client, &cluster_id, &nodegroup_id, &node_id)?,

        // nodegroup list
        conf::Resource::Nodegroup(conf::Nodegroup {
            command: conf::NodegroupCommand::List { output, cluster_id },
        }) => nodegroup::list(&client, &output, &cluster_id)?,

        // nodegroup get
        conf::Resource::Nodegroup(conf::Nodegroup {
            command:
                conf::NodegroupCommand::Get {
                    output,
                    cluster_id,
                    nodegroup_id,
                },
        }) => nodegroup::get(&client, &output, &cluster_id, &nodegroup_id)?,

        // nodegroup create
        conf::Resource::Nodegroup(conf::Nodegroup {
            command:
                conf::NodegroupCommand::Create {
                    cluster_id,
                    nodes_count,
                    flavor_id,
                    cpus,
                    ram_mb,
                    volume_gb,
                    volume_type,
                    local_volume,
                    keypair_name,
                    affinity_policy,
                    availability_zone,
                },
        }) => {
            let mut opts = mks_nodegroup::schemas::CreateOpts::new(
                nodes_count,
                local_volume,
                &availability_zone,
            );
            if let Some(flavor_id) = flavor_id {
                opts = opts.with_flavor_id(&flavor_id);
            }
            if let Some(cpus) = cpus {
                opts = opts.with_cpus(cpus);
            }
            if let Some(ram_mb) = ram_mb {
                opts = opts.with_ram_mb(ram_mb);
            }
            if let Some(volume_gb) = volume_gb {
                opts = opts.with_volume_gb(volume_gb);
            }
            if let Some(volume_type) = volume_type {
                opts = opts.with_volume_type(&volume_type);
            }
            if let Some(keypair_name) = keypair_name {
                opts = opts.with_keypair_name(&keypair_name);
            }
            if let Some(affinity_policy) = affinity_policy {
                opts = opts.with_affinity_policy(&affinity_policy);
            }

            nodegroup::create(&client, &cluster_id, opts)?
        }

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
