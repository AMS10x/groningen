use std::io::{self, Stdout};

use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

pub mod events;
pub mod ui;

pub type GroningenTerminal = Terminal<CrosstermBackend<Stdout>>;

pub fn enter_terminal() -> anyhow::Result<GroningenTerminal> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    Ok(Terminal::new(CrosstermBackend::new(stdout))?)
}

pub fn restore_terminal(terminal: &mut GroningenTerminal) -> anyhow::Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

pub struct TerminalGuard {
    terminal: GroningenTerminal,
}

impl TerminalGuard {
    pub fn enter() -> anyhow::Result<Self> {
        Ok(Self {
            terminal: enter_terminal()?,
        })
    }
    pub fn terminal_mut(&mut self) -> &mut GroningenTerminal {
        &mut self.terminal
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = restore_terminal(&mut self.terminal);
    }
}
