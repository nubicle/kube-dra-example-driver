use std::env;
use std::path::PathBuf;

use tracing::info;

pub struct Driver {
    driver_name: String,
    plugin_dir: PathBuf,
    registrar_dir: PathBuf,
    plugin: Option<kube_dra::KubeletPlugin>,
}

use super::app::Config;

impl Driver {
    pub fn new(cfg: &Config) -> Self {
        Self {
            driver_name: cfg.driver_name.as_ref().unwrap().clone(),
            plugin_dir: cfg.driver_plugin_path(),
            registrar_dir: PathBuf::from(&cfg.kubelet_registrar_directory_path),
            plugin: None,
        }
    }

    pub async fn start(&mut self) -> anyhow::Result<()> {
        info!(driver = %self.driver_name, "initializing driver");

        let node_name = env::var("NODE_NAME")?;
        let client = kube::Client::try_default().await?;

        let mut plugin = kube_dra::KubeletPlugin::builder()
            .driver_name(&self.driver_name)
            .kube_client(client)
            .plugin_data_dir(&self.plugin_dir)
            .registrar_directory_path(&self.registrar_dir)
            .node_name(node_name.as_str())
            .build()?;

        plugin.start().await?;
        info!(driver = %self.driver_name, "driver started");

        self.plugin = Some(plugin);

        Ok(())
    }

    pub async fn stop(&mut self) -> anyhow::Result<()> {
        info!(driver = %self.driver_name, "stopping driver");

        if let Some(plugin) = self.plugin.take() {
            plugin.stop().await?;
        }

        info!(driver = %self.driver_name, "driver stopped");

        Ok(())
    }
}
