use clap::Parser;
use deserializer::deserialize_csv;
use opts::{Opts, SubCommand};
use serializer::serialize_json;

mod deserializer;
mod dto;
mod opts;
mod serializer;

fn main() -> anyhow::Result<()> {
  let args = Opts::parse();

  match args.cmd {
    SubCommand::Csv(opt) => {
      std::fs::write(opt.output, serialize_json(deserialize_csv(opt.input)?)?)?;
    }
  }

  Ok(())
}
