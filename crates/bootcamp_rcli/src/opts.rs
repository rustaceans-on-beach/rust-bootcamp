use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "rcli", author, version, about, long_about = None)]
pub struct Opts {
  #[command(subcommand)]
  pub cmd: SubCommand,
}

#[derive(Debug, Subcommand)]
pub enum SubCommand {
  Csv(Csv),
}

#[derive(Debug, Parser)]
#[clap(name = "csv", about = "Show or convert CSV to other formats.")]
pub struct Csv {
  // value_parser is a custom attribute that allows us to specify a function that will parse the value of the argument.
  #[arg(short, long, value_parser = parse_input_file_path)]
  pub input: String,
  #[arg(short, long, default_value = "output.json", value_parser = parse_output_file_path)]
  pub output: String,
}

fn parse_input_file_path(file_path: &str) -> Result<String, String> {
  let is_csv_file = file_path.ends_with(".csv");

  if !is_csv_file {
    return Err(format!("The file {file_path} is not a CSV file."));
  }

  let exists = std::path::Path::new(file_path).exists();
  if !exists {
    return Err(format!("The file {file_path} does not exist."));
  }

  Ok(file_path.into())
}

fn parse_output_file_path(file_path: &str) -> Result<String, String> {
  let is_json_file = file_path.ends_with(".json");

  if !is_json_file {
    return Err(format!("Only JSON files are supported, but got {file_path}."));
  }

  Ok(file_path.into())
}
