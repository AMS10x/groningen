use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::app::{ActivePane, AppState, InputMode, ThemeName};

#[derive(Debug, Clone, Copy)]
struct Theme {
    base: Color,
    text: Color,
    accent: Color,
    success: Color,
    brand: Color,
    surface: Color,
    muted: Color,
    warning: Color,
}

impl Theme {
    fn from_name(name: ThemeName) -> Self {
        match name {
            ThemeName::CatppuccinMocha => Self {
                base: Color::Rgb(30, 30, 46),
                text: Color::Rgb(205, 214, 244),
                accent: Color::Rgb(137, 220, 235),
                success: Color::Rgb(166, 227, 161),
                brand: Color::Rgb(203, 166, 247),
                surface: Color::Rgb(49, 50, 68),
                muted: Color::Rgb(127, 132, 156),
                warning: Color::Rgb(249, 226, 175),
            },
            ThemeName::GroningenLight => Self {
                base: Color::Rgb(246, 248, 250),
                text: Color::Rgb(36, 41, 47),
                accent: Color::Rgb(9, 105, 218),
                success: Color::Rgb(26, 127, 55),
                brand: Color::Rgb(130, 80, 223),
                surface: Color::Rgb(208, 215, 222),
                muted: Color::Rgb(87, 96, 106),
                warning: Color::Rgb(154, 103, 0),
            },
            ThemeName::Nord => Self {
                base: Color::Rgb(46, 52, 64),
                text: Color::Rgb(236, 239, 244),
                accent: Color::Rgb(136, 192, 208),
                success: Color::Rgb(163, 190, 140),
                brand: Color::Rgb(129, 161, 193),
                surface: Color::Rgb(67, 76, 94),
                muted: Color::Rgb(216, 222, 233),
                warning: Color::Rgb(235, 203, 139),
            },
            ThemeName::Dracula => Self {
                base: Color::Rgb(40, 42, 54),
                text: Color::Rgb(248, 248, 242),
                accent: Color::Rgb(139, 233, 253),
                success: Color::Rgb(80, 250, 123),
                brand: Color::Rgb(189, 147, 249),
                surface: Color::Rgb(68, 71, 90),
                muted: Color::Rgb(98, 114, 164),
                warning: Color::Rgb(241, 250, 140),
            },
            ThemeName::TokyoNight => Self {
                base: Color::Rgb(26, 27, 38),
                text: Color::Rgb(192, 202, 245),
                accent: Color::Rgb(125, 207, 255),
                success: Color::Rgb(158, 206, 106),
                brand: Color::Rgb(187, 154, 247),
                surface: Color::Rgb(36, 40, 59),
                muted: Color::Rgb(86, 95, 137),
                warning: Color::Rgb(224, 175, 104),
            },
            ThemeName::GruvboxDark => Self {
                base: Color::Rgb(40, 40, 40),
                text: Color::Rgb(235, 219, 178),
                accent: Color::Rgb(131, 165, 152),
                success: Color::Rgb(184, 187, 38),
                brand: Color::Rgb(211, 134, 155),
                surface: Color::Rgb(60, 56, 54),
                muted: Color::Rgb(146, 131, 116),
                warning: Color::Rgb(250, 189, 47),
            },
            ThemeName::SolarizedDark => Self {
                base: Color::Rgb(0, 43, 54),
                text: Color::Rgb(131, 148, 150),
                accent: Color::Rgb(42, 161, 152),
                success: Color::Rgb(133, 153, 0),
                brand: Color::Rgb(108, 113, 196),
                surface: Color::Rgb(7, 54, 66),
                muted: Color::Rgb(88, 110, 117),
                warning: Color::Rgb(181, 137, 0),
            },
            ThemeName::RosePine => Self {
                base: Color::Rgb(25, 23, 36),
                text: Color::Rgb(224, 222, 244),
                accent: Color::Rgb(156, 207, 216),
                success: Color::Rgb(49, 116, 143),
                brand: Color::Rgb(196, 167, 231),
                surface: Color::Rgb(38, 35, 58),
                muted: Color::Rgb(110, 106, 134),
                warning: Color::Rgb(246, 193, 119),
            },
            ThemeName::Cyberpunk => Self {
                base: Color::Rgb(10, 10, 18),
                text: Color::Rgb(236, 252, 255),
                accent: Color::Rgb(0, 245, 255),
                success: Color::Rgb(57, 255, 20),
                brand: Color::Rgb(255, 0, 128),
                surface: Color::Rgb(28, 24, 48),
                muted: Color::Rgb(145, 120, 190),
                warning: Color::Rgb(255, 230, 0),
            },
            ThemeName::TerminalClassic => Self {
                base: Color::Black,
                text: Color::White,
                accent: Color::Cyan,
                success: Color::Green,
                brand: Color::Blue,
                surface: Color::DarkGray,
                muted: Color::Gray,
                warning: Color::Yellow,
            },
        }
    }
}

pub fn draw(frame: &mut Frame<'_>, app: &AppState) {
    let theme = Theme::from_name(app.theme);
    let area = frame.size();
    let root = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(10),
            Constraint::Length(3),
        ])
        .split(area);
    draw_top_bar(frame, root[0], app, theme);
    draw_workspace(frame, root[1], app, theme);
    draw_drawer(frame, root[2], app, theme);
    draw_status(frame, root[3], app, theme);
}

