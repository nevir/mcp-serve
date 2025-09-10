use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Directory to discover tools from
    #[arg(long, default_value = ".")]
    tools: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    println!("Discovering tools from directory: {}", cli.tools.display());
    println!("Tools functionality working");
}