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

    loop {
        terminal
            .terminal_mut()
            .draw(|frame| tui::ui::draw(frame, &app))?;
        let Some(event) = rx.recv().await else {
            break;
        };
        match app.handle_event(event) {
            AppAction::Quit => break,
            AppAction::Translate => spawn_translation(&app, engine.clone(), tx.clone()),
            AppAction::DownloadActiveLanguage => spawn_download(&app, tx.clone()),
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
) {
    let text = app.input_buffer.clone();
    let source = app.source_language.clone();
    let target = app.target_language.clone();
    tokio::spawn(async move {
        let event = match engine.translate(&text, &source, &target).await {
            Ok(output) => AppEvent::TranslationReady(output),
            Err(err) => AppEvent::Error(err.to_string()),
        };
        if tx.send(event).await.is_err() {}
    });
}

fn spawn_download(app: &AppState, tx: mpsc::Sender<AppEvent>) {
    let code = app
        .languages
        .get(app.selected_language)
        .map(|language| language.code.clone())
        .unwrap_or_else(|| app.target_language.clone());
    tokio::spawn(async move {
        if let Err(err) = manager::download_extension(&code, tx.clone()).await {
            if tx.send(AppEvent::Error(err.to_string())).await.is_err() {}
        }
    });
}
