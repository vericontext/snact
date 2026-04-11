mod cmd;
mod validate;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "snact",
    about = "AI agent-optimized browser CLI — snap + act",
    version,
    after_long_help = AGENT_GUIDE
)]
struct Cli {
    /// Chrome debugging port
    #[arg(long, default_value = "9222", global = true)]
    port: u16,

    /// Output format: text, json, or ndjson (auto-detects json when piped)
    #[arg(long, global = true)]
    output: Option<String>,

    /// Preview what would happen without executing
    #[arg(long, global = true)]
    dry_run: bool,

    /// Browser language for Accept-Language header (e.g. en-US, ko, ja)
    #[arg(long, default_value = "en-US", global = true)]
    lang: String,

    /// Verbose logging
    #[arg(long, global = true)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Extract interactable elements as @eN references. Always snap before acting.
    /// Output: @e1 [role] "label" attrs — one element per line
    Snap {
        /// URL to navigate to (optional if already on a page)
        url: Option<String>,

        /// CSS selector to limit extraction scope (e.g. "main", "#content")
        #[arg(long)]
        focus: Option<String>,
    },

    /// Read visible text content as structured markdown. Use when you need to understand
    /// page content (lists, tables, headings) — not just interactable elements.
    Read {
        /// URL to navigate to (optional if already on a page)
        url: Option<String>,

        /// CSS selector to limit reading scope (e.g. "main", "#content", ".pr-list")
        #[arg(long)]
        focus: Option<String>,

        /// Maximum number of lines to return [default: 200]
        #[arg(long, default_value = "200")]
        max_lines: usize,
    },

    /// Click an element by @eN reference from snap output.
    /// Returns updated page snapshot automatically (use --no-snap to disable).
    Click {
        /// Element reference from snap (e.g. @e1)
        #[arg(name = "ref")]
        element_ref: String,

        /// Skip automatic re-snap after action
        #[arg(long)]
        no_snap: bool,
    },

    /// Set an input field's value (clears existing). Use for <input>, <textarea>.
    /// Returns updated page snapshot automatically.
    Fill {
        /// Element reference from snap (e.g. @e2)
        #[arg(name = "ref")]
        element_ref: String,

        /// Value to set
        value: String,

        /// Skip automatic re-snap after action
        #[arg(long)]
        no_snap: bool,
    },

    /// Type text character by character with key events. Use for autocomplete/search.
    /// Returns updated page snapshot automatically.
    Type {
        /// Element reference from snap (e.g. @e2)
        #[arg(name = "ref")]
        element_ref: String,

        /// Text to type
        text: String,

        /// Skip automatic re-snap after action
        #[arg(long)]
        no_snap: bool,
    },

    /// Select an option in a <select> dropdown by value.
    /// Returns updated page snapshot automatically.
    Select {
        /// Element reference from snap (e.g. @e3)
        #[arg(name = "ref")]
        element_ref: String,

        /// Option value to select
        value: String,

        /// Skip automatic re-snap after action
        #[arg(long)]
        no_snap: bool,
    },

    /// Scroll the page in a direction.
    /// Returns updated page snapshot automatically.
    Scroll {
        /// Direction: up, down, left, right
        #[arg(default_value = "down")]
        direction: String,

        /// Pixels to scroll [default: 400]
        #[arg(long)]
        amount: Option<i64>,

        /// Skip automatic re-snap after action
        #[arg(long)]
        no_snap: bool,
    },

    /// Capture a PNG screenshot of the current page
    Screenshot {
        /// Output file path [default: stdout as base64 in JSON mode]
        #[arg(short, long = "file")]
        file: Option<String>,
    },

