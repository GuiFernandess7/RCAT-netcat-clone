use std::process;

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

            let run_result = connect::run(host, port);
            match run_result {
                Ok(()) => {
                    process::exit(0);
                }
                Err(e) => {
                    println!("failed - {}", e);
                    process::exit(0);
                }
            }
        },
        Commands::Serve { bind_host, port } => {
            println!("serve to {} and {}", bind_host, port);

            let run_result = serve::run(bind_host, port);
            match run_result {
                Ok(()) => {
                    process::exit(0);
                }
                Err(e) => {
                    println!("failed - {}", e);
                    process::exit(1);
                }
            }
        }
    }
}
