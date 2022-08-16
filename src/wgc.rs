use serde_derive::Serialize;
use tera::{Context, Tera};

use crate::config::{self, Config};

#[derive(Serialize, Debug)]
pub struct WireguardConfig {
    pub interface: Interface,
    pub peers: Vec<Peer>,
}

impl WireguardConfig {
    pub fn new(config: &Config, node: &str, include_all: bool) -> Self {
        let server = config.servers.iter().find(|v| v.node == node).unwrap();
        let interface = Interface::new(server, config);

        let mut peers = Vec::new();
        if let Some(routes) = &server.routes {
            for route in routes {
                let peer = Peer::from_route(route, config);
                peers.push(peer);
            }
        }

        if include_all {
            for server in &config.servers {
                if server.node == node {
                    continue;
                }

                let exists = peers.iter().any(|v| v.node == server.node);

                if exists {
                    continue;
                }

                let peer = Peer::from_server(server);
                peers.push(peer);
            }
        }

        Self { interface, peers }
    }
}

#[derive(Serialize, Debug)]
pub struct Interface {
    pub node: String,
    pub network: String,
    pub listen_port: u16,
    pub private_key: String,
    pub mtu: u16,
    pub pre_up: Option<String>,
    pub pre_down: Option<String>,
    pub post_up: Option<String>,
    pub post_down: Option<String>,
}

impl Interface {
    pub fn new(server: &config::Server, cfg: &config::Config) -> Self {
        Self {
            node: server.node.clone(),
            network: server.network.clone(),
            listen_port: server.listen_port.unwrap_or(0),
            private_key: server.private_key.clone(),
            mtu: cfg.mtu,
            pre_up: server.pre_up.clone().or_else(|| cfg.pre_up.clone()),
            pre_down: server.pre_down.clone().or_else(|| cfg.pre_down.clone()),
            post_up: server.post_up.clone().or_else(|| cfg.post_up.clone()),
            post_down: server.post_down.clone().or_else(|| cfg.post_down.clone()),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct Peer {
    pub node: String,
    pub public_key: String,
    pub allowed_ips: Vec<String>,
    pub endpoint: Option<String>,
    pub persistent_keepalive: Option<u16>,
}

impl Peer {
    pub fn from_server(server: &config::Server) -> Self {
        Self {
            node: server.node.clone(),
            public_key: server.public_key.clone(),
            allowed_ips: vec![server.address.clone()],
            endpoint: server.public_address.clone(),
            persistent_keepalive: server.persistent_keepalive,
        }
    }

    pub fn from_route(route: &config::Route, cfg: &config::Config) -> Self {
        let server = cfg.servers.iter().find(|s| s.node == route.via).unwrap();
        let node = server.node.clone();
        let public_key = server.public_key.clone();

        let mut allowed_ips = vec![];

        allowed_ips.push(server.address.clone());
        let mut routes_ips = route.routes.to_vec();
        allowed_ips.append(&mut routes_ips);

        let endpoint = route
            .endpoint
            .clone()
            .or_else(|| server.public_address.clone());
        let persistent_keepalive = route.persistent_keepalive.or(server.persistent_keepalive);

        Self {
            node,
            public_key,
            allowed_ips,
            endpoint,
            persistent_keepalive,
        }
    }
}

impl ToString for WireguardConfig {
    fn to_string(&self) -> String {
        let mut tera = Tera::default();
        let tp = include_str!("wg.ini");
        let mut context = Context::new();
        context.insert("cfg", &self);
        tera.render_str(tp, &context).unwrap()
    }
}
