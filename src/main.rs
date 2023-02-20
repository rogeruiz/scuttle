mod constants;
mod export;

use std::process;
use std::{fs, path::PathBuf};

use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use anyhow::anyhow;

use crate::constants::structurizr;
use crate::export::command::run_export;

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
        #[arg(short, long, default_value = "out")]
        output: PathBuf,
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

fn exit_on_error(error: anyhow::Error) {
    let mut output_stream = StandardStream::stdout(ColorChoice::Always);
    match output_stream.set_color(ColorSpec::new().set_fg(Some(Color::Red))) {
        Ok(_) => {
            writeln!(&mut output_stream, "Error: {}", error).unwrap();
        }
        Err(_) => unreachable!(),
    }
    process::exit(1);
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Export {
            workspace,
            format,
            output,
        }) => {
            // Check to see if the workspace is pointing to a file and a DSL file at that.
            let mut valid_workspace: &str = "";
            match workspace.is_file() {
                true => match workspace.extension() {
                    None => {
                        unreachable!();
                    }
                    Some(ext) => {
                        if "dsl" != ext {
                            exit_on_error(anyhow!(
                                "The workspace must be a Structurizr DSL file: {:?}\nPlease use the .dsl extension when selecting a workspace.",
                            ext))
                        }
                        valid_workspace = workspace.to_str().unwrap();
                    }
                },
                false => exit_on_error(anyhow!("The workspace must be a file: {:?}", workspace)),
            }

            // Check for valid format according to our structurizr::Formatters and match them to
            // what the Structurizr-CLI supports as a format.
            let valid_format: &str;
            match format {
                structurizr::Formatters::PlantUML => valid_format = "plantuml",
                structurizr::Formatters::Mermaid => valid_format = "mermaid",
            }

            // Check to see if the output directory is a valid path and create it if it does not
            // exist.
            let mut valid_output: &str = "";
            // We start here to check if the output PathBuf is a directory. If it is, we're good
            // and we return path as a &str. And if it's false (not a directory), then we try to
            // use fs::create_dir_all with the a reference to `output`. And if that successfully
            // gets created, then we'll return the path as a &str again. If any of this fails, then
            // we'll exit and say that the directory could not be created at all.
            match output.is_dir() {
                true => {
                    valid_output = output.to_str().unwrap();
                }
                false => match fs::create_dir_all(&output) {
                    Ok(_) => {
                        valid_output = output.to_str().unwrap();
                    }
                    Err(_) => exit_on_error(anyhow!(
                        "The output directory could not be created!: {:?}",
                        output
                    )),
                },
            };

            run_export(valid_format, valid_workspace, valid_output).ok();
        }
        Some(Commands::Render { input, format }) => {
            assert_eq!(input, input);
            assert_eq!(format, format);
            todo!()
        }
        None => {}
    }
}
