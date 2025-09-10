use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Enable tool functionality
    #[arg(long)]
    tool: bool,
}

fn main() {
    let cli = Cli::parse();

    if cli.tool {
        println!("Tool functionality working");
    } else {
        println!("mcp-serve wireframe - use --help for options");
    }
}