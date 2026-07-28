use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionManifest {
    pub language_pair: String,
    pub display_name: String,
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

pub fn bundled_extensions() -> Vec<ExtensionManifest> {
    vec![ExtensionManifest {
        language_pair: "en-it".to_string(),
        display_name: "English → Italian (mock)".to_string(),
        url: "https://example.com/groningen/models/en-it.bin".to_string(),
        checksum: None,
        size_bytes: Some(1_048_576),
    }]
}
