use tracing::info;

pub struct Driver {
    driver_name: String,
    plugin_dir: PathBuf,
    registrar_dir: PathBuf,
    plugin: Option<kube_dra::KubeletPlugin>,
}

use std::path::PathBuf;

use super::app::Config;

impl Driver {
    pub fn new(cfg: &Config) -> Self {
        let driver_name = cfg.driver_name.clone().unwrap();
        let plugin_dir = PathBuf::from(&cfg.kubelet_plugins_directory_path).join(&driver_name);
        let registrar_dir = PathBuf::from(&cfg.kubelet_registrar_directory_path);

        Self {
            driver_name,
            plugin_dir,
            registrar_dir,
            plugin: None,
        }
    }

    pub async fn start(&mut self) -> anyhow::Result<()> {
        info!(driver = %self.driver_name, "starting driver");

        let plugin = kube_dra::KubeletPlugin::builder()
            .driver_name(&self.driver_name)
            .plugin_dir(&self.plugin_dir)
            .registrar_dir(&self.registrar_dir)
            .build();

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
