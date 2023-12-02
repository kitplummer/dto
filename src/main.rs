use anyhow::Context;
use clap::{Parser, Subcommand, ValueEnum};
use colored::Colorize;
use dittolive_ditto::{identity::*, prelude::*};
use rustyline::completion::FilenameCompleter;
use rustyline::error::ReadlineError;
use rustyline::highlight::Highlighter;
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser as SqlParser;
//use rustyline::hint::HistoryHinter;
use rustyline::validate::MatchingBracketValidator;
use rustyline::{Config, Editor};
use std::path::PathBuf;
use std::{self, str::FromStr, sync::Arc};
use std::{fmt, process};

use rustyline::{Completer, Helper, Hinter, Validator};

mod collection;
mod config;
mod observe;
mod presence;
mod query;
mod repl;
mod subscription;
mod utils;

use std::borrow::Cow::{self, Borrowed};

//use rustyline::error::ReadlineError;

#[derive(Parser)]
#[command(name = "dto")]
#[command(author = "Ditto")]
#[command(version = "0.0.1")]
#[command(
    about = "Interact with a Ditto database.",
    long_about = "A simple command-line interface for interacting with a local Ditto database"
)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

// For use by the Presence command
#[derive(clap::Parser)]
struct Args {
    #[clap(value_enum)]
    scope: PeerScope,
}
#[derive(ValueEnum, Clone, Copy, Debug)]
enum PeerScope {
    All,
    Local,
    Remote,
}
impl fmt::Display for PeerScope {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PeerScope::All => write!(f, "all"),
            PeerScope::Local => write!(f, "local"),
            PeerScope::Remote => write!(f, "remote"),
        }
    }
}

#[derive(Subcommand)]
enum Commands {
    // /// Simple REPL interface for interacting with Ditto data
    // Repl {},
    /// View Ditto's local collections for your App ID
    Collections {},
    /// Create or show dto's configuration file
    Configure {
        /// Create configuration file with details at ~/.config/dto.toml
        #[arg(short, long)]
        create: bool,
        /// Show default configuration file contents
        #[arg(short, long)]
        show: bool,
    },
    /// Execute a single synchronous query for a given configured APP ID
    Execute {
        /// SQL query to be executed
        #[arg(short, long)]
        query: String,
    },
    /// Observe data on a specific query for a given configured APP ID
    Observe {
        /// SQL query for data to be observed
        #[arg(short, long)]
        query: String,
    },
    /// Observe local and remote peer metadata
    Presence {
        /// See all, local or remote metadata
        #[arg(short, long)]
        scope: PeerScope,
    },
    /// Ditto database utilities and information
    Utils {
        /// Get current local store resources used
        #[arg(short, long)]
        storage: bool,
    },
}

#[derive(Helper, Completer, Hinter, Validator)]
pub struct MyHelper {
    colored_prompt: String,
    #[rustyline(Completer)]
    completer: FilenameCompleter,
    //highlighter: MatchingBracketHighlighter,
    #[rustyline(Validator)]
    validator: MatchingBracketValidator,
}

impl Highlighter for MyHelper {
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        default: bool,
    ) -> Cow<'b, str> {
        if default {
            Borrowed(&self.colored_prompt)
        } else {
            Borrowed(prompt)
        }
    }
}

pub fn start_repl(ditto: Ditto) -> anyhow::Result<()> {
    let h = MyHelper {
        completer: FilenameCompleter::new(),
        //highlighter: MatchingBracketHighlighter::new(),
        colored_prompt: "".to_owned(),
        validator: MatchingBracketValidator::new(),
    };
    let config = Config::builder().history_ignore_space(true).build();
    let mut rl = Editor::with_config(config)?;
    //let mut rl = Editor::with_config(config)?;
    rl.set_helper(Some(h));

    rl.helper_mut().expect("No helper").colored_prompt = format!("{}", ">> ".blue().bold());

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                let _ = rl.add_history_entry(line.as_str());
                let _ = repl::parse(line, &ditto);
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                process::exit(0x0100);
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                process::exit(0x0100);
            }
            Err(err) => {
                println!("Error: {:?}", err);
                process::exit(0x0100);
            }
        }
    }
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let config;
    if let Some(config_path) = cli.config.as_deref() {
        println!("Value for config: {}", config_path.display());
        config = config::read_config(config_path);
    } else {
        let path_buf = config::find_default_config_file();
        config = config::read_config(path_buf.unwrap().as_path());
    }

    // match cli.debug {
    //     0 => println!("Debug mode is off"),
    //     1 => println!("Debug mode is on 1"),
    //     2 => println!("Debug mode is on 2"),
    //     _ => println!("Debug mode is wacko"),
    // }

    let ditto: Ditto = Ditto::builder()
        .with_root(Arc::new(PersistentRoot::from_current_exe()?))
        .with_minimum_log_level(LogLevel::Error)
        .with_identity(|ditto_root| {
            let app_id = AppId::from_str(&config.app_id).unwrap();
            let shared_token = config.license_token;
            let enable_cloud_sync = true;
            let custom_auth_url = None;
            OnlinePlayground::new(
                ditto_root,
                app_id,
                shared_token,
                enable_cloud_sync,
                custom_auth_url,
            )
        })?
        .build()?;

    ditto.set_device_name("dto-cli");
    ditto.small_peer_info().set_enabled(true);

    let _sync = ditto.start_sync().context("unable to start sync")?;

    let store = ditto.store();

    match &cli.command {
        // Pausing dev on the REPL until have working inserts from execute command
        // Some(Commands::Repl {}) => {
        //     let _repl = start_repl(ditto);
        // }
        Some(Commands::Collections {}) => {
            collection::list_collections(store);
        }
        Some(Commands::Configure { create, show }) => {
            if *create {
                if cli.verbose > 0 {
                    println!(
                        "Create default configuration template and prompt for App ID and token."
                    )
                }
                config::create_configuration_file();
            } else if *show {
                if cli.verbose > 0 {
                    println!("Show current configuration file contents.");
                }
                config::show_configuration_file();
            } else {
                if config::config_file_exists() {
                    println!("Configuration file exists at ~/.config/dto.toml");
                }
            }
        }
        Some(Commands::Execute { query }) => {
            // evaluate SQL
            let dialect = GenericDialect {};
            let _ast = match SqlParser::parse_sql(&dialect, query.to_owned()) {
                Ok(_statement) => {
                    query::query(query.to_owned(), &ditto);
                }
                Err(e) => {
                    println!("not a valid SQL query {:#?}", e);
                }
            };
        }
        Some(Commands::Observe { query }) => {
            // evaluate SQL
            let dialect = GenericDialect {};
            let _ast = match SqlParser::parse_sql(&dialect, query.to_owned()) {
                Ok(_statement) => {
                    // TODO: set this up to output only with verbosity flag
                    // println!("have a valid SQL: {:#?}", statement);
                    observe::observe(query.to_owned(), &ditto);
                }
                Err(e) => {
                    println!("not a valid SQL query {:#?}", e);
                }
            };
        }
        Some(Commands::Presence { scope }) => match scope {
            PeerScope::All => {
                presence::presence(PeerScope::All.to_string(), &ditto);
            }
            PeerScope::Local => {
                presence::presence(PeerScope::Local.to_string(), &ditto);
            }
            PeerScope::Remote => {
                presence::presence(PeerScope::Remote.to_string(), &ditto);
            }
        },
        Some(Commands::Utils { storage }) => {
            if *storage {
                utils::show_storage_resources_used(store);
            }
        }
        None => {
            let _repl = start_repl(ditto);
        }
    };
    Ok(())
}
