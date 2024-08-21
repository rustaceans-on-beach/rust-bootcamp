mod csv2yaml;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  use csv2yaml::main as csv2yaml_main;

  csv2yaml_main()?;

  Ok(())
}
