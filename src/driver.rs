pub struct Driver;

use super::app::Config;

impl Driver {
    pub fn new(_: &Config) -> Self {
        Self
    }

    pub fn start(&self) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn stop(&self) -> anyhow::Result<()> {
        Ok(())
    }
}
