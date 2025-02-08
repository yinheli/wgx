use std::process;

use clap::Parser;
use config::Config;
use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use wgc::WireguardConfig;

mod cli;
mod config;
mod wgc;

fn main() {
    let cli = cli::Cli::parse();

    let config = match Config::new(&cli.config) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("load config file failed: {}", e);
            process::exit(1);
        }
    };

    let mut node = cli.node;

    if node.is_empty() {
        let nodes = config
            .servers
            .iter()
            .map(|s| s.node.clone())
            .collect::<Vec<_>>();

        let id = FuzzySelect::with_theme(&ColorfulTheme::default())
            .with_prompt("Select node to export:")
            .default(0)
            .items(&nodes)
            .interact();

        if let Ok(id) = id {
            node = nodes[id].clone();
        } else {
            println!("No node selected");
            process::exit(0);
        }
    }

    let cfg = WireguardConfig::new(&config, &node, cli.all);

    match cli.format {
        cli::Format::Conf => {
            println!("\n\n{}", cfg);
        }
        cli::Format::Qr => {
            let text = cfg.to_string();
            qr2term::print_qr(text).unwrap();
        }
    };
}
