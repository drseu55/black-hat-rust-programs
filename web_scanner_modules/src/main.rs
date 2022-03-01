use std::env;

use anyhow::Result;
use clap::{arg, Command};

mod cli;
mod common_ports;
mod dns;
mod error;
mod modules;
mod ports;
pub use error::Error;

fn main() -> Result<()> {
    env::set_var("RUST_LOG", "info,trust_dns_proto=error");
    env_logger::init();

    let cli = Command::new("Subdomain, port and vulnerabilities scanner")
        .version("0.1.0")
        .about("About section")
        .subcommand(Command::new("modules").about("List all modules"))
        .subcommand(
            Command::new("scan")
                .about("Scan a target")
                .arg(arg!(<TARGET> "The domain name to scan"))
                .arg_required_else_help(true),
        )
        .arg_required_else_help(true)
        .get_matches();

    if cli.subcommand_matches("modules").is_some() {
        cli::modules();
    } else if let Some(matches) = cli.subcommand_matches("scan") {
        let target = matches.value_of("TARGET").unwrap();
        cli::scan(target)?;
    }

    Ok(())
}
