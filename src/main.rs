use clap::Parser;
use ctrlrs::app::App;
use ctrlrs::config::Config;
use ctrlrs::ui::ui::*;
use ctrlrs::Result;
use std::fs::File;
use std::io::Write;
use tempfile::NamedTempFile;

/// Enhanced Ctrl-R for shell history with n-dimensional search (up to 5 dimensions)
/// 
/// The `ctrlrs` tool provides a powerful alternative to the standard Ctrl-R
/// reverse history search in shells, with support for multiple nested filters.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Enable debug logging
    #[clap(short, long)]
    debug: bool,

    /// Specify shell type (auto-detected if not specified)
    #[clap(short, long)]
    shell: Option<String>,

    /// Specify history file path (auto-detected if not specified)
    #[clap(short = 'f', long)]
    history_file: Option<String>,
    
    /// Specify output file path for the selected command
    #[clap(short = 'o', long)]
    output_file: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Initialize logging only if debug flag is set
    if args.debug {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug"))
            .target(env_logger::Target::Stderr)
            .init();
        
        log::info!("Starting ctrlrs");
    } else {
        // Disable logging
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("off"))
            .target(env_logger::Target::Stderr)
            .init();
    }
    
    // Load configuration
    let config = Config::new(args.shell.clone(), args.history_file.clone())?;
    
    if args.debug {
        log::info!("Configuration loaded: {:?}", config);
    }

    // Setup terminal
    let mut terminal = setup_terminal()?;

    // Create app state
    let mut app = App::new(config)?;

    // Run the application
    let res = run_app(&mut terminal, &mut app);

    // Get the selected command
    let selected_command = app.selected_command();

    // Restore terminal
    restore_terminal(&mut terminal)?;

    // Handle application result
    if let Err(err) = res {
        eprintln!("Error: {}", err);
        return Err(err);
    }

    // Write the selected command to the specified output file or a temporary file
    if let Some(cmd) = selected_command {
        if let Some(output_path) = &args.output_file {
            // Write to the specified output file
            match std::fs::File::create(output_path) {
                Ok(mut file) => {
                    if let Err(err) = writeln!(file, "{}", cmd) {
                        eprintln!("Error writing to output file: {}", err);
                        return Err(ctrlrs::error::Error::Other(format!("Failed to write to output file: {}", err)));
                    }
                },
                Err(err) => {
                    eprintln!("Error creating output file: {}", err);
                    return Err(ctrlrs::error::Error::Other(format!("Failed to create output file: {}", err)));
                }
            }
        } else {
            // If no output file is specified, print to stdout (fallback for backward compatibility)
            println!("{}", cmd);
        }
    }

    Ok(())
}
