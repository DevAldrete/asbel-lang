use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct AsbelCli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Parses an Asbel file and prints the AST
    Parse {
        /// The path to the Asbel file to parse
        #[clap(value_parser)]
        filepath: PathBuf,
    },
    // TODO: Add other commands like 'compile', 'run' later
}

fn main() -> anyhow::Result<()> {
    let cli = AsbelCli::parse();

    match cli.command {
        Commands::Parse { filepath } => {
            handle_parse_command(filepath)?;
        }
    }

    Ok(())
}

fn handle_parse_command(filepath: PathBuf) -> anyhow::Result<()> {
    // Read file content
    let file_content = match std::fs::read_to_string(&filepath) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file {:?}: {}", filepath, e);
            // anyhow::bail! will convert this into an anyhow::Error
            // and propagate it up, exiting main with non-zero.
            anyhow::bail!("Failed to read file: {}", filepath.display());
        }
    };

    // Create Lexer and Parser
    let lexer = asbel_compiler::Lexer::new(&file_content);
    let mut parser = asbel_compiler::Parser::new(lexer);

    // Parse the program
    let program = parser.parse_program();

    // Check for parsing errors
    if !parser.errors.is_empty() {
        eprintln!("Found {} parsing errors in {}:", parser.errors.len(), filepath.display());
        for error in parser.errors {
            eprintln!("  - {}", error);
        }
        // Exit with a non-zero status code to indicate failure
        // Using anyhow::bail! achieves a similar effect by returning an error from main.
        anyhow::bail!("Parsing failed with errors.");
    } else {
        // Print the AST
        println!("{:#?}", program);
    }

    Ok(())
}
