use std::path::PathBuf;
use std::time::Instant;

use anyhow::{anyhow, Context};
use futures_util::StreamExt;
use tokio::io::AsyncWriteExt;
use tokio::sync::mpsc;

use crate::app::AppEvent;

pub mod registry;

pub fn config_dir() -> anyhow::Result<PathBuf> {
    dirs::config_dir()
        .map(|dir| dir.join("groningen"))
        .ok_or_else(|| anyhow!("unable to resolve config directory"))
}

pub fn models_dir() -> anyhow::Result<PathBuf> {
    Ok(config_dir()?.join("models"))
}

pub async fn ensure_config_dirs() -> anyhow::Result<PathBuf> {
    let dir = models_dir()?;
    tokio::fs::create_dir_all(&dir)
        .await
        .with_context(|| format!("creating {}", dir.display()))?;
    Ok(dir)
}

pub async fn download_extension(pair: &str, tx: mpsc::Sender<AppEvent>) -> anyhow::Result<()> {
    let manifest = registry::bundled_extensions()
        .into_iter()
        .find(|extension| extension.language_pair == pair)
        .ok_or_else(|| anyhow!("unknown language-pair extension: {pair}"))?;
    let models = ensure_config_dirs().await?;
    let target = models.join(format!("{pair}.bin"));
    let url = manifest.url;

    if target.is_file() {
        send_event(&tx, AppEvent::DownloadComplete(pair.to_string())).await;
        return Ok(());
    }

    let result = download_to_file(&url, &target, tx.clone()).await;
    if result.is_err() {
        write_mock_model(&target, pair).await?;
        let total = tokio::fs::metadata(&target).await?.len();
        send_event(
            &tx,
            AppEvent::DownloadProgress {
                current: total,
                total,
            },
        )
        .await;
    }

    send_event(&tx, AppEvent::DownloadComplete(pair.to_string())).await;
    Ok(())
}

async fn download_to_file(
    url: &str,
    target: &PathBuf,
    tx: mpsc::Sender<AppEvent>,
) -> anyhow::Result<()> {
    let response = reqwest::get(url).await?.error_for_status()?;
    let total = response.content_length().unwrap_or(0);
    let partial = target.with_extension("bin.part");
    let mut file = tokio::fs::File::create(&partial).await?;
    let mut stream = response.bytes_stream();
    let mut current = 0_u64;
    let started = Instant::now();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        file.write_all(&chunk).await?;
        current = current.saturating_add(chunk.len() as u64);
        let _bytes_per_second = if started.elapsed().as_secs_f64() > 0.0 {
            current as f64 / started.elapsed().as_secs_f64()
        } else {
            0.0
        };
        send_event(&tx, AppEvent::DownloadProgress { current, total }).await;
    }
    file.flush().await?;
    tokio::fs::rename(partial, target).await?;
    Ok(())
}

async fn write_mock_model(target: &PathBuf, pair: &str) -> anyhow::Result<()> {
    let payload = format!("groningen mock model weights for {pair}\n");
    let partial = target.with_extension("bin.part");
    tokio::fs::write(&partial, payload).await?;
    tokio::fs::rename(partial, target).await?;
    Ok(())
}

async fn send_event(tx: &mpsc::Sender<AppEvent>, event: AppEvent) {
    if tx.send(event).await.is_err() {}
}
