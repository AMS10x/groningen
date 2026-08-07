mod app;
mod cli;
mod engine;
mod manager;
mod tui;

use std::sync::Arc;

use app::{AppAction, AppEvent, AppState};
use cli::Cli;
use engine::{dummy::DummyEngine, TranslationEngine};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse_args();
    if cli::handle_cli(&cli).await? {
        return Ok(());
    }
    run_tui().await
}

async fn run_tui() -> anyhow::Result<()> {
    manager::ensure_config_dirs().await?;
    let mut terminal = tui::TerminalGuard::enter()?;
    let (tx, mut rx) = mpsc::channel::<AppEvent>(256);
    tokio::spawn(tui::events::listen(tx.clone()));

    let engine = Arc::new(DummyEngine::new(manager::models_dir()?));
    let mut app = AppState::default();
    let installed_pairs = app
        .extensions
        .iter()
        .filter(|item| engine.is_model_installed(&item.pair))
        .map(|item| item.pair.clone())
        .collect::<Vec<_>>();
    app.mark_installed_models(installed_pairs);

    loop {
        terminal
            .terminal_mut()
            .draw(|frame| tui::ui::draw(frame, &app))?;
        let Some(event) = rx.recv().await else {
            break;
        };
        match app.handle_event(event) {
            AppAction::Quit => break,
            AppAction::Translate { source, target } => {
                spawn_translation(&app, engine.clone(), tx.clone(), source, target)
            }
            AppAction::DownloadActiveModel => spawn_download(&app, tx.clone()),
            AppAction::None => {}
        }
        if app.should_quit {
            break;
        }
    }
    Ok(())
}

fn spawn_translation(
    app: &AppState,
    engine: Arc<dyn TranslationEngine>,
    tx: mpsc::Sender<AppEvent>,
    source: String,
    target: String,
) {
    let text = app.input_buffer.clone();
    tokio::spawn(async move {
        let event = match engine.translate(&text, &source, &target).await {
            Ok(output) => AppEvent::TranslationReady(output),
            Err(err) => AppEvent::Error(err.to_string()),
        };
        if tx.send(event).await.is_err() {}
    });
}

fn spawn_download(app: &AppState, tx: mpsc::Sender<AppEvent>) {
    let pair = app
        .active_model
        .clone()
        .unwrap_or_else(|| "en-it".to_string());
    tokio::spawn(async move {
        if let Err(err) = manager::download_extension(&pair, tx.clone()).await {
            if tx.send(AppEvent::Error(err.to_string())).await.is_err() {}
        }
    });
}
