use clap::Parser;

mod app;
mod driver;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();

    let mut app = app::Cli::parse();

    app.run().await
}
