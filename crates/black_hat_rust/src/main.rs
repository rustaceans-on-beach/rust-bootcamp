use clap::{Parser, Subcommand};

mod sha1_cracker;
mod tricoder;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
  #[command(subcommand)]
  command: Commands,
}

#[derive(Subcommand)]
enum Commands {
  #[command(about = "Sha1 cracker")]
  Sha1Cracker,
  #[command(about = "Tricoder")]
  Tricoder {
    #[arg()]
    target: String,
  },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let cli = Cli::parse();

  match &cli.command {
    Commands::Sha1Cracker => sha1_cracker::main(),
    Commands::Tricoder { target } => {
      tricoder::main(target).unwrap();
      Ok(())
    }
  }
}
