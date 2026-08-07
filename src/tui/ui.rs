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
            },
            ThemeName::GroningenLight => Self {
                base: Color::Rgb(246, 248, 250),
                text: Color::Rgb(36, 41, 47),
                accent: Color::Rgb(9, 105, 218),
                success: Color::Rgb(26, 127, 55),
                brand: Color::Rgb(130, 80, 223),
                surface: Color::Rgb(208, 215, 222),
                muted: Color::Rgb(87, 96, 106),
            },
            ThemeName::TerminalClassic => Self {
                base: Color::Black,
                text: Color::White,
                accent: Color::Cyan,
                success: Color::Green,
                brand: Color::Blue,
                surface: Color::DarkGray,
                muted: Color::Gray,
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
            Constraint::Length(8),
            Constraint::Length(3),
        ])
        .split(area);
    draw_top_bar(frame, root[0], app, theme);
    draw_workspace(frame, root[1], app, theme);
    draw_drawer(frame, root[2], app, theme);
    draw_status(frame, root[3], app, theme);
}

fn draw_top_bar(frame: &mut Frame<'_>, area: Rect, app: &AppState, theme: Theme) {
    let model = app.active_language_label();
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
    ]);
    frame.render_widget(
        Paragraph::new(line).block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().bg(theme.base)),
        ),
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
    let source = Paragraph::new(app.input_buffer.as_str())
        .style(Style::default().fg(theme.text).bg(theme.base))
        .block(
            Block::default()
                .title(" Source Text ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(source_border)),
        )
        .wrap(Wrap { trim: false });
    frame.render_widget(source, panes[0]);

    let output = if app.is_translating {
        "Translating..."
    } else if app.output_buffer.is_empty() {
        "Your translation will appear here."
    } else {
        app.output_buffer.as_str()
    };
    let target = Paragraph::new(output)
        .style(Style::default().fg(theme.success).bg(theme.base))
        .block(
            Block::default()
                .title(" Translation ")
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

    let items = app.extensions.iter().enumerate().map(|(index, extension)| {
        let marker = if index == app.selected_extension {
            "›"
        } else {
            " "
        };
        let status = if extension.installed {
            "installed"
        } else {
            "available"
        };
        let style = if index == app.selected_extension {
            Style::default()
                .fg(theme.accent)
                .add_modifier(Modifier::BOLD)
        } else if extension.installed {
            Style::default().fg(theme.success)
        } else {
            Style::default().fg(theme.text)
        };
        ListItem::new(Line::from(vec![
            Span::raw(format!("{marker} ")),
            Span::styled(format!("{} ", extension.pair), style),
            Span::styled(format!("{} ", extension.name), style),
            Span::styled(format!("({status})"), Style::default().fg(theme.muted)),
        ]))
    });
    frame.render_widget(
        List::new(items.collect::<Vec<_>>()).block(
            Block::default()
                .title(" Extensions — click/press i to install like VS Code or LazyVim ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(focused(app, ActivePane::ExtensionList, theme))),
        ),
        chunks[0],
    );

    let settings = Paragraph::new(format!(
        "Theme\n  {}\n\nActive model\n  {}\n\nPress i while this pane is focused to cycle theme.",
        app.theme.label(),
        app.active_language_label()
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
    let keys = "[i] Edit/Install/Theme | [c] Clear | [Esc] Normal | [Tab] Pane | [↑↓/jk] Select Extension | [Enter] Translate | [q] Quit";
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
