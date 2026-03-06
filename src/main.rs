use clap::Parser;

mod app;
mod driver;

fn main() -> anyhow::Result<()> {
    let mut app = app::Cli::parse();

    app.run()
}
