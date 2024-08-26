use clap::Parser;
use opts::{Opts, SubCommand};

mod deserializer;
mod opts;
mod serializer;

fn main() -> anyhow::Result<()> {
  let args = Opts::parse();

  match args.cmd {
    SubCommand::Csv(opt) => {
      let output = format!(
        "{}.{}",
        if opt.output.is_some() { opt.output.unwrap() } else { "output".to_string() },
        opt.format
      );

      let deserialized = deserializer::deserialize_csv(opt.input)?;

      let serialized = match opt.format {
        opts::OutputFormat::Json => serializer::serialize_json(deserialized)?,
        opts::OutputFormat::Yaml => serializer::serialize_yaml(deserialized)?,
      };

      std::fs::write(output, serialized)?;
    }
  }

  Ok(())
}
