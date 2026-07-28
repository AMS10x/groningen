use std::future::Future;
use std::pin::Pin;

pub mod dummy;

pub type TranslationFuture<'a> = Pin<Box<dyn Future<Output = anyhow::Result<String>> + Send + 'a>>;

pub trait TranslationEngine: Send + Sync {
    fn translate<'a>(
        &'a self,
        text: &'a str,
        source_lang: &'a str,
        target_lang: &'a str,
    ) -> TranslationFuture<'a>;
    fn is_model_installed(&self, language_pair: &str) -> bool;
}
