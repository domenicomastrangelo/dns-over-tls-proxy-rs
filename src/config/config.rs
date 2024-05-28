use tracing::event;
use tracing::Level;

use crate::cache;
use crate::config;

pub struct Config {
    logger: tracing_subscriber::FmtSubscriber,
    dns_over_tls_host: String,
    dns_over_tls_port: String,
    dns_over_tls_cert_path: String,
    cache_host: String,
    cache_port: String,
    cache: Box<dyn cache::cache::Cache>,
}

pub fn new(
    dns_over_tls_host: String,
    dns_over_tls_port: String,
    dns_over_tls_cert_path: String,
    cache_host: String,
    cache_port: String,
) -> Config {
    let logger = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .finish();
    config::config::Config {
        logger,
        dns_over_tls_host,
        dns_over_tls_port,
        dns_over_tls_cert_path,
        cache_host,
        cache_port,
        cache: Box::new(cache::cache::MemoryCache {
            cache: std::collections::HashMap::new(),
        }),
    }
}

impl Config {
    pub fn check(&self) {
        event!(
            Level::INFO,
            message = "Checking configuration",
            file = file!(),
            line = line!()
        );
    }
}
