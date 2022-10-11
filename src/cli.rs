use std::str::FromStr;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Cli {
    /// Config file
    #[arg(short, long, value_hint=clap::ValueHint::FilePath, default_value="./config.yml")]
    pub config: String,

    /// Node to export
    #[arg(short, long, default_value = "")]
    pub node: String,

    /// Output format, conf: wg config file, qr: QR code
    #[arg(short, long, default_value = "conf")]
    pub format: Format,

    /// Include all nodes
    #[arg(short, long)]
    pub all: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum Format {
    Conf,
    Qr,
}

impl Default for Format {
    fn default() -> Self {
        Format::Conf
    }
}

impl FromStr for Format {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "conf" | "c" => Ok(Format::Conf),
            "qr" | "q" => Ok(Format::Qr),
            _ => Err(anyhow::anyhow!("invalid format: {}", s)),
        }
    }
}
