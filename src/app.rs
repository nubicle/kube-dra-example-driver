use std::fs;
use std::os::unix::fs::DirBuilderExt;
use std::path::{self, PathBuf};

use clap::{Args, Parser};

use crate::driver;

// const CDI_ROOT: &str = "/etc/cdi";

#[derive(Parser)]
#[command(name = "example-driver")]
#[command(long_about = None)]
/// A DRA driver plugin implemented using kube-dra.
pub struct Cli {
    #[command(flatten)]
    config: Config,
}

#[derive(Args)]
pub struct Config {
    /// Name of the DRA driver. Its default is derived from the device profile.
    #[arg(long, env = "DRIVER_NAME")]
    pub driver_name: Option<String>,

    /// Name of the device profile.
    #[arg(long, env = "DEVICE_PROFILE", default_value_t = String::from("gpu"))]
    pub device_profile: String,

    /// Absolute path to the directory where CDI files will be generated.
    // #[arg(long, env = "CDI_ROOT", default_value_t = String::from(CDI_ROOT))]
    // pub cdi_root: String,

    /// Absolute path to the directory where kubelet stores plugin data.
    #[arg(long, env = "KUBELET_PLUGINS_DIRECTORY_PATH")]
    pub kubelet_plugins_directory_path: String,

    /// Absolute path to the directory where kubelet stores plugin registrations.
    #[arg(long, env = "KUBELET_REGISTRAR_DIRECTORY_PATH")]
    pub kubelet_registrar_directory_path: String,
}

impl Cli {
    pub async fn run(&mut self) -> anyhow::Result<()> {
        if self.config.driver_name == None {
            self.config.driver_name = Some(self.config.device_profile.clone() + ".example.com");
        }

        fs::DirBuilder::new()
            .recursive(true)
            .mode(0o750)
            .create(&self.driver_plugin_path())?;

        // match fs::metadata(&self.config.cdi_root) {
        //     Ok(m) => {
        //         if !m.is_dir() {
        //             anyhow::bail!(
        //                 "path for cdi file generation is not a directory: {}",
        //                 &self.config.cdi_root
        //             );
        //         }
        //     }
        //     Err(e) => match e.kind() {
        //         std::io::ErrorKind::NotFound => {
        //             fs::DirBuilder::new()
        //                 .recursive(true)
        //                 .mode(0o750)
        //                 .create(&self.config.cdi_root)?;
        //         }
        //         _ => anyhow::bail!(e),
        //     },
        // };

        let mut driver = driver::Driver::new(&self.config);
        driver.start().await?;

        // after a signal stop the driver
        tokio::signal::ctrl_c().await?;

        driver.stop().await?;

        Ok(())
    }

    fn driver_plugin_path(&self) -> path::PathBuf {
        let driver_name = &self.config.driver_name.as_ref().unwrap();

        PathBuf::from(&self.config.kubelet_plugins_directory_path).join(driver_name)
    }
}
