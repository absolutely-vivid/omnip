use clap::Parser;
use log::{error, info};
use rsproxy::*;

extern crate pretty_env_logger;

fn main() {
    let args = RsproxyArgs::parse();

    rs_utilities::LogHelper::init_logger("rsp", &args.loglevel);

    let addr = parse_sock_addr(&args.addr);
    if addr.is_none() {
        error!("invalid address: {}", &args.addr);
        return;
    }

    let config = Config {
        addr: addr.unwrap(),
        downstream_addr: parse_sock_addr(args.downstream.as_str()),
        proxy_rules_file: args.proxy_rules_file,
        threads: args.threads,
        dot_server: args.dot_server,
        name_servers: args.name_servers,
    };

    let mut server = Server::new(config);
    server.set_enable_on_info_report(true);
    server.set_on_info_listener(|data: &str| {
        info!("Server Info: {}", data);
    });
    server.start_and_block().ok();
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct RsproxyArgs {
    /// Address ([ip:]port pair) to listen on
    #[clap(short = 'l', long, required = true, display_order = 1)]
    addr: String,

    /// Downstream of current proxy server, e.g. -d [ip:]port
    #[clap(short = 'd', long, default_value = "", display_order = 2)]
    downstream: String,

    /// Path to the proxy rules file
    #[clap(short = 'r', long, default_value = "", display_order = 3)]
    proxy_rules_file: String,

    /// Threads to run async tasks
    #[clap(short = 't', long, default_value = "0", display_order = 4)]
    threads: usize,

    /// DoT (DNS-over-TLS) server, e.g. dns.google
    #[clap(long, default_value = "", display_order = 5)]
    dot_server: String,

    /// comma saprated domain servers (E.g. 1.1.1.1,8.8.8.8), will be used if no dot_server is specified, or system default if empty
    #[clap(long, default_value = "", display_order = 6)]
    name_servers: String,

    #[clap(short = 'L', long, possible_values = &["T", "D", "I", "W", "E"], default_value = "I", display_order = 7)]
    loglevel: String,
}
