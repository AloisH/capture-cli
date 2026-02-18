mod meta;
mod start;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "capture", version, about = "Capture and retrieve output of long-running processes by name")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start a named capture
    Start {
        /// Name of the capture
        #[arg(short, long)]
        name: String,
        /// Command to run
        command: Vec<String>,
    },
    /// Retrieve captured output
    Logs {
        /// Name of the capture
        name: String,
        /// Last N lines
        #[arg(short, long)]
        lines: Option<usize>,
        /// First N lines
        #[arg(long)]
        head: Option<usize>,
        /// Filter by pattern
        #[arg(short, long)]
        grep: Option<String>,
        /// Follow output in real-time
        #[arg(short, long)]
        follow: bool,
        /// Show stderr only
        #[arg(long)]
        stderr: bool,
    },
    /// List active captures
    List,
    /// Stop a capture
    Stop {
        /// Name of the capture
        name: Option<String>,
        /// Stop all captures
        #[arg(long)]
        all: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Start { name, command } => {
            start::run(&name, &command);
        }
        Commands::Logs { name, lines, head, grep, follow, stderr } => {
            println!("logs: {name}");
        }
        Commands::List => {
            println!("list");
        }
        Commands::Stop { name, all } => {
            println!("stop");
        }
    }
}
