use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Directory to discover tools from
    #[arg(default_value = ".")]
    tools_dir: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    println!("Discovering tools from directory: {}", cli.tools_dir.display());
    println!("Tools functionality working");
}