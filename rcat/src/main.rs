use clap::{
    Parser,
    Subcommand
};

mod connect;
mod serve;

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

    Serve {
        #[arg(default_value = "127.0.0.1")]
        bind_host: String,

        #[arg(short, long, value_parser)]
        port: u16,
    }
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Connect { host, port } => {
            println!("connected to {}:{}", host, port);
        },
        Commands::Serve { bind_host, port } => {
            println!("serve to {} and {}", bind_host, port);
        }
    }
}
