use clap::{Parser, Subcommand};
use std::{fmt::Display, str::FromStr};

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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OutputFormat {
  Json,
  Yaml,
}

#[derive(Debug, Parser)]
#[clap(name = "csv", about = "Show or convert CSV to other formats.")]
pub struct Csv {
  // value_parser is a custom attribute that allows us to specify a function that will parse the value of the argument.
  #[arg(short, long, value_parser = parse_input_file_path)]
  pub input: String,
  #[arg(short, long)]
  pub output: Option<String>,
  #[arg(long, value_parser = parse_output_format, default_value = "json")]
  pub format: OutputFormat,
}

impl Display for OutputFormat {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", Into::<&str>::into(*self))
  }
}

impl FromStr for OutputFormat {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "json" => Ok(OutputFormat::Json),
      "yaml" => Ok(OutputFormat::Yaml),
      _ => Err(anyhow::anyhow!("Invalid output format: {s}").to_string()),
    }
  }
}

impl From<OutputFormat> for &str {
  fn from(format: OutputFormat) -> Self {
    match format {
      OutputFormat::Json => "json",
      OutputFormat::Yaml => "yaml",
    }
  }
}

fn parse_output_format(format: &str) -> Result<OutputFormat, String> {
  format.parse()
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