    /// Wait for navigation, a CSS selector to appear, or a timeout in ms
    Wait {
        /// "navigation" | CSS selector | milliseconds (e.g. "2000")
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

    /// Start an MCP server exposing snact tools over JSON-RPC (stdio)
    Mcp,

    /// Show JSON Schema for a command's inputs and outputs
    Schema {
        /// Command name (e.g. snap, click, fill). Omit for full schema.
        command: Option<String>,
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
    /// Launch Chrome with remote debugging. Use --background for agent use
    Launch {
        /// Run in headless mode
        #[arg(long)]
        headless: bool,

        /// Run in background (detach immediately, use "browser stop" to terminate)
        #[arg(long)]
        background: bool,
    },
    /// Stop a running Chrome instance
    Stop,
    /// Check if Chrome is running
    Status,
}

const AGENT_GUIDE: &str = "\
WORKFLOW (for AI agents):
  1. snact browser launch --background  # start Chrome (auto-detaches)
  2. snact snap <url>                   # get interactable elements as @eN refs
  3. snact read                         # read visible text content (headings, lists, tables)
  4. snact click/fill/type @eN          # act on elements by reference
  5. snact snap                         # re-snap to see updated state
  6. snact browser stop                 # stop Chrome when done

SNAP vs READ:
  snap   → interactable elements only (buttons, links, inputs) — use before acting
  read   → visible text content (headings, paragraphs, lists, tables) — use to understand content
  Both support --focus=<selector> to scope to a page section.

ELEMENT REFERENCES:
  snap output: @e1 [button] \"Sign In\" id=\"submit\"
  use @e1 in: snact click @e1 / snact fill @e1 \"value\"
  refs persist on disk — valid until next snap

OUTPUT MODES:
  terminal  → human-readable text (default)
  piped     → JSON auto-detected (snact snap url | jq .)
  explicit  → --output=json for forced JSON

EXAMPLES:
  snact snap https://github.com/login       # list login form elements
  snact fill @e2 \"user\" && snact fill @e3 \"pass\" && snact click @e5
  snact read https://github.com/pulls       # read PR list as structured text
  snact read --focus=\"main\"                 # read only the main section
  snact screenshot --file=page.png          # capture current page
  snact session save mysite                 # persist cookies/storage
  snact session load mysite                 # restore session later
  snact --lang=ko snap https://google.com   # Korean content

SCHEMA INTROSPECTION:
  snact schema                           # full JSON Schema for all commands
  snact schema snap                      # schema for a specific command

MCP SERVER:
  snact mcp                              # start JSON-RPC server over stdio
  Add to claude_desktop_config.json:
    {\"mcpServers\":{\"snact\":{\"command\":\"snact\",\"args\":[\"mcp\"]}}}

SAFETY:
  --dry-run on any mutation shows what would execute without acting
  element refs are validated (rejects invalid formats)
  CSS selectors are validated (rejects javascript: protocol)";

/// Resolved output format after TTY detection.
fn resolve_output_format(explicit: Option<&str>) -> &str {
    match explicit {
        Some(fmt) => fmt,
        None => {
            // Default to text — contextual snap output is most useful for LLMs.
            // Use --output=json explicitly when you need structured JSON.
            "text"
        }
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let cli = Cli::parse();

    if cli.verbose {
        tracing_subscriber_init();
    }

    let fmt = resolve_output_format(cli.output.as_deref()).to_owned();

    if let Err(e) = dispatch(cli, &fmt).await {
        emit_error(&e, &fmt);
        std::process::exit(1);
    }
}

fn emit_error(err: &anyhow::Error, fmt: &str) {
    let msg = err.to_string();
    let code = if msg.contains("Cannot connect")
        || msg.contains("BrowserNotFound")
        || msg.contains("Chrome")
    {
        "BROWSER_NOT_CONNECTED"
    } else if msg.contains("not found") || msg.contains("No such") || msg.contains("does not exist")
    {
        "NOT_FOUND"
    } else if msg.contains("Invalid") || msg.contains("invalid") || msg.contains("Unknown") {
        "INVALID_INPUT"
    } else if msg.contains("timeout") || msg.contains("Timeout") {
        "TIMEOUT"
    } else {
        "ERROR"
    };

    if fmt == "json" {
        eprintln!(
            "{}",
            serde_json::json!({"error": {"code": code, "message": msg}})
        );
    } else {
        eprintln!("error: {msg}");
    }
}

async fn dispatch(cli: Cli, fmt: &str) -> anyhow::Result<()> {
    match cli.command {
        Commands::Snap { url, focus } => {
            if let Some(f) = &focus {
                validate::css_selector(f)?;
            }
            cmd::snap::run(cli.port, url.as_deref(), focus.as_deref(), fmt, &cli.lang).await?;
        }
        Commands::Read {
            url,
            focus,
            max_lines,
        } => {
            if let Some(f) = &focus {
                validate::css_selector(f)?;
            }
            cmd::read::run(
                cli.port,
                url.as_deref(),
                focus.as_deref(),
                fmt,
                &cli.lang,
                max_lines,
            )
            .await?;
        }
        Commands::Click {
            element_ref,
            no_snap,
        } => {
            validate::element_ref(&element_ref)?;
            cmd::action::run_click(cli.port, &element_ref, fmt, cli.dry_run, no_snap, &cli.lang)
                .await?;
        }
        Commands::Fill {
            element_ref,
            value,
            no_snap,
        } => {
            validate::element_ref(&element_ref)?;
            validate::fill_value(&value)?;
            cmd::action::run_fill(
                cli.port,
                &element_ref,
                &value,
                fmt,
                cli.dry_run,
                no_snap,
                &cli.lang,
            )
            .await?;
        }
        Commands::Type {
            element_ref,
            text,
            no_snap,
        } => {
            validate::element_ref(&element_ref)?;
            validate::fill_value(&text)?;
            cmd::action::run_type(
                cli.port,
                &element_ref,
                &text,
                fmt,
                cli.dry_run,
                no_snap,
                &cli.lang,
            )
            .await?;
        }
        Commands::Select {
            element_ref,
            value,
            no_snap,
        } => {
            validate::element_ref(&element_ref)?;
            cmd::action::run_select(
                cli.port,
                &element_ref,
                &value,
                fmt,
                cli.dry_run,
                no_snap,
                &cli.lang,
            )
            .await?;
        }
        Commands::Scroll {
            direction,
            amount,
            no_snap,
        } => {
            cmd::action::run_scroll(
                cli.port,
                &direction,
                amount,
                fmt,
                cli.dry_run,
                no_snap,
                &cli.lang,
            )
            .await?;
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
            BrowserAction::Launch {
                headless,
                background,
            } => {
                cmd::browser::run_launch(cli.port, headless, background, fmt)?;
            }
            BrowserAction::Stop => {
                cmd::browser::run_stop(cli.port, fmt)?;
            }
            BrowserAction::Status => {
                cmd::browser::run_status(cli.port, fmt)?;
            }
        },
        Commands::Mcp => {
            cmd::mcp::run(cli.port).await?;
        }
        Commands::Schema { command } => {
            cmd::schema::run(command.as_deref(), fmt);
        }
    }

    Ok(())
}

fn tracing_subscriber_init() {
    use tracing_subscriber::fmt;
    let _ = fmt::try_init();
}
