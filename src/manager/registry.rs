use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionManifest {
    pub language_pair: String,
    pub display_name: String,
    pub url: String,
    pub checksum: Option<String>,
    pub size_bytes: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageManifest {
    pub code: String,
    pub english_name: String,
    pub native_name: String,
    pub preinstalled: bool,
    pub url: String,
    pub checksum: Option<String>,
    pub size_bytes: Option<u64>,
}

pub async fn fetch_remote_index(url: &str) -> anyhow::Result<Vec<ExtensionManifest>> {
    let manifests = reqwest::get(url)
        .await?
        .error_for_status()?
        .json::<Vec<ExtensionManifest>>()
        .await?;
    Ok(manifests)
}

pub fn bundled_languages() -> Vec<LanguageManifest> {
    language_specs()
        .into_iter()
        .map(
            |(code, english_name, native_name, preinstalled)| LanguageManifest {
                code: code.to_string(),
                english_name: english_name.to_string(),
                native_name: native_name.to_string(),
                preinstalled,
                url: format!("https://example.com/groningen/languages/{code}.bin"),
                checksum: None,
                size_bytes: Some(1_048_576),
            },
        )
        .collect()
}

pub fn bundled_extensions() -> Vec<ExtensionManifest> {
    bundled_languages()
        .into_iter()
        .map(|language| ExtensionManifest {
            language_pair: language.code.clone(),
            display_name: format!("{} / {}", language.english_name, language.native_name),
            url: language.url,
            checksum: language.checksum,
            size_bytes: language.size_bytes,
        })
        .collect()
}

fn language_specs() -> Vec<(&'static str, &'static str, &'static str, bool)> {
    vec![
        ("nl", "Dutch", "Nederlands", true),
        ("en", "English", "English", true),
        ("ru", "Russian", "Русский", true),
        ("af", "Afrikaans", "Afrikaans", false),
        ("ar", "Arabic", "العربية", false),
        ("az", "Azerbaijani", "Azərbaycanca", false),
        ("be", "Belarusian", "Беларуская", false),
        ("bg", "Bulgarian", "Български", false),
        ("bn", "Bengali", "বাংলা", false),
        ("bs", "Bosnian", "Bosanski", false),
        ("ca", "Catalan", "Català", false),
        ("cs", "Czech", "Čeština", false),
        ("cy", "Welsh", "Cymraeg", false),
        ("da", "Danish", "Dansk", false),
        ("de", "German", "Deutsch", false),
        ("el", "Greek", "Ελληνικά", false),
        ("es", "Spanish", "Español", false),
        ("et", "Estonian", "Eesti", false),
        ("fa", "Persian", "فارسی", false),
        ("fi", "Finnish", "Suomi", false),
        ("fr", "French", "Français", false),
        ("ga", "Irish", "Gaeilge", false),
        ("he", "Hebrew", "עברית", false),
        ("hi", "Hindi", "हिन्दी", false),
        ("hr", "Croatian", "Hrvatski", false),
        ("hu", "Hungarian", "Magyar", false),
        ("hy", "Armenian", "Հայերեն", false),
        ("id", "Indonesian", "Bahasa Indonesia", false),
        ("is", "Icelandic", "Íslenska", false),
        ("it", "Italian", "Italiano", false),
        ("ja", "Japanese", "日本語", false),
        ("ka", "Georgian", "ქართული", false),
        ("kk", "Kazakh", "Қазақша", false),
        ("ko", "Korean", "한국어", false),
        ("lt", "Lithuanian", "Lietuvių", false),
        ("lv", "Latvian", "Latviešu", false),
        ("mk", "Macedonian", "Македонски", false),
        ("ms", "Malay", "Bahasa Melayu", false),
        ("no", "Norwegian", "Norsk", false),
        ("pl", "Polish", "Polski", false),
        ("pt", "Portuguese", "Português", false),
        ("ro", "Romanian", "Română", false),
        ("sk", "Slovak", "Slovenčina", false),
        ("sl", "Slovenian", "Slovenščina", false),
        ("sq", "Albanian", "Shqip", false),
        ("sr", "Serbian", "Српски", false),
        ("sv", "Swedish", "Svenska", false),
        ("sw", "Swahili", "Kiswahili", false),
        ("th", "Thai", "ไทย", false),
        ("tr", "Turkish", "Türkçe", false),
        ("uk", "Ukrainian", "Українська", false),
        ("ur", "Urdu", "اردو", false),
        ("vi", "Vietnamese", "Tiếng Việt", false),
        ("zh", "Chinese", "中文", false),
    ]
}
