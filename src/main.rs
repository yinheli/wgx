use std::process;

use clap::Parser;
use config::Config;
use dialoguer::{theme::ColorfulTheme, Select};
use qrcodegen::{QrCode, QrCodeEcc};
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

        let id = Select::with_theme(&ColorfulTheme::default())
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
            println!("\n\n{}", cfg.to_string());
        }
        cli::Format::Qr => {
            let text = cfg.to_string();
            let qr = QrCode::encode_text(&text, QrCodeEcc::Low).unwrap();
            print_qr(&qr);
        }
    };
}

// https://github.com/nayuki/QR-Code-generator/blob/master/rust/examples/qrcodegen-demo.rs
fn print_qr(qr: &QrCode) {
    let border: i32 = 2;
    for y in -border..qr.size() + border {
        for x in -border..qr.size() + border {
            let c: char = if qr.get_module(x, y) { '█' } else { ' ' };
            print!("{0}{0}", c);
        }
        println!();
    }
    println!();
}
