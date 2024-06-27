use clap::{
    Parser,
    Subcommand
};

mod connect;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Connect {
        host: String,

        #[arg(short, long)]
        port: u16,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Connect { host, port } => {
            println!("connected to {}:{}", host, port);
        }
    }
}
