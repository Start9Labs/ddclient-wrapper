use std::io::Write;

#[derive(serde::Deserialize)]
#[serde(tag = "protocol")]
#[allow(non_camel_case_types)]
pub enum DNSConfig {
    dyndns2 {
        server: String,
        login: String,
        password: String,
        hostnames: Vec<String>,
        wildcard: bool,
        r#static: bool,
        custom: bool,
    },
    dyndns1 {
        server: String,
        login: String,
        password: String,
        hostnames: Vec<String>,
        wildcard: bool,
    },
    easydns {
        server: String,
        login: String,
        password: String,
        hostnames: Vec<String>,
        wildcard: bool,
    },
    namecheap {
        server: String,
        login: String,
        password: String,
        hostnames: Vec<String>,
    },
    zoneedit1 {
        server: String,
        login: String,
        password: String,
        hostnames: Vec<String>,
    },
    changeip {
        server: String,
        login: String,
        password: String,
        hostnames: Vec<String>,
    },
    googledomains {
        login: String,
        password: String,
        hostnames: Vec<String>,
    },
    duckdns {
        server: String,
        password: String,
        hostnames: Vec<String>,
    },
    dslreports {
        server: String,
        login: String,
        password: String,
        #[serde(rename = "host-id")]
        host_id: u64,
    },
}
use DNSConfig::*;

fn bool_str(b: bool) -> &'static str {
    if b {
        "yes"
    } else {
        "no"
    }
}

fn main() {
    let cfg: Vec<DNSConfig> = serde_yaml::from_reader(
        std::fs::File::open("/root/start9/config.yaml")
            .expect("open root/start9/config.yaml for read"),
    )
    .expect("parse config");
    let mut out = std::fs::File::create("/etc/ddclient/ddclient.conf")
        .expect("open /etc/ddclient/ddclient.conf for write");
    for cfg in cfg {
        match cfg {
            dyndns2 {
                server,
                login,
                password,
                hostnames,
                wildcard,
                r#static,
                custom,
            } => {
                write!(out, "server={},", server).expect("writing to /etc/ddclient/ddclient.conf");
                write!(out, "login={},", login).expect("writing to /etc/ddclient/ddclient.conf");
                write!(out, "password={},", password)
                    .expect("writing to /etc/ddclient/ddclient.conf");
                write!(out, "wildcard={},", bool_str(wildcard))
                    .expect("writing to /etc/ddclient/ddclient.conf");
                write!(out, "static={},", bool_str(r#static))
                    .expect("writing to /etc/ddclient/ddclient.conf");
                write!(out, "custom={},", bool_str(custom))
                    .expect("writing to /etc/ddclient/ddclient.conf");
                write!(out, "{}", hostnames.join(","))
                    .expect("writing to /etc/ddclient/ddclient.conf");
            }
            dyndns1 {
                server,
                login,
                password,
                hostnames,
                wildcard,
            } => {
                write!(out, "server={},", server).expect("writing to /etc/ddclient/ddclient.conf");
                write!(out, "login={},", login).expect("writing to /etc/ddclient/ddclient.conf");
                write!(out, "password={},", password)
                    .expect("writing to /etc/ddclient/ddclient.conf");
                write!(out, "wildcard={},", bool_str(wildcard))
                    .expect("writing to /etc/ddclient/ddclient.conf");
                write!(out, "{}", hostnames.join(","))
                    .expect("writing to /etc/ddclient/ddclient.conf");
            }
            easydns {
                server,
                login,
                password,
                hostnames,
                wildcard,
            } => {
                write!(out, "server={},", server).expect("writing to /etc/ddclient/ddclient.conf");
                write!(out, "login={},", login).expect("writing to /etc/ddclient/ddclient.conf");
                write!(out, "password={},", password)
                    .expect("writing to /etc/ddclient/ddclient.conf");
                write!(out, "wildcard={},", bool_str(wildcard))
                    .expect("writing to /etc/ddclient/ddclient.conf");
                write!(out, "{}", hostnames.join(","))
                    .expect("writing to /etc/ddclient/ddclient.conf");
            }
            namecheap {
                server,
                login,
                password,
                hostnames,
            } => {
                write!(out, "server={},", server).expect("writing to /etc/ddclient/ddclient.conf");
                write!(out, "login={},", login).expect("writing to /etc/ddclient/ddclient.conf");
                write!(out, "password={},", password)
                    .expect("writing to /etc/ddclient/ddclient.conf");
                write!(out, "{}", hostnames.join(","))
                    .expect("writing to /etc/ddclient/ddclient.conf");
            }
            zoneedit1 {
                server,
                login,
                password,
                hostnames,
            } => {
                write!(out, "server={},", server).expect("writing to /etc/ddclient/ddclient.conf");
                write!(out, "login={},", login).expect("writing to /etc/ddclient/ddclient.conf");
                write!(out, "password={},", password)
                    .expect("writing to /etc/ddclient/ddclient.conf");
                write!(out, "{}", hostnames.join(","))
                    .expect("writing to /etc/ddclient/ddclient.conf");
            }
            changeip {
                server,
                login,
                password,
                hostnames,
            } => {
                write!(out, "server={},", server).expect("writing to /etc/ddclient/ddclient.conf");
                write!(out, "login={},", login).expect("writing to /etc/ddclient/ddclient.conf");
                write!(out, "password={},", password)
                    .expect("writing to /etc/ddclient/ddclient.conf");
                write!(out, "{}", hostnames.join(","))
                    .expect("writing to /etc/ddclient/ddclient.conf");
            }
            googledomains {
                login,
                password,
                hostnames,
            } => {
                write!(out, "login={},", login).expect("writing to /etc/ddclient/ddclient.conf");
                write!(out, "password={},", password)
                    .expect("writing to /etc/ddclient/ddclient.conf");
                write!(out, "{}", hostnames.join(","))
                    .expect("writing to /etc/ddclient/ddclient.conf");
            }
            duckdns {
                server,
                password,
                hostnames,
            } => {
                write!(out, "server={},", server).expect("writing to /etc/ddclient/ddclient.conf");
                write!(out, "password={},", password)
                    .expect("writing to /etc/ddclient/ddclient.conf");
                write!(out, "{}", hostnames.join(","))
                    .expect("writing to /etc/ddclient/ddclient.conf");
            }
            dslreports {
                server,
                login,
                password,
                host_id,
            } => {
                write!(out, "server={},", server).expect("writing to /etc/ddclient/ddclient.conf");
                write!(out, "login={},", login).expect("writing to /etc/ddclient/ddclient.conf");
                write!(out, "password={},", password)
                    .expect("writing to /etc/ddclient/ddclient.conf");
                write!(out, "{}", host_id).expect("writing to /etc/ddclient/ddclient.conf");
            }
        }
    }
}
