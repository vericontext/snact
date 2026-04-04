mod cmd;
mod validate;

use clap::{Parser, Subcommand};
use std::io::IsTerminal;

#[derive(Parser)]
#[command(
    name = "snact",
    about = "AI agent-optimized browser CLI — snap + act",
    version
)]
struct Cli {
    /// Chrome debugging port
    #[arg(long, default_value = "9222", global = true)]
    port: u16,

    /// Output format: text or json (auto-detects json when piped)
    #[arg(long, global = true)]
    output: Option<String>,

    /// Preview what would happen without executing
    #[arg(long, global = true)]
    dry_run: bool,

    /// Verbose logging
    #[arg(long, global = true)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Take a smart snapshot of the current page
    Snap {
        /// URL to navigate to (optional if already on a page)
        url: Option<String>,

        /// CSS selector to focus extraction on
        #[arg(long)]
        focus: Option<String>,
    },

    /// Click an element
    Click {
        /// Element reference (e.g., @e1)
        #[arg(name = "ref")]
        element_ref: String,
    },

    /// Fill an input field
    Fill {
        /// Element reference (e.g., @e2)
        #[arg(name = "ref")]
        element_ref: String,

        /// Value to fill
        value: String,
    },

    /// Type text character by character
    Type {
        /// Element reference (e.g., @e2)
        #[arg(name = "ref")]
        element_ref: String,

        /// Text to type
        text: String,
    },

    /// Select an option in a <select> element
    Select {
        /// Element reference (e.g., @e3)
        #[arg(name = "ref")]
        element_ref: String,

        /// Value to select
        value: String,
    },

    /// Scroll the page
    Scroll {
        /// Direction: up, down, left, right
        #[arg(default_value = "down")]
        direction: String,

        /// Pixels to scroll
        #[arg(long)]
        amount: Option<i64>,
    },

    /// Take a screenshot
    Screenshot {
        /// Output file path
        #[arg(short, long = "file")]
        file: Option<String>,
    },

    /// Wait for a condition
    Wait {
        /// Condition: "navigation", a CSS selector, or milliseconds
        condition: String,
    },

    /// Session management
    Session {
        #[command(subcommand)]
        action: SessionAction,
    },

    /// Record a workflow
    Record {
        #[command(subcommand)]
        action: RecordAction,
    },

    /// Replay a recorded workflow
    Replay {
        /// Workflow name
        name: String,

        /// Speed multiplier (1.0 = original speed, 0 = instant)
        #[arg(long, default_value = "0")]
        speed: f64,
    },

    /// Launch Chrome with remote debugging enabled
    Browser {
        #[command(subcommand)]
        action: BrowserAction,
    },
}

#[derive(Subcommand)]
enum SessionAction {
    /// Save current browser session
    Save { name: String },
    /// Restore a saved session
    Load { name: String },
    /// List saved sessions
    List,
    /// Delete a saved session
    Delete { name: String },
}

#[derive(Subcommand)]
enum RecordAction {
    /// Start recording
    Start {
        /// Recording name (optional, auto-generated if not provided)
        name: Option<String>,
    },
    /// Stop recording and save
    Stop,
    /// List recorded workflows
    List,
    /// Delete a recorded workflow
    Delete { name: String },
}

#[derive(Subcommand)]
enum BrowserAction {
    /// Launch Chrome with remote debugging
    Launch {
        /// Run in headless mode
        #[arg(long)]
        headless: bool,
    },
}

/// Resolved output format after TTY detection.
fn resolve_output_format(explicit: Option<&str>) -> &str {
    match explicit {
        Some(fmt) => fmt,
        None => {
            // Auto-detect: use JSON when stdout is not a terminal (piped to agent)
            if std::io::stdout().is_terminal() {
                "text"
            } else {
                "json"
            }
        }
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if cli.verbose {
        tracing_subscriber_init();
    }

    let fmt = resolve_output_format(cli.output.as_deref());

    match cli.command {
        Commands::Snap { url, focus } => {
            if let Some(f) = &focus {
                validate::css_selector(f)?;
            }
            cmd::snap::run(cli.port, url.as_deref(), focus.as_deref(), fmt).await?;
        }
        Commands::Click { element_ref } => {
            validate::element_ref(&element_ref)?;
            cmd::action::run_click(cli.port, &element_ref, fmt, cli.dry_run).await?;
        }
        Commands::Fill { element_ref, value } => {
            validate::element_ref(&element_ref)?;
            cmd::action::run_fill(cli.port, &element_ref, &value, fmt, cli.dry_run).await?;
        }
        Commands::Type { element_ref, text } => {
            validate::element_ref(&element_ref)?;
            cmd::action::run_type(cli.port, &element_ref, &text, fmt, cli.dry_run).await?;
        }
        Commands::Select { element_ref, value } => {
            validate::element_ref(&element_ref)?;
            cmd::action::run_select(cli.port, &element_ref, &value, fmt, cli.dry_run).await?;
        }
        Commands::Scroll { direction, amount } => {
            cmd::action::run_scroll(cli.port, &direction, amount, fmt, cli.dry_run).await?;
        }
        Commands::Screenshot { file } => {
            cmd::action::run_screenshot(cli.port, file.as_deref(), fmt).await?;
        }
        Commands::Wait { condition } => {
            if condition != "navigation" && condition.parse::<u64>().is_err() {
                validate::css_selector(&condition)?;
            }
            cmd::action::run_wait(cli.port, &condition, fmt).await?;
        }
        Commands::Session { action } => match action {
            SessionAction::Save { name } => {
                cmd::session::run_save(cli.port, &name, fmt).await?;
            }
            SessionAction::Load { name } => {
                cmd::session::run_load(cli.port, &name, fmt).await?;
            }
            SessionAction::List => {
                cmd::session::run_list(fmt)?;
            }
            SessionAction::Delete { name } => {
                cmd::session::run_delete(&name, fmt)?;
            }
        },
        Commands::Record { action } => match action {
            RecordAction::Start { name } => {
                cmd::record::run_start(name.as_deref(), fmt)?;
            }
            RecordAction::Stop => {
                cmd::record::run_stop(fmt)?;
            }
            RecordAction::List => {
                cmd::record::run_list(fmt)?;
            }
            RecordAction::Delete { name } => {
                cmd::record::run_delete(&name, fmt)?;
            }
        },
        Commands::Replay { name, speed } => {
            cmd::replay::run(cli.port, &name, speed, fmt, cli.dry_run).await?;
        }
        Commands::Browser { action } => match action {
            BrowserAction::Launch { headless } => {
                cmd::browser::run_launch(cli.port, headless)?;
            }
        },
    }

    Ok(())
}

fn tracing_subscriber_init() {
    use tracing_subscriber::fmt;
    let _ = fmt::try_init();
}