fn draw_top_bar(frame: &mut Frame<'_>, area: Rect, app: &AppState, theme: Theme) {
    let model = format!(
        "{} ➔ {}",
        app.source_language.to_uppercase(),
        app.target_language.to_uppercase()
    );
    let line = Line::from(vec![
        Span::styled(
            " GRONINGEN ",
            Style::default()
                .fg(theme.base)
                .bg(theme.brand)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("  local translator  "),
        Span::styled(
            format!("[Active: {model}]"),
            Style::default().fg(theme.accent),
        ),
        Span::styled(
            format!("  Theme: {}", app.theme.label()),
            Style::default().fg(theme.muted),
        ),
        Span::styled(
            format!(
                "  Languages: {}/{}",
                app.installed_language_count(),
                app.total_language_count()
            ),
            Style::default().fg(theme.success),
        ),
    ]);
    frame.render_widget(
        Paragraph::new(line).block(Block::default().borders(Borders::ALL).bg(theme.base)),
        area,
    );
}

fn draw_workspace(frame: &mut Frame<'_>, area: Rect, app: &AppState, theme: Theme) {
    let panes = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let source_border = focused(app, ActivePane::Source, theme);
    let target_border = focused(app, ActivePane::Target, theme);
    let source = Paragraph::new(if app.input_buffer.is_empty() {
        "Type source text here. Press i to edit."
    } else {
        app.input_buffer.as_str()
    })
    .style(Style::default().fg(theme.text).bg(theme.base))
    .block(
        Block::default()
            .title(format!(
                " Source Text [{}] · {} chars ",
                app.selected_source_label(),
                app.input_character_count()
            ))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(source_border)),
    )
    .wrap(Wrap { trim: false });
    frame.render_widget(source, panes[0]);

    let output = if app.is_translating {
        "⠋ Translating locally..."
    } else if app.output_buffer.is_empty() {
        "Your translation will appear here."
    } else {
        app.output_buffer.as_str()
    };
    let target = Paragraph::new(output)
        .style(Style::default().fg(theme.success).bg(theme.base))
        .block(
            Block::default()
                .title(format!(
                    " Translation [{}] · {} chars ",
                    app.selected_target_label(),
                    app.output_character_count()
                ))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(target_border)),
        )
        .wrap(Wrap { trim: false });
    frame.render_widget(target, panes[1]);

    if app.input_mode == InputMode::Editing && app.active_pane == ActivePane::Source {
        let x = panes[0]
            .x
            .saturating_add(1)
            .saturating_add(app.input_buffer.len() as u16)
            .min(panes[0].right().saturating_sub(2));
        frame.set_cursor(x, panes[0].y.saturating_add(1));
    }
}

fn draw_drawer(frame: &mut Frame<'_>, area: Rect, app: &AppState, theme: Theme) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(62), Constraint::Percentage(38)])
        .split(area);

    let items = app.languages.iter().enumerate().map(|(index, language)| {
        let marker = if index == app.selected_language {
            "›"
        } else {
            " "
        };
        let status = if language.code == app.source_language {
            "source"
        } else if language.code == app.target_language {
            "target"
        } else if language.installed {
            "installed"
        } else {
            "available"
        };
        let style = if index == app.selected_language {
            Style::default()
                .fg(theme.accent)
                .add_modifier(Modifier::BOLD)
        } else if language.code == app.source_language || language.code == app.target_language {
            Style::default().fg(theme.warning)
        } else if language.installed {
            Style::default().fg(theme.success)
        } else {
            Style::default().fg(theme.text)
        };
        ListItem::new(Line::from(vec![
            Span::raw(format!("{marker} ")),
            Span::styled(format!("{} ", language.code), style),
            Span::styled(format!("{} ", language.label()), style),
            Span::styled(format!("({status})"), Style::default().fg(theme.muted)),
        ]))
    });
    frame.render_widget(
        List::new(items.collect::<Vec<_>>()).block(
            Block::default()
                .title(" Languages — press i to install/select inside the app ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(focused(app, ActivePane::ExtensionList, theme))),
        ),
        chunks[0],
    );

    let settings = Paragraph::new(format!(
        "Theme Studio\n  {}\n\n[i] or Right bracket: next theme\nLeft bracket: previous theme\n\nLanguage actions\n  i = install/select target\n  s = set installed language as source\n\nActive pair\n  {} → {}",
        app.theme.label(),
        app.selected_source_label(),
        app.selected_target_label()
    ))
    .style(Style::default().fg(theme.text).bg(theme.base))
    .block(
        Block::default()
            .title(" Settings ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(focused(app, ActivePane::Settings, theme))),
    );
    frame.render_widget(settings, chunks[1]);
}

fn draw_status(frame: &mut Frame<'_>, area: Rect, app: &AppState, theme: Theme) {
    if let Some(progress) = &app.download_progress {
        let ratio = if progress.total == 0 {
            0.0
        } else {
            progress.current as f64 / progress.total as f64
        };
        let gauge = Gauge::default()
            .block(
                Block::default()
                    .title(app.status_message.as_str())
                    .borders(Borders::ALL),
            )
            .gauge_style(Style::default().fg(theme.accent).bg(theme.surface))
            .ratio(ratio.clamp(0.0, 1.0));
        frame.render_widget(gauge, area);
        return;
    }

    let mode = match app.input_mode {
        InputMode::Normal => "NORMAL",
        InputMode::Editing => "EDIT",
    };
    let keys = "[i] Edit/Install/Theme+ | [s] Source | [/] Theme +/- | [Tab] Pane | [↑↓/jk] Language | [Enter] Translate | [q] Quit";
    let line = format!(" {mode} | {keys} | {}", app.status_message);
    frame.render_widget(
        Paragraph::new(line)
            .alignment(Alignment::Center)
            .style(Style::default().fg(theme.text).bg(theme.surface)),
        area,
    );
}

fn focused(app: &AppState, pane: ActivePane, theme: Theme) -> Color {
    if app.active_pane == pane {
        theme.accent
    } else {
        theme.surface
    }
}
