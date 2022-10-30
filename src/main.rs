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
    /// The export command exports DSL files into PlantUML files using Structurizr-CLI
    Export {
        /// A file path for DSL files to be read from
        #[arg(short, long)]
        workspace: PathBuf,

        /// Format that Structurizr-CLI supports
        #[arg(short, long)]
        formatter: String,

        /// A file path for PlantUML files to be exported to
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// The render command renders PlantUML files into PNG files using PlantUML CLI
    Render {
        /// Format that Structurizr-CLI supports
        #[arg(short, long)]
        formatter: String,

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
            formatter,
            output,
        }) => {
            todo!()
        }
        Some(Commands::Render { input, formatter }) => {
            todo!()
        }
        None => {}
    }
}
