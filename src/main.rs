use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum)]
enum Formatters {
    PlantUML,
    PlantUMLStructurizr,
    PlantUMLC4,
    Mermaid,
    WebSequence,
    Ilograph,
    DOT,
    JSON,
    DSL,
    Theme,
}

#[derive(Subcommand)]
enum Commands {
    /// The export command exports DSL files into specified formatted files using Structurizr-CLI
    Export {
        /// A file path for DSL files to be read from
        #[arg(short, long)]
        workspace: PathBuf,

        /// A format that Structurizr-CLI supports
        #[arg(short, long, value_enum)]
        formatter: Formatters,

        /// A file path for PlantUML files to be exported to
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// The render command renders DSL files into PNG files using a specific formatter
    Render {
        /// A format that Structurizr-CLI supports
        #[arg(short, long, value_enum)]
        formatter: Formatters,

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
