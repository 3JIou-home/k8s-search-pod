use tracing;
use k8s_openapi::api::core::v1::Pod;
use kube::{api::{Api, ListParams, ResourceExt}, Client, Config};
use prettytable::*;
use getopts::Options;
use std::env;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("a", "address", "IP address for find pod", "ADDR");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!("{}", f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
    }
    if matches.opt_present("a") {

        let address = matches.opt_str("a").unwrap();
        let config = Config::infer().await?;
        let cluster_url = config.cluster_url;
        println!("Searching pod in cluster - {cluster_url:?}");
        let client = Client::try_default().await?;
        let pods: Api<Pod> = Api::all(client);
        let lp = ListParams::default().fields(&format!("status.podIP={}", &address));
        for item in pods.list(&lp).await? {
            let result_table = table!(["Request IP", "Pod name", "Pod namespace", "Pod on node"],
                                        [&address,
                                            item.name(),
                                            item.namespace().expect("No information"),
                                            item.spec.unwrap().node_name.expect("No information")]);
            result_table.printstd();
        }
    };
    Ok(())
}
