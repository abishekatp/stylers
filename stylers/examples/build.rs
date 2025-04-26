fn main() -> color_eyre::Result<()> {
  // setup any logging you want,
  // if your in a build script this will only be visible if
  // the build script fails and `cargo` shows the raw stdout + stderr
  tracing_subscriber::FmtSubscriber::builder()
    .with_max_level(tracing::Level::DEBUG)
    .init();

  stylers::build(
    // this shows the defaults
    // the CWD for build scripts is in the root of the package,
    // i.e. where your Cargo.toml is
    stylers::BuildParams::builder()
      .with_output_path("./target/stylers_out.css".into())?
      .with_search_dir("./src".into())?
      .finish()?,
  )?;

  Ok(())
}
