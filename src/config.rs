use clap::Parser;

/// Command line arguments
#[derive(Parser)]
pub struct Args {
    /// Output file for binary
    #[clap(short = 'o', long)]
    pub output: Option<String>,

    /// Path to input
    pub file: Option<String>,

    /// Verbose flag
    #[clap(short = 'v', long, default_value_t = false)]
    pub verbose: bool,

    /// Debug flag
    #[clap(short = 'd', long, default_value_t = false)]
    pub debug: bool,
}

/// Parse command line arguments and return the configuration
pub fn declare_config() -> Args {
    let cli = Args::parse();
    let file = cli.file.unwrap_or_else(|| "main.asm".to_string());
    let output = cli.output.unwrap_or_else(|| "main".to_string());

    Args {
        file: Some(file),
        output: Some(output),
        verbose: cli.verbose,
        debug: cli.debug,
    }
}
