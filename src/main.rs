use std::{fmt::Display, path::PathBuf};

use clap::{CommandFactory, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// adds a new snippet
    Add {
        code: String,
        #[arg(short, long)]
        lang: String,

        #[arg(short, long)]
        tags: Option<Vec<String>>,
    },

    List {
        #[arg(short, long)]
        tag: String,
    },

    Get {
        position: u8,
    },
}

fn main() {
    let cli = Cli::parse();

    if let Some(config_path) = cli.config.as_deref() {
        println!("Value for config: {}", config_path.display())
    }

    match cli.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }

    match &cli.command {
        Some(Commands::Add { code, lang, tags }) => {
            let output = Snippet {
                id: 0,
                code: code.to_owned(),
                lang: lang.to_owned(),
                tags: tags.to_owned().unwrap_or_default(),
            };

            println!("Saved snippet {output}")
        }
        Some(Commands::List { tag }) => {}
        Some(Commands::Get { position }) => {}
        _ => {}
    };
}

#[derive(Debug)]
struct Snippet {
    id: u32,
    code: String,
    lang: String,
    tags: Vec<String>,
}

impl Display for Snippet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ id: {}, code: {}, lang: {}, tags: {:#?} }}",
            self.id, self.code, self.lang, self.tags
        )
    }
}
