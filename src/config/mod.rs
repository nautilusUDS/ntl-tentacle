use anyhow::Result;
use std::env;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Config {
    pub service_name: String,
    pub target_addr: String,
    pub socket_name: String,
    pub base_dir: PathBuf,
    pub max_connections: usize,
    pub metrics_interval_secs: u64,
}

impl Config {
    pub fn load() -> Result<Self> {
        tracing::debug!("loading configuration from environment variables");

        let service_name = env::var("NAUTROUDS_SERVICE_NAME")
            .or_else(|_| env::var("NAUTROUDS_SERVICE_ID"))
            .or_else(|_| env::var("SERVICE_NAME"))
            .or_else(|_| env::var("SERVICE"))
            .or_else(|_| env::var("NAME"))
            .map_err(|_| {
                tracing::error!(
                    var = "NAUTROUDS_SERVICE_NAME",
                    "missing required environment variable"
                );
                anyhow::anyhow!("NAUTROUDS_SERVICE_NAME is required")
            })?;

        let target_addr = env::var("NAUTROUDS_TARGET_ADDR")
            .or_else(|_| env::var("TARGET_ADDR"))
            .or_else(|_| env::var("TARGET"))
            .or_else(|_| env::var("ADDR"))
            .map_err(|_| {
            tracing::error!(
                var = "NAUTROUDS_TARGET_ADDR",
                "missing required environment variable"
            );
            anyhow::anyhow!("NAUTROUDS_TARGET_ADDR is required")
        })?;

        let socket_name =
            env::var("NAUTROUDS_SOCKET_NAME").unwrap_or_else(|_| "node-0.sock".to_string());

        let base_dir = env::var("NAUTROUDS_SERVICES_DIR")
            .unwrap_or_else(|_| "/var/run/nautrouds/services".to_string())
            .into();

        let max_connections = env::var("NAUTROUDS_MAX_CONNS")
            .unwrap_or_else(|_| "1024".to_string())
            .parse()
            .unwrap_or(1024);

        let metrics_interval_secs = env::var("NAUTROUDS_METRICS_INTERVAL_SECS")
            .unwrap_or_else(|_| "15".to_string())
            .parse()
            .unwrap_or(15);

        let cfg = Self {
            service_name,
            target_addr,
            socket_name,
            base_dir,
            max_connections,
            metrics_interval_secs,
        };

        tracing::info!(
            service = %cfg.service_name,
            target = %cfg.target_addr,
            socket = ?cfg.socket_path(),
            max_conns = cfg.max_connections,
            metrics_interval = cfg.metrics_interval_secs,
            "configuration loaded"
        );

        Ok(cfg)
    }

    pub fn socket_path(&self) -> PathBuf {
        self.base_dir
            .join(&self.service_name)
            .join(&self.socket_name)
    }

    #[cfg(unix)]
    pub fn service_dir(&self) -> PathBuf {
        self.base_dir.join(&self.service_name)
    }
}
