use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
    fs::{File, OpenOptions, create_dir_all},
    io::BufWriter,
    path::PathBuf,
};

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
        tag: Option<String>,
    },

    Get {
        id: u32,
    },

    Pop,

    Remove {
        id: u32,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    let mut config_path: PathBuf = dirs::config_dir().expect("No config directory found");

    config_path.push("snip");
    create_dir_all(&config_path)?;
    config_path.push("snippets.json");

    let mut snippets: Vec<Snippet> = if config_path.exists() && config_path.metadata()?.len() > 0 {
        let reader = File::open(&config_path)?;
        serde_json::from_reader(reader)?
    } else {
        vec![]
    };

    match &cli.command {
        Some(Commands::Add { code, lang, tags }) => {
            let new_snippet = Snippet {
                id: (snippets.iter().map(|s| s.id).max().unwrap_or(0) + 1) as u32,
                code: code.to_owned(),
                lang: lang.to_owned(),
                tags: tags.to_owned().unwrap_or_default(),
            };

            snippets.push(new_snippet);

            let writer = BufWriter::new(File::create(&config_path)?);
            serde_json::to_writer_pretty(writer, &snippets)?;
        }
        Some(Commands::List { tag }) => {
            let matched_snippets: Vec<_> = snippets
                .into_iter()
                .filter(|s| match tag {
                    Some(tag) => s.tags.iter().any(|t| t.contains(tag)),
                    None => true,
                })
                .collect();

            for ele in matched_snippets {
                println!("{}: {}", ele.id, serde_json::to_string_pretty(&ele)?)
            }
        }
        Some(Commands::Get { id }) => {
            if let Some(found_snippet) = snippets.iter().find(|s| s.id.eq(id)) {
                println!("{}", found_snippet.code)
            } else {
                eprintln!("Snippet with ID {id} not found")
            }
        }
        Some(Commands::Pop) => {
            todo!()
        }
        Some(Commands::Remove { id }) => {
            todo!()
        }
        _ => {}
    };

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
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
