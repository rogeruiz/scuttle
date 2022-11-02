mod constants;
mod export;

use crate::constants::structurizr;
use crate::export::command::run_export;

use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// The export command exports DSL files into specified formatted files using Structurizr-CLI
    Export {
        /// A file path for DSL files to be read from
        #[arg(short, long)]
        workspace: PathBuf,

        /// A format that Structurizr-CLI supports
        #[arg(short, long, value_enum, default_value_t = structurizr::Formatters::PlantUML)]
        format: structurizr::Formatters,

        /// A file path for PlantUML files to be exported to
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// The render command renders DSL files into PNG files using a specific format
    Render {
        /// A format that Structurizr-CLI supports
        #[arg(short, long, value_enum, default_value_t = structurizr::Formatters::PlantUML)]
        format: structurizr::Formatters,

        /// Input file path for files
        #[arg(short, long)]
        input: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Export {
            workspace,
            format,
            output,
        }) => {
            run_export();
        }
        Some(Commands::Render { input, format }) => {
            todo!()
        }
        None => {}
    }
}
