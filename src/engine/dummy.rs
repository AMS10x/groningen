use std::path::PathBuf;
use std::time::Duration;

use super::{TranslationEngine, TranslationFuture};

#[derive(Debug, Clone)]
pub struct DummyEngine {
    model_dir: PathBuf,
}

impl DummyEngine {
    pub fn new(model_dir: PathBuf) -> Self {
        Self { model_dir }
    }

    fn mock_italian(word: &str) -> Option<&'static str> {
        match word.to_ascii_lowercase().as_str() {
            "computers" => Some("computer"),
            "process" => Some("elaborano"),
            "data" => Some("dati"),
            "hello" => Some("ciao"),
            "world" => Some("mondo"),
            "fast" => Some("veloce"),
            "local" => Some("locale"),
            "translation" => Some("traduzione"),
            "text" => Some("testo"),
            _ => None,
        }
    }
}

impl TranslationEngine for DummyEngine {
    fn translate<'a>(
        &'a self,
        text: &'a str,
        source_lang: &'a str,
        target_lang: &'a str,
    ) -> TranslationFuture<'a> {
        Box::pin(async move {
            tokio::time::sleep(Duration::from_millis(200)).await;
            let translated = text
                .split_whitespace()
                .map(|word| {
                    Self::mock_italian(word).map_or_else(
                        || word.chars().rev().collect::<String>(),
                        ToString::to_string,
                    )
                })
                .collect::<Vec<_>>()
                .join(" ");
            Ok(format!("[{source_lang}→{target_lang}] {translated}"))
        })
    }

    fn is_model_installed(&self, language_pair: &str) -> bool {
        self.model_dir
            .join(format!("{language_pair}.bin"))
            .is_file()
    }
}
