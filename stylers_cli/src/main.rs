use clap::Parser;
use color_eyre::eyre::Context as _;

#[derive(clap::Parser)]
#[clap(version, about, long_about = None)]
struct Cli {
  #[clap(flatten)]
  args: stylers::BuildParamsBuilder,
}

fn tracing() {
  use tracing_error::ErrorLayer;
  use tracing_subscriber::prelude::*;
  use tracing_subscriber::{EnvFilter, fmt};

  let fmt_layer = fmt::layer().with_target(true);
  let filter_layer = EnvFilter::try_from_default_env()
    .or_else(|_| EnvFilter::try_new("debug,stylers=trace"))
    .unwrap();

  tracing_subscriber::registry()
    .with(filter_layer)
    .with(fmt_layer)
    .with(ErrorLayer::default())
    .init();
}

fn main() -> color_eyre::Result<()> {
  tracing();
  color_eyre::install()?;

  let args = Cli::parse();
  let build_params: stylers::BuildParams = args
    .args
    .finish()
    .wrap_err("Defaults for build_params to stylers::build didn't suffice")?;

  stylers::build(build_params).wrap_err("Failed to build using stylers")?;

  Ok(())
}
