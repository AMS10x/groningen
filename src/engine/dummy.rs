use std::collections::HashMap;
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

    fn get_translation_map(target_lang: &str) -> HashMap<&'static str, &'static str> {
        match target_lang {
            "es" => HashMap::from([
                ("computers", "computadoras"),
                ("process", "procesan"),
                ("data", "datos"),
                ("hello", "hola"),
                ("world", "mundo"),
                ("fast", "rápido"),
                ("local", "local"),
                ("translation", "traducción"),
                ("text", "texto"),
                ("i", "yo"),
                ("you", "tú"),
                ("he", "él"),
                ("she", "ella"),
                ("we", "nosotros"),
                ("they", "ellos"),
                ("the", "el"),
                ("a", "un"),
                ("an", "un"),
                ("and", "y"),
                ("or", "o"),
                ("but", "pero"),
                ("in", "en"),
                ("on", "en"),
                ("at", "en"),
                ("to", "a"),
                ("for", "para"),
                ("of", "de"),
                ("is", "es"),
                ("are", "son"),
                ("was", "era"),
                ("were", "eran"),
                ("be", "ser"),
                ("have", "tener"),
                ("has", "tiene"),
                ("do", "hacer"),
                ("does", "hace"),
                ("can", "poder"),
                ("will", "va a"),
                ("this", "esto"),
                ("that", "eso"),
                ("it", "ello"),
                ("good", "bueno"),
                ("great", "genial"),
                ("new", "nuevo"),
                ("old", "viejo"),
                ("big", "grande"),
                ("small", "pequeño"),
            ]),
            "it" => HashMap::from([
                ("computers", "computer"),
                ("process", "elaborano"),
                ("data", "dati"),
                ("hello", "ciao"),
                ("world", "mondo"),
                ("fast", "veloce"),
                ("local", "locale"),
                ("translation", "traduzione"),
                ("text", "testo"),
                ("i", "io"),
                ("you", "tu"),
                ("he", "lui"),
                ("she", "lei"),
                ("we", "noi"),
                ("they", "loro"),
                ("the", "il"),
                ("a", "un"),
                ("an", "un"),
                ("and", "e"),
                ("or", "o"),
                ("but", "ma"),
                ("in", "in"),
                ("on", "su"),
                ("at", "a"),
                ("to", "a"),
                ("for", "per"),
                ("of", "di"),
                ("is", "è"),
                ("are", "sono"),
                ("was", "era"),
                ("were", "erano"),
                ("be", "essere"),
                ("have", "avere"),
                ("has", "ha"),
                ("do", "fare"),
                ("does", "fa"),
                ("can", "potere"),
                ("will", "volere"),
                ("this", "questo"),
                ("that", "quello"),
                ("it", "esso"),
                ("good", "buono"),
                ("great", "grandioso"),
                ("new", "nuovo"),
                ("old", "vecchio"),
                ("big", "grande"),
                ("small", "piccolo"),
            ]),
            "de" => HashMap::from([
                ("computers", "computer"),
                ("process", "verarbeiten"),
                ("data", "daten"),
                ("hello", "hallo"),
                ("world", "welt"),
                ("fast", "schnell"),
                ("local", "lokal"),
                ("translation", "übersetzung"),
                ("text", "text"),
                ("i", "ich"),
                ("you", "du"),
                ("he", "er"),
                ("she", "sie"),
                ("we", "wir"),
                ("they", "sie"),
                ("the", "der"),
                ("a", "ein"),
                ("an", "ein"),
                ("and", "und"),
                ("or", "oder"),
                ("but", "aber"),
                ("in", "in"),
                ("on", "auf"),
                ("at", "bei"),
                ("to", "zu"),
                ("for", "für"),
                ("of", "von"),
                ("is", "ist"),
                ("are", "sind"),
                ("was", "war"),
                ("were", "waren"),
                ("be", "sein"),
                ("have", "haben"),
                ("has", "hat"),
                ("do", "tun"),
                ("does", "tut"),
                ("can", "können"),
                ("will", "werden"),
                ("this", "dies"),
                ("that", "das"),
                ("it", "es"),
                ("good", "gut"),
                ("great", "großartig"),
                ("new", "neu"),
                ("old", "alt"),
                ("big", "groß"),
                ("small", "klein"),
            ]),
            "ru" => HashMap::from([
                ("computers", "компьютеры"),
                ("process", "обрабатывают"),
                ("data", "данные"),
                ("hello", "привет"),
                ("world", "мир"),
                ("fast", "быстрый"),
                ("local", "локальный"),
                ("translation", "перевод"),
                ("text", "текст"),
                ("i", "я"),
                ("you", "ты"),
                ("he", "он"),
                ("she", "она"),
                ("we", "мы"),
                ("they", "они"),
                ("the", ""),
                ("a", ""),
                ("an", ""),
                ("and", "и"),
                ("or", "или"),
                ("but", "но"),
                ("in", "в"),
                ("on", "на"),
                ("at", "в"),
                ("to", ""),
                ("for", "для"),
                ("of", ""),
                ("is", "это"),
                ("are", "это"),
                ("was", "был"),
                ("were", "были"),
                ("be", "быть"),
                ("have", "иметь"),
                ("has", "имеет"),
                ("do", "делать"),
                ("does", "делает"),
                ("can", "мочь"),
                ("will", "будет"),
                ("this", "этот"),
                ("that", "тот"),
                ("it", "оно"),
                ("good", "хороший"),
                ("great", "отличный"),
                ("new", "новый"),
                ("old", "старый"),
                ("big", "большой"),
                ("small", "маленький"),
            ]),
            "fr" => HashMap::from([
                ("computers", "ordinateurs"),
                ("process", "traitent"),
                ("data", "données"),
                ("hello", "bonjour"),
                ("world", "monde"),
                ("fast", "rapide"),
                ("local", "local"),
                ("translation", "traduction"),
                ("text", "texte"),
                ("i", "je"),
                ("you", "tu"),
                ("he", "il"),
                ("she", "elle"),
                ("we", "nous"),
                ("they", "ils"),
                ("the", "le"),
                ("a", "un"),
                ("an", "un"),
                ("and", "et"),
                ("or", "ou"),
                ("but", "mais"),
                ("in", "dans"),
                ("on", "sur"),
                ("at", "à"),
                ("to", "à"),
                ("for", "pour"),
                ("of", "de"),
                ("is", "est"),
                ("are", "sont"),
                ("was", "était"),
                ("were", "étaient"),
                ("be", "être"),
                ("have", "avoir"),
                ("has", "a"),
                ("do", "faire"),
                ("does", "fait"),
                ("can", "pouvoir"),
                ("will", "va"),
                ("this", "ce"),
                ("that", "ce"),
                ("it", "il"),
                ("good", "bon"),
                ("great", "génial"),
                ("new", "nouveau"),
                ("old", "vieux"),
                ("big", "grand"),
                ("small", "petit"),
            ]),
            _ => HashMap::new(),
        }
    }

    fn translate_text(text: &str, target_lang: &str) -> String {
        let vocab = Self::get_translation_map(target_lang);
        let mut result = String::new();
        let mut current_word = String::new();

        for c in text.chars() {
            if c.is_alphabetic() {
                current_word.push(c.to_ascii_lowercase());
            } else {
                if !current_word.is_empty() {
                    if let Some(&translation) = vocab.get(current_word.as_str()) {
                        result.push_str(translation);
                    } else {
                        result.push_str(&current_word);
                    }
                    current_word.clear();
                }
                result.push(c);
            }
        }

        if !current_word.is_empty() {
            if let Some(&translation) = vocab.get(current_word.as_str()) {
                result.push_str(translation);
            } else {
                result.push_str(&current_word);
            }
        }

        result
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
            let translated = Self::translate_text(text, target_lang);
            Ok(format!("[{source_lang}->{target_lang}] {translated}"))
        })
    }

    fn is_model_installed(&self, language_pair: &str) -> bool {
        self.model_dir
            .join(format!("{language_pair}.bin"))
            .is_file()
    }
}
