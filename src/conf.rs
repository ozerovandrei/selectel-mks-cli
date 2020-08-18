use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "mks")]
pub struct CliOptions {
    #[structopt(short, long)]
    /// Activate debug mode
    debug: bool,

    #[structopt(short, long)]
    /// Activate verbose mode
    verbose: bool,

    #[structopt(long, env = "MKS_TOKEN")]
    /// MKS project-scoped token
    mks_token: String,

    #[structopt(long, env = "MKS_ENDPOINT")]
    /// MKS endpoint
    mks_endpoint: String,

    #[structopt(subcommand)]
    resource: Resource,
}

#[derive(Debug, StructOpt)]
enum Resource {
    /// Cluster commands
    Cluster(Cluster),

    /// Kubeversion commands
    Kubeversion(Kubeversion),

    /// Node commands
    Node(Node),

    /// Nodegroup commands
    Nodegroup(Nodegroup),

    /// Task commands
    Task(Task),
}

#[derive(Debug, StructOpt)]
struct Cluster {
    #[structopt(subcommand)]
    command: ClusterCommand,
}

#[derive(Debug, StructOpt)]
enum ClusterCommand {
    /// Get cluster
    Get {
        /// Cluster identifier
        #[structopt(name = "cluster-id")]
        id: String,
    },

    /// List all clusters
    List,

    /// Create a new cluster
    Create {
        /// Cluster name
        #[structopt(long)]
        name: String,

        /// Kubernetes version
        #[structopt(long)]
        kube_version: String,

        /// Cluster region
        #[structopt(long)]
        region: String,

        /// Reference to a pre-created network
        #[structopt(long)]
        network_id: Option<String>,

        /// Reference to a pre-created subnet
        #[structopt(long)]
        subnet_id: Option<String>,

        /// UTC time in "hh:mm:ss" format of when the cluster will start its maintenance tasks
        #[structopt(long)]
        maintenance_window_start: Option<String>,

        /// Flag that indicates if worker nodes are allowed to be reinstalled automatically
        /// in case of their unavailability or unhealthiness
        #[structopt(long)]
        enable_autorepair: Option<bool>,

        /// Flag that indicates if Kubernetes patch version of the cluster is allowed to be upgraded
        /// automatically
        #[structopt(long)]
        enable_patch_version_auto_upgrade: Option<bool>,

        /// Flag that indicates that cluster has only a single master and that
        /// control-plane is not in highly available mode
        #[structopt(long)]
        zonal: Option<bool>,
    },

    /// Delete cluster
    Delete {
        /// Cluster identifier
        #[structopt(name = "cluster-id")]
        id: String,
    },
}

#[derive(Debug, StructOpt)]
struct Kubeversion {
    #[structopt(subcommand)]
    command: KubeversionCommand,
}

#[derive(Debug, StructOpt)]
enum KubeversionCommand {
    /// List all available Kubernetes versions
    List,
}

#[derive(Debug, StructOpt)]
struct Node {
    #[structopt(subcommand)]
    command: NodeCommand,
}

#[derive(Debug, StructOpt)]
enum NodeCommand {
    /// List cluster nodes in the nodegroup
    List {
        /// Cluster identifier
        #[structopt(long)]
        cluster_id: String,

        /// Nodegroup identifier
        #[structopt(long)]
        nodegroup_id: String,
    },

    /// Reinstall single node
    Reinstall {
        /// Cluster identifier
        #[structopt(long)]
        cluster_id: String,

        /// Nodegroup identifier
        #[structopt(long)]
        nodegroup_id: String,

        /// Node identifier
        #[structopt(name = "node-id")]
        id: String,
    },
}

#[derive(Debug, StructOpt)]
struct Nodegroup {
    #[structopt(subcommand)]
    command: NodegroupCommand,
}

#[derive(Debug, StructOpt)]
enum NodegroupCommand {
    /// List cluster nodegroups
    List {
        /// Cluster identifier
        #[structopt(long)]
        cluster: String,
    },

    /// Get cluster nodegroup
    Get {
        /// Cluster identifier
        #[structopt(long)]
        cluster: String,

        /// Nodegroup identifier
        #[structopt(name = "nodegroup-id")]
        nodegroup: String,
    },

    /// Create a new nodegroup
    Create {
        /// Cluster identifier
        #[structopt(long)]
        cluster: String,

        /// Count of nodes
        #[structopt(long)]
        node_count: u32,

        /// Availability zone for all nodes in this nodegroup
        #[structopt(long)]
        availability_zone: String,

        /// Reference to a pre-created flavor, it can be omitted in most cases
        #[structopt(long)]
        flavor_id: Option<String>,

        /// CPU count for each node, it can be omitted only in cases when
        /// flavor-id is provided
        #[structopt(long)]
        cpus: Option<u32>,

        /// RAM value in MB for each node, it can be omitted only in cases when
        /// flavor-id is provided
        #[structopt(long)]
        ram_mb: Option<u32>,

        /// Volume size in GB for each node, it can be omitted only in cases when
        /// flavor-id is provided and volume is local
        #[structopt(long)]
        volume_gb: Option<u32>,

        /// BlockStorage volume type for each node, it can be omitted only in cases when
        /// flavor-id is set and volume is local
        #[structopt(long)]
        volume_type: Option<String>,

        /// Use local volume for each node
        #[structopt(long)]
        local_volume: Option<bool>,

        /// Name of the SSH key that will be added to all nodes
        #[structopt(long)]
        keypair_name: Option<String>,

        /// Optional parameter to tune nodes affinity
        #[structopt(long)]
        affinity_policy: Option<String>,
    },

    /// Set nodegroup parameters
    Set {
        /// Cluster identifier
        #[structopt(long)]
        cluster: String,

        /// Count of nodes
        #[structopt(long)]
        node_count: Option<u32>,
    },

    /// Delete nodegroup
    Delete {
        /// Cluster identifier
        #[structopt(long)]
        cluster: String,

        /// Nodegroup identifier
        #[structopt(name = "nodegroup-id")]
        nodegroup: String,
    },
}

#[derive(Debug, StructOpt)]
struct Task {
    #[structopt(subcommand)]
    command: TaskCommand,
}

#[derive(Debug, StructOpt)]
enum TaskCommand {
    /// List cluster tasks
    List {
        /// Cluster identifier
        #[structopt(long)]
        cluster: String,
    },

    /// Get cluster task
    Get {
        /// Cluster identifier
        #[structopt(long)]
        cluster: String,

        /// Task identifier
        #[structopt(name = "task-id")]
        task_id: String,
    },
}
