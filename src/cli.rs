use std::io::{self, Read};

use clap::{Parser, Subcommand};
use indicatif::{ProgressBar, ProgressStyle};
use tokio::sync::mpsc;

use crate::app::AppEvent;
use crate::engine::{dummy::DummyEngine, TranslationEngine};
use crate::manager::{self, registry};

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    #[arg(short = 't', long = "to", default_value = "it")]
    pub target_lang: String,

    #[arg(short = 's', long = "source", default_value = "en")]
    pub source_lang: String,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Install a language-pair model extension.
    Install {
        /// Language pair to install, such as en-it.
        pair: String,
    },
    /// List available language-pair extensions.
    List,
}

impl Cli {
    pub fn parse_args() -> Self {
        Self::parse()
    }
}

pub async fn handle_cli(cli: &Cli) -> anyhow::Result<bool> {
    match &cli.command {
        Some(Commands::Install { pair }) => {
            install_with_progress(pair).await?;
            Ok(true)
        }
        Some(Commands::List) => {
            print_extension_list().await?;
            Ok(true)
        }
        None if stdin_has_data() => {
            let mut input = String::new();
            io::stdin().read_to_string(&mut input)?;
            let engine = DummyEngine::new(manager::ensure_config_dirs().await?);
            let output = engine
                .translate(input.trim(), &cli.source_lang, &cli.target_lang)
                .await?;
            println!("{output}");
            Ok(true)
        }
        None => Ok(false),
    }
}

async fn install_with_progress(pair: &str) -> anyhow::Result<()> {
    let (tx, mut rx) = mpsc::channel(64);
    let progress = ProgressBar::new(100);
    progress.set_style(ProgressStyle::with_template(
        "{spinner:.cyan} {msg} [{bar:40.cyan/blue}] {pos:>3}%",
    )?);
    let pair_owned = pair.to_string();
    let task = tokio::spawn(async move { manager::download_extension(&pair_owned, tx).await });

    while let Some(event) = rx.recv().await {
        match event {
            AppEvent::DownloadProgress { current, total } => {
                let percent = crate::app::progress_percent(current, total);
                progress.set_position(percent);
                progress.set_message(format!("downloading {pair}"));
            }
            AppEvent::DownloadComplete(done) => {
                progress.finish_with_message(format!("installed {done}"));
                break;
            }
            AppEvent::Error(message) => progress.println(format!("error: {message}")),
            _ => {}
        }
    }
    task.await??;
    Ok(())
}

async fn print_extension_list() -> anyhow::Result<()> {
    let model_dir = manager::ensure_config_dirs().await?;
    println!("PAIR\tSTATUS\tNAME");
    for extension in registry::bundled_extensions() {
        let status = if model_dir
            .join(format!("{}.bin", extension.language_pair))
            .is_file()
        {
            "installed"
        } else {
            "preinstalled"
        };
        println!(
            "{}\t{}\t{}",
            extension.language_pair, status, extension.display_name
        );
    }
    Ok(())
}

fn stdin_has_data() -> bool {
    use std::io::IsTerminal;
    !io::stdin().is_terminal()
}
