use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Paragraph, Wrap},
    Frame,
};

use crate::app::{ActivePane, AppState, InputMode};

const MOCHA_BASE: Color = Color::Rgb(30, 30, 46);
const MOCHA_TEXT: Color = Color::Rgb(205, 214, 244);
const MOCHA_CYAN: Color = Color::Rgb(137, 220, 235);
const MOCHA_GREEN: Color = Color::Rgb(166, 227, 161);
const MOCHA_MAUVE: Color = Color::Rgb(203, 166, 247);
const MOCHA_SURFACE: Color = Color::Rgb(49, 50, 68);

pub fn draw(frame: &mut Frame<'_>, app: &AppState) {
    let area = frame.size();
    let root = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(3),
        ])
        .split(area);
    draw_top_bar(frame, root[0], app);
    draw_workspace(frame, root[1], app);
    draw_status(frame, root[2], app);
}

fn draw_top_bar(frame: &mut Frame<'_>, area: Rect, app: &AppState) {
    let model = app
        .active_model
        .as_deref()
        .unwrap_or("none")
        .to_ascii_uppercase()
        .replace('-', " ➔ ");
    let line = Line::from(vec![
        Span::styled(
            " GRONINGEN ",
            Style::default()
                .fg(MOCHA_BASE)
                .bg(MOCHA_MAUVE)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("  v0.1.0  "),
        Span::styled(
            format!("[Active: {model}]"),
            Style::default().fg(MOCHA_CYAN),
        ),
    ]);
    frame.render_widget(
        Paragraph::new(line).block(Block::default().borders(Borders::ALL).bg(MOCHA_BASE)),
        area,
    );
}

fn draw_workspace(frame: &mut Frame<'_>, area: Rect, app: &AppState) {
    let panes = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let source_border = if app.active_pane == ActivePane::Source {
        MOCHA_CYAN
    } else {
        MOCHA_SURFACE
    };
    let target_border = if app.active_pane == ActivePane::Target {
        MOCHA_CYAN
    } else {
        MOCHA_SURFACE
    };
    let source = Paragraph::new(app.input_buffer.as_str())
        .style(Style::default().fg(MOCHA_TEXT).bg(MOCHA_BASE))
        .block(
            Block::default()
                .title(" Source Text [English] ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(source_border)),
        )
        .wrap(Wrap { trim: false });
    frame.render_widget(source, panes[0]);

    let output = if app.is_translating {
        "Translating..."
    } else {
        app.output_buffer.as_str()
    };
    let target = Paragraph::new(output)
        .style(Style::default().fg(MOCHA_GREEN).bg(MOCHA_BASE))
        .block(
            Block::default()
                .title(" Translation [Italian] ")
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
        let y = panes[0].y.saturating_add(1);
        frame.set_cursor(x, y);
    }
}

fn draw_status(frame: &mut Frame<'_>, area: Rect, app: &AppState) {
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
            .gauge_style(Style::default().fg(MOCHA_CYAN).bg(MOCHA_SURFACE))
            .ratio(ratio.clamp(0.0, 1.0));
        frame.render_widget(gauge, area);
        return;
    }

    let mode = match app.input_mode {
        InputMode::Normal => "NORMAL",
        InputMode::Editing => "EDIT",
    };
    let keys = format!(" {mode} | [i] Edit | [Esc] Normal Mode | [Tab] Switch Pane | [Ctrl+D] Download en-it | [q] Quit | {}", app.status_message);
    frame.render_widget(
        Paragraph::new(keys)
            .alignment(Alignment::Center)
            .style(Style::default().fg(MOCHA_TEXT).bg(MOCHA_SURFACE)),
        area,
    );
}
