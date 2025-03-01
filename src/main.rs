use clap::Parser;
use ctrlrs::app::App;
use ctrlrs::config::Config;
use ctrlrs::ui::ui::*;
use ctrlrs::Result;

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

    // Print the selected command to stdout
    if let Some(cmd) = selected_command {
        println!("{}", cmd);
    }

    Ok(())
}
