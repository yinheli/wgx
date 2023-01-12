use std::collections::HashSet;
use std::fs;
use std::net::IpAddr;
use std::str::FromStr;

use base64::{engine::general_purpose, Engine as _};
use ipnet::IpNet;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use x25519_dalek::PublicKey;
use x25519_dalek::StaticSecret;

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub network: String,
    pub mtu: u16,
    pub pre_up: Option<String>,
    pub pre_down: Option<String>,
    pub post_up: Option<String>,
    pub post_down: Option<String>,
    pub persistent_keepalive: Option<u16>,
    pub servers: Vec<Server>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Server {
    pub node: String,
    pub address: String,
    #[serde(skip_serializing, skip_deserializing)]
    pub network: String,
    pub listen_port: Option<u16>,
    pub dns: Option<String>,
    pub private_key: String,
    #[serde(skip_serializing, skip_deserializing)]
    pub public_key: String,
    pub public_address: Option<String>,
    pub pre_up: Option<String>,
    pub pre_down: Option<String>,
    pub post_up: Option<String>,
    pub post_down: Option<String>,
    pub persistent_keepalive: Option<u16>,
    pub routes: Option<Vec<Route>>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Route {
    pub via: String,
    pub routes: Vec<String>,
    pub endpoint: Option<String>,
    pub persistent_keepalive: Option<u16>,
}

impl Config {
    pub fn new(file: &str) -> Result<Self, anyhow::Error> {
        let mut config: Self = serde_yaml::from_str(fs::read_to_string(file)?.as_str())?;

        config.valid()?;

        let network = IpNet::from_str(config.network.as_str())?;

        config.servers.iter_mut().for_each(|s| {
            s.network = format!("{}/{}", s.address, network.prefix_len());

            let private_key: [u8; 32] = general_purpose::STANDARD
                .decode(&s.private_key)
                .unwrap()
                .try_into()
                .unwrap();
            let private_key = StaticSecret::from(private_key);
            let public_key = PublicKey::from(&private_key);
            s.public_key = general_purpose::STANDARD.encode(public_key.as_bytes());
        });

        Ok(config)
    }

    fn valid(&self) -> Result<(), anyhow::Error> {
        anyhow::ensure!(!self.network.is_empty(), "no network defined");
        anyhow::ensure!(!self.servers.is_empty(), "no servers defined");

        let network = IpNet::from_str(self.network.as_str())?;

        let addrs = self
            .servers
            .iter()
            .map(|v| IpAddr::from_str(v.address.as_str()))
            .collect::<Result<Vec<_>, _>>()?;

        anyhow::ensure!(
            addrs.len() == self.servers.len(),
            "each node should have an unique address"
        );

        for a in addrs.iter() {
            anyhow::ensure!(
                network.contains(a),
                "address {} is not in network {}",
                a,
                network
            );
        }

        let private_keys = self
            .servers
            .iter()
            .map(|v| v.private_key.as_str())
            .collect::<Vec<_>>();
        let private_keys: HashSet<&str> = HashSet::from_iter(private_keys.iter().cloned());

        anyhow::ensure!(
            private_keys.len() == self.servers.len(),
            "each node should have a private key"
        );

        for v in self.servers.iter() {
            if let Some(routes) = &v.routes {
                for r in routes.iter() {
                    let contains = self
                        .servers
                        .iter()
                        .filter(|s| s.node != v.node)
                        .map(|s| s.node.as_str())
                        .any(|x| x == r.via.as_str());

                    anyhow::ensure!(contains, "{} require node {} not found", v.node, r.via);
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Config;
    use crate::wgc::WireguardConfig;

    #[test]
    fn it_works() {
        let config = Config::new("config.example.yml").unwrap();

        let config = WireguardConfig::new(&config, "home-gateway", true);
        println!("{:?}", config);
    }
}
