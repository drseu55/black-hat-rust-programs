use clap::{Arg, Command};
use std::{env, sync::Arc, time::Duration};

mod crawler;
mod error;
mod spiders;

use crate::crawler::Crawler;
use error::Error;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    env::set_var("RUST_LOG", "info,trust_dns_proto=error");
    env_logger::init();

    let cli = Command::new("Web crawler for cvedetalis website and Github JSON API")
        .version("0.1.0")
        .about("About section")
        .subcommand(Command::new("spiders").about("List all spiders"))
        .subcommand(
            Command::new("run")
                .about("Run a spider")
                .arg(
                    Arg::new("spider")
                        .short('s')
                        .long("spider")
                        .help("The spider to run")
                        .takes_value(true)
                        .required(true),
                )
                .arg_required_else_help(true),
        )
        .arg_required_else_help(true)
        .get_matches();

    if cli.subcommand_matches("spiders").is_some() {
        let spider_names = vec!["cvedetails", "github"];
        for name in spider_names {
            println!("{}", name);
        }
    } else if let Some(matches) = cli.subcommand_matches("run") {
        let spider_name = matches.value_of("spider").unwrap();
        let crawler = Crawler::new(Duration::from_millis(200), 2, 500);

        match spider_name {
            "cvedetails" => {
                let spider = Arc::new(spiders::cvedetails::CveDetailsSpider::new());
                crawler.run(spider).await;
            }
            "github" => {
                let spider = Arc::new(spiders::github::GitHubSpider::new());
                crawler.run(spider).await;
            }
            _ => return Err(Error::InvalidSpider(spider_name.to_string()).into()),
        };
    }

    Ok(())
}
