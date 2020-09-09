# selectel-mks-cli: CLI for the Selectel Managed Kubernetes Service

[![crates.io](https://img.shields.io/crates/v/selectel-mks-cli.svg)](https://crates.io/crates/selectel-mks-cli)
[![Documentation](https://docs.rs/selectel-mks-cli/badge.svg)](https://docs.rs/selectel-mks-cli)
![CI](https://github.com/ozerovandrei/selectel-mks-cli/workflows/CI/badge.svg?branch=master)

CLI to the Selectel MKS V1 API.

## Installation

Download a binary for the needed platform from the releases page.

## Usage

You need to specify two mandatory parameters to use CLI:

 * `MKS endpoint` that can be specified by the `--mks-endpoint` option or `MKS_ENDPOINT` environment variable;
 * `MKS project-scoped token` that can be specified by the `--mks-token` option or `MKS_TOKEN` environment variable.

You can use `help` command to see all available subcommands:

```bash
$ mks help
mks 0.1.0

USAGE:
    mks [FLAGS] --mks-endpoint <mks-endpoint> --mks-token <mks-token> <SUBCOMMAND>

FLAGS:
    -d, --debug      Activate debug mode
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Activate verbose mode

OPTIONS:
        --mks-endpoint <mks-endpoint>    MKS endpoint [env: MKS_ENDPOINT]
        --mks-token <mks-token>          MKS project-scoped token [env: MKS_TOKEN]

SUBCOMMANDS:
    cluster        Cluster commands
    help           Prints this message or the help of the given subcommand(s)
    kubeversion    Kubeversion commands
    node           Node commands
    nodegroup      Nodegroup commands
    task           Task commands
```

You can also use `help` with any subcommand to see all available nested subcommands and their options:

```bash
$ mks help nodegroup
mks-nodegroup 0.1.0
Nodegroup commands

USAGE:
    mks nodegroup <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    create    Create a new nodegroup
    delete    Delete nodegroup
    get       Get cluster nodegroup
    help      Prints this message or the help of the given subcommand(s)
    list      List cluster nodegroups
    set       Set nodegroup parameters
``` 

```bash
$ mks help nodegroup set
mks-nodegroup-set 0.1.0
Set nodegroup parameters

USAGE:
    mks nodegroup set [OPTIONS] <nodegroup-id> --cluster-id <cluster-id>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --cluster-id <cluster-id>      Cluster identifier
        --nodes-count <nodes-count>    Count of nodes

ARGS:
    <nodegroup-id>    Nodegroup identifier
```

## How to get available values for mks-endpoint option

There are separate endpoints for each MKS region:

```bash
ru-1: https://ru-1.mks.selcloud.ru/v1
ru-2: https://ru-2.mks.selcloud.ru/v1
ru-3: https://ru-3.mks.selcloud.ru/v1
ru-7: https://ru-7.mks.selcloud.ru/v1
ru-8: https://ru-8.mks.selcloud.ru/v1
```

You can get available endpoints from the endpoint catalog in the Selectel Cloud.

## How to get mks-token value

You can see this token in your web-browser console when working with the Selectel Cloud [API](https://developers.selectel.ru/docs/selectel-cloud-platform/main-services/selectel_cloud_management_api/).  
You can also create this token with [Terraform](https://registry.terraform.io/providers/selectel/selectel/latest/docs/resources/vpc_token_v2), [Go library](https://pkg.go.dev/github.com/selectel/go-selvpcclient@v1.12.0/selvpcclient/resell/v2/tokens?tab=doc) or [Python library/CLI](https://github.com/selectel/python-selvpcclient).

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.