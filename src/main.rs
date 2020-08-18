use structopt::StructOpt;

mod conf;

fn main() {
    let opt = conf::CliOptions::from_args();
    println!("{:#?}", opt);
}
