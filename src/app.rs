use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::manager::registry;

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
    Nord,
    Dracula,
    TokyoNight,
    GruvboxDark,
    SolarizedDark,
    RosePine,
    Cyberpunk,
    TerminalClassic,
}

impl ThemeName {
    pub const ALL: [Self; 10] = [
        Self::CatppuccinMocha,
        Self::GroningenLight,
        Self::Nord,
        Self::Dracula,
        Self::TokyoNight,
        Self::GruvboxDark,
        Self::SolarizedDark,
        Self::RosePine,
        Self::Cyberpunk,
        Self::TerminalClassic,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::CatppuccinMocha => "Catppuccin Mocha",
            Self::GroningenLight => "Groningen Light",
            Self::Nord => "Nord Aurora",
            Self::Dracula => "Dracula",
            Self::TokyoNight => "Tokyo Night",
            Self::GruvboxDark => "Gruvbox Dark",
            Self::SolarizedDark => "Solarized Dark",
            Self::RosePine => "Rosé Pine",
            Self::Cyberpunk => "Cyberpunk Neon",
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
    pub fn previous(self) -> Self {
        let index = Self::ALL
            .iter()
            .position(|theme| *theme == self)
            .unwrap_or(0);
        let previous_index = index.checked_sub(1).unwrap_or(Self::ALL.len() - 1);
        Self::ALL[previous_index]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppAction {
    None,
    Quit,
    Translate,
    DownloadActiveLanguage,
}

#[derive(Debug, Clone)]
pub struct DownloadState {
    pub current: u64,
    pub total: u64,
}

#[derive(Debug, Clone)]
pub struct LanguageItem {
    pub code: String,
    pub english_name: String,
    pub native_name: String,
    pub installed: bool,
}

impl LanguageItem {
    pub fn label(&self) -> String {
        format!("{} / {}", self.english_name, self.native_name)
    }
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub input_buffer: String,
    pub output_buffer: String,
    pub active_pane: ActivePane,
    pub input_mode: InputMode,
    pub source_language: String,
    pub target_language: String,
    pub active_model: Option<String>,
    pub is_translating: bool,
    pub download_progress: Option<DownloadState>,
    pub status_message: String,
    pub should_quit: bool,
    pub theme: ThemeName,
    pub languages: Vec<LanguageItem>,
    pub selected_language: usize,
}

impl Default for AppState {
    fn default() -> Self {
        let languages = registry::bundled_languages()
            .into_iter()
            .map(|language| LanguageItem {
                installed: language.preinstalled,
                code: language.code,
                english_name: language.english_name,
                native_name: language.native_name,
            })
            .collect::<Vec<_>>();
        Self {
            input_buffer: String::new(),
            output_buffer: String::new(),
            active_pane: ActivePane::Source,
            input_mode: InputMode::Normal,
            source_language: "en".to_string(),
            target_language: "nl".to_string(),
            active_model: Some("en-nl".to_string()),
            is_translating: false,
            download_progress: None,
            status_message: "Welcome — Dutch, English, and Russian are pre-installed; focus Languages and press i to install more".to_string(),
            should_quit: false,
            theme: ThemeName::CatppuccinMocha,
            languages,
            selected_language: 0,
        }
    }
}

impl AppState {
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
                self.status_message =
                    format!("Installing language: {}%", progress_percent(current, total));
                AppAction::None
            }
            AppEvent::DownloadComplete(code) => {
                self.download_progress = None;
                if let Some(item) = self.languages.iter_mut().find(|item| item.code == code) {
                    item.installed = true;
                    self.target_language = item.code.clone();
                    self.active_model = Some(self.language_pair());
                    self.status_message = format!("Installed and selected {}", item.label());
                } else {
                    self.status_message = format!("Installed language {code}");
                }
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

    pub fn selected_source_label(&self) -> String {
        self.language_label(&self.source_language)
    }

    pub fn selected_target_label(&self) -> String {
        self.language_label(&self.target_language)
    }

    pub fn language_pair(&self) -> String {
        format!("{}-{}", self.source_language, self.target_language)
    }

    pub fn installed_language_count(&self) -> usize {
        self.languages
            .iter()
            .filter(|language| language.installed)
            .count()
    }

    pub fn total_language_count(&self) -> usize {
        self.languages.len()
    }

    pub fn input_character_count(&self) -> usize {
        self.input_buffer.chars().count()
    }

    pub fn output_character_count(&self) -> usize {
        self.output_buffer.chars().count()
    }

    fn language_label(&self, code: &str) -> String {
        self.languages
            .iter()
            .find(|language| language.code == code)
            .map(LanguageItem::label)
            .unwrap_or_else(|| code.to_string())
    }

    fn handle_key(&mut self, key: KeyEvent) -> AppAction {
        if key.modifiers.contains(KeyModifiers::CONTROL) && matches!(key.code, KeyCode::Char('c')) {
            self.should_quit = true;
            return AppAction::Quit;
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
            KeyCode::Char('i') if self.active_pane == ActivePane::ExtensionList => {
                self.install_or_select_language()
            }
            KeyCode::Char('i') | KeyCode::Char(']') if self.active_pane == ActivePane::Settings => {
                self.theme = self.theme.next();
                self.status_message = format!("Theme set to {}", self.theme.label());
                AppAction::None
            }
            KeyCode::Char('[') if self.active_pane == ActivePane::Settings => {
                self.theme = self.theme.previous();
                self.status_message = format!("Theme set to {}", self.theme.label());
                AppAction::None
            }
            KeyCode::Char('s') if self.active_pane == ActivePane::ExtensionList => {
                self.select_source_language();
                AppAction::None
            }
            KeyCode::Char('i') => {
                self.input_mode = InputMode::Editing;
                self.active_pane = ActivePane::Source;
                AppAction::None
            }
            KeyCode::Tab => {
                self.cycle_pane();
                AppAction::None
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.select_next_language();
                AppAction::None
            }
            KeyCode::Up | KeyCode::Char('k') => {
                self.select_previous_language();
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
        if self.input_buffer.trim().is_empty() {
            return AppAction::None;
        }
        if !self.is_language_installed(&self.source_language)
            || !self.is_language_installed(&self.target_language)
        {
            self.status_message =
                "Install both source and target languages before translating".to_string();
            return AppAction::None;
        }
        self.active_model = Some(self.language_pair());
        self.is_translating = true;
        self.status_message = "Translating...".to_string();
        AppAction::Translate
    }

    fn select_source_language(&mut self) {
        if let Some(item) = self.languages.get(self.selected_language) {
            if item.installed {
                self.source_language = item.code.clone();
                self.active_model = Some(self.language_pair());
                self.status_message = format!("Source language set to {}", item.label());
            } else {
                self.status_message = format!("Install {} before using it as source", item.label());
            }
        }
    }

    fn install_or_select_language(&mut self) -> AppAction {
        if let Some(item) = self.languages.get(self.selected_language) {
            if item.installed {
                self.target_language = item.code.clone();
                self.active_model = Some(self.language_pair());
                self.status_message = format!("Target language set to {}", item.label());
                AppAction::None
            } else {
                self.status_message = format!("Installing {}...", item.label());
                AppAction::DownloadActiveLanguage
            }
        } else {
            AppAction::None
        }
    }

    fn is_language_installed(&self, code: &str) -> bool {
        self.languages
            .iter()
            .any(|language| language.code == code && language.installed)
    }

    fn select_next_language(&mut self) {
        if !self.languages.is_empty() {
            self.selected_language = (self.selected_language + 1) % self.languages.len();
        }
    }

    fn select_previous_language(&mut self) {
        if !self.languages.is_empty() {
            self.selected_language = self
                .selected_language
                .checked_sub(1)
                .unwrap_or(self.languages.len() - 1);
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
