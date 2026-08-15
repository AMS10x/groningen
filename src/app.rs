use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[derive(Debug, Clone)]
pub enum AppEvent {
    Input(KeyEvent),
    Tick,
    TranslationReady(String),
    DownloadProgress { current: u64, total: u64 },
    DownloadComplete(String),
    Error(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActivePane {
    Source,
    Target,
    ExtensionList,
    Settings,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    Normal,
    Editing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeName {
    CatppuccinMocha,
    GroningenLight,
    TerminalClassic,
}

impl ThemeName {
    pub const ALL: [Self; 3] = [
        Self::CatppuccinMocha,
        Self::GroningenLight,
        Self::TerminalClassic,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::CatppuccinMocha => "Catppuccin Mocha",
            Self::GroningenLight => "Groningen Light",
            Self::TerminalClassic => "Terminal Classic",
        }
    }

    pub fn next(self) -> Self {
        let index = Self::ALL
            .iter()
            .position(|theme| *theme == self)
            .unwrap_or(0);
        Self::ALL[(index + 1) % Self::ALL.len()]
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppAction {
    None,
    Quit,
    Translate { source: String, target: String },
    DownloadActiveModel,
}

#[derive(Debug, Clone)]
pub struct DownloadState {
    pub current: u64,
    pub total: u64,
}

#[derive(Debug, Clone)]
pub struct ExtensionItem {
    pub pair: String,
    pub name: String,
    pub installed: bool,
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub input_buffer: String,
    pub output_buffer: String,
    pub active_pane: ActivePane,
    pub input_mode: InputMode,
    pub active_model: Option<String>,
    pub is_translating: bool,
    pub download_progress: Option<DownloadState>,
    pub status_message: String,
    pub should_quit: bool,
    pub theme: ThemeName,
    pub extensions: Vec<ExtensionItem>,
    pub selected_extension: usize,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            input_buffer: String::new(),
            output_buffer: String::new(),
            active_pane: ActivePane::Source,
            input_mode: InputMode::Normal,
            active_model: Some("en-it".to_string()),
            is_translating: false,
            download_progress: None,
            status_message: "Welcome — press e to edit, x to install/activate, t to cycle theme"
                .to_string(),
            should_quit: false,
            theme: ThemeName::CatppuccinMocha,
            extensions: vec![
                ExtensionItem {
                    pair: "en-es".to_string(),
                    name: "English → Spanish".to_string(),
                    installed: true,
                },
                ExtensionItem {
                    pair: "en-it".to_string(),
                    name: "English → Italian".to_string(),
                    installed: true,
                },
                ExtensionItem {
                    pair: "en-de".to_string(),
                    name: "English → German".to_string(),
                    installed: true,
                },
                ExtensionItem {
                    pair: "en-ru".to_string(),
                    name: "English → Russian".to_string(),
                    installed: true,
                },
                ExtensionItem {
                    pair: "en-fr".to_string(),
                    name: "English → French".to_string(),
                    installed: true,
                },
            ],
            selected_extension: 0,
        }
    }
}

impl AppState {
    pub fn mark_installed_models<I, S>(&mut self, installed_pairs: I)
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        for pair in installed_pairs {
            let pair = pair.as_ref();
            if let Some(item) = self.extensions.iter_mut().find(|item| item.pair == pair) {
                item.installed = true;
            }
        }
    }

    pub fn handle_event(&mut self, event: AppEvent) -> AppAction {
        match event {
            AppEvent::Input(key) => self.handle_key(key),
            AppEvent::Tick => AppAction::None,
            AppEvent::TranslationReady(text) => {
                self.output_buffer = text;
                self.is_translating = false;
                self.status_message = "Translation complete".to_string();
                AppAction::None
            }
            AppEvent::DownloadProgress { current, total } => {
                self.download_progress = Some(DownloadState { current, total });
                self.status_message = format!(
                    "Installing extension: {}%",
                    progress_percent(current, total)
                );
                AppAction::None
            }
            AppEvent::DownloadComplete(pair) => {
                self.download_progress = None;
                if let Some(item) = self.extensions.iter_mut().find(|item| item.pair == pair) {
                    item.installed = true;
                }
                self.active_model = Some(pair.clone());
                self.status_message = format!("Installed and activated {pair}");
                AppAction::None
            }
            AppEvent::Error(message) => {
                self.is_translating = false;
                self.download_progress = None;
                self.status_message = format!("Error: {message}");
                AppAction::None
            }
        }
    }

    fn handle_key(&mut self, key: KeyEvent) -> AppAction {
        if key.modifiers.contains(KeyModifiers::CONTROL) && matches!(key.code, KeyCode::Char('c')) {
            self.should_quit = true;
            return AppAction::Quit;
        }
        if key.modifiers.contains(KeyModifiers::CONTROL) && matches!(key.code, KeyCode::Char('d')) {
            return AppAction::DownloadActiveModel;
        }

        match self.input_mode {
            InputMode::Normal => self.handle_normal_key(key),
            InputMode::Editing => self.handle_editing_key(key),
        }
    }

    fn handle_normal_key(&mut self, key: KeyEvent) -> AppAction {
        match key.code {
            KeyCode::Char('q') => {
                self.should_quit = true;
                AppAction::Quit
            }
            KeyCode::Char('x') => self.install_selected_extension(),
            KeyCode::Char('t') => {
                self.theme = self.theme.next();
                self.status_message = format!("Theme set to {}", self.theme.label());
                AppAction::None
            }
            KeyCode::Char('e') => {
                self.input_mode = InputMode::Editing;
                self.active_pane = ActivePane::Source;
                self.status_message =
                    "Editing source — Enter translates, Esc returns to normal".to_string();
                AppAction::None
            }
            KeyCode::Char('c') => {
                self.input_buffer.clear();
                self.output_buffer.clear();
                self.status_message = "Cleared source and translation".to_string();
                AppAction::None
            }
            KeyCode::Tab => {
                self.cycle_pane();
                AppAction::None
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.select_next_extension();
                AppAction::None
            }
            KeyCode::Up | KeyCode::Char('k') => {
                self.select_previous_extension();
                AppAction::None
            }
            KeyCode::Enter => self.request_translation(),
            _ => AppAction::None,
        }
    }

    fn handle_editing_key(&mut self, key: KeyEvent) -> AppAction {
        match key.code {
            KeyCode::Esc => {
                self.input_mode = InputMode::Normal;
                AppAction::None
            }
            KeyCode::Enter => self.request_translation(),
            KeyCode::Backspace => {
                self.input_buffer.pop();
                AppAction::None
            }
            KeyCode::Char(c) => {
                self.input_buffer.push(c);
                AppAction::None
            }
            _ => AppAction::None,
        }
    }

    fn request_translation(&mut self) -> AppAction {
        if self.is_translating {
            self.status_message = "Translation already in progress".to_string();
            return AppAction::None;
        }
        if self.input_buffer.trim().is_empty() {
            self.status_message = "Enter source text before translating".to_string();
            return AppAction::None;
        }
        let Some((source, target)) = self.active_language_pair() else {
            self.status_message = "Select or install a language extension first".to_string();
            return AppAction::None;
        };
        self.is_translating = true;
        self.input_mode = InputMode::Normal;
        self.status_message = format!("Translating {source} → {target}...");
        AppAction::Translate { source, target }
    }

    pub fn active_language_pair(&self) -> Option<(String, String)> {
        let pair = self.active_model.as_deref()?;
        pair.split_once('-')
            .map(|(source, target)| (source.to_string(), target.to_string()))
    }

    pub fn active_language_label(&self) -> String {
        self.active_language_pair()
            .map(|(source, target)| {
                format!(
                    "{} → {}",
                    source.to_ascii_uppercase(),
                    target.to_ascii_uppercase()
                )
            })
            .unwrap_or_else(|| "none".to_string())
    }

    fn install_selected_extension(&mut self) -> AppAction {
        if self.download_progress.is_some() {
            self.status_message = "An extension install is already in progress".to_string();
            return AppAction::None;
        }
        if let Some(item) = self.extensions.get(self.selected_extension) {
            self.active_model = Some(item.pair.clone());
            if item.installed {
                self.status_message = format!("Activated {}", item.name);
                AppAction::None
            } else {
                self.status_message = format!("Installing {}...", item.name);
                AppAction::DownloadActiveModel
            }
        } else {
            self.status_message = "No extension selected".to_string();
            AppAction::None
        }
    }

    fn select_next_extension(&mut self) {
        if !self.extensions.is_empty() {
            self.selected_extension = (self.selected_extension + 1) % self.extensions.len();
        }
    }

    fn select_previous_extension(&mut self) {
        if !self.extensions.is_empty() {
            self.selected_extension = self
                .selected_extension
                .checked_sub(1)
                .unwrap_or(self.extensions.len() - 1);
        }
    }

    fn cycle_pane(&mut self) {
        self.active_pane = match self.active_pane {
            ActivePane::Source => ActivePane::Target,
            ActivePane::Target => ActivePane::ExtensionList,
            ActivePane::ExtensionList => ActivePane::Settings,
            ActivePane::Settings => ActivePane::Source,
        };
    }
}

pub fn progress_percent(current: u64, total: u64) -> u64 {
    if total == 0 {
        0
    } else {
        current.saturating_mul(100) / total
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    fn key(code: KeyCode) -> AppEvent {
        AppEvent::Input(KeyEvent::new(code, KeyModifiers::NONE))
    }

    #[test]
    fn active_language_pair_is_derived_from_active_model() {
        let app = AppState::default();

        assert_eq!(
            app.active_language_pair(),
            Some(("en".to_string(), "it".to_string()))
        );
        assert_eq!(app.active_language_label(), "EN → IT");
    }

    #[test]
    fn mark_installed_models_updates_matching_extensions_only() {
        let mut app = AppState::default();

        app.mark_installed_models(["en-es", "missing-pair"]);

        assert!(app.extensions[0].installed);
        assert!(app.extensions[1].installed);
        assert!(app.extensions[2].installed);
    }

    #[test]
    fn clear_shortcut_empties_source_and_target_buffers() {
        let mut app = AppState {
            input_buffer: "hello".to_string(),
            output_buffer: "ciao".to_string(),
            ..AppState::default()
        };

        let action = app.handle_event(key(KeyCode::Char('c')));

        assert_eq!(action, AppAction::None);
        assert!(app.input_buffer.is_empty());
        assert!(app.output_buffer.is_empty());
        assert_eq!(app.status_message, "Cleared source and translation");
    }

    #[test]
    fn activate_installed_extension_uses_dedicated_key_without_downloading() {
        let mut app = AppState::default();

        let action = app.handle_event(key(KeyCode::Char('x')));

        assert_eq!(action, AppAction::None);
        assert_eq!(app.active_model.as_deref(), Some("en-es"));
        assert_eq!(app.status_message, "Activated English → Spanish");
    }

    #[test]
    fn translation_is_not_reentered_while_in_progress() {
        let mut app = AppState {
            input_buffer: "hello world".to_string(),
            is_translating: true,
            ..AppState::default()
        };

        let action = app.handle_event(key(KeyCode::Enter));

        assert_eq!(action, AppAction::None);
        assert_eq!(app.status_message, "Translation already in progress");
    }

    #[test]
    fn translation_action_carries_active_language_pair() {
        let mut app = AppState {
            input_buffer: "hello world".to_string(),
            active_model: Some("en-de".to_string()),
            ..AppState::default()
        };

        let action = app.handle_event(key(KeyCode::Enter));

        assert_eq!(
            action,
            AppAction::Translate {
                source: "en".to_string(),
                target: "de".to_string()
            }
        );
        assert!(app.is_translating);
        assert_eq!(app.input_mode, InputMode::Normal);
    }
}
