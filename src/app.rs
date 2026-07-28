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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppAction {
    None,
    Quit,
    Translate,
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
            status_message: "Welcome — press i to edit source or install the selected extension"
                .to_string(),
            should_quit: false,
            theme: ThemeName::CatppuccinMocha,
            extensions: vec![
                ExtensionItem {
                    pair: "en-it".to_string(),
                    name: "English → Italian".to_string(),
                    installed: false,
                },
                ExtensionItem {
                    pair: "en-es".to_string(),
                    name: "English → Spanish".to_string(),
                    installed: false,
                },
                ExtensionItem {
                    pair: "en-de".to_string(),
                    name: "English → German".to_string(),
                    installed: false,
                },
            ],
            selected_extension: 0,
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
            KeyCode::Char('i') if self.active_pane == ActivePane::ExtensionList => {
                self.install_selected_extension()
            }
            KeyCode::Char('i') if self.active_pane == ActivePane::Settings => {
                self.theme = self.theme.next();
                self.status_message = format!("Theme set to {}", self.theme.label());
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
        if self.input_buffer.trim().is_empty() {
            return AppAction::None;
        }
        self.is_translating = true;
        self.status_message = "Translating...".to_string();
        AppAction::Translate
    }

    fn install_selected_extension(&mut self) -> AppAction {
        if let Some(item) = self.extensions.get(self.selected_extension) {
            self.active_model = Some(item.pair.clone());
            self.status_message = format!("Installing {}...", item.name);
            AppAction::DownloadActiveModel
        } else {
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
