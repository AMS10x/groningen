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

    fn mock_word(word: &str, target_lang: &str) -> Option<&'static str> {
        let lower = word.to_ascii_lowercase();
        match target_lang {
            "es" => match lower.as_str() {
                "computers" => Some("computadoras"),
                "process" => Some("procesan"),
                "data" => Some("datos"),
                "hello" => Some("hola"),
                "world" => Some("mundo"),
                "fast" => Some("rápido"),
                "local" => Some("local"),
                "translation" => Some("traducción"),
                "text" => Some("texto"),
                _ => None,
            },
            "it" => match lower.as_str() {
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
            },
            "de" => match lower.as_str() {
                "computers" => Some("computer"),
                "process" => Some("verarbeiten"),
                "data" => Some("daten"),
                "hello" => Some("hallo"),
                "world" => Some("welt"),
                "fast" => Some("schnell"),
                "local" => Some("lokal"),
                "translation" => Some("übersetzung"),
                "text" => Some("text"),
                _ => None,
            },
            "ru" => match lower.as_str() {
                "computers" => Some("компьютеры"),
                "process" => Some("обрабатывают"),
                "data" => Some("данные"),
                "hello" => Some("привет"),
                "world" => Some("мир"),
                "fast" => Some("быстрый"),
                "local" => Some("локальный"),
                "translation" => Some("перевод"),
                "text" => Some("текст"),
                _ => None,
            },
            "fr" => match lower.as_str() {
                "computers" => Some("ordinateurs"),
                "process" => Some("traitent"),
                "data" => Some("données"),
                "hello" => Some("bonjour"),
                "world" => Some("monde"),
                "fast" => Some("rapide"),
                "local" => Some("local"),
                "translation" => Some("traduction"),
                "text" => Some("texte"),
                _ => None,
            },
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
                    Self::mock_word(word, target_lang).map_or_else(
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
