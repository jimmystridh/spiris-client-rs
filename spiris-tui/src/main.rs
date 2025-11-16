mod app;
mod auth;
mod screens;
mod ui;

use anyhow::Result;
use app::App;
use crossterm::{
    event::{self as terminal_event, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

#[tokio::main]
async fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run it
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Error: {:?}", err);
    }

    Ok(())
}

async fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> Result<()> {
    loop {
        terminal.draw(|f| ui::draw(f, app))?;

        if terminal_event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = terminal_event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') if app.can_quit() => return Ok(()),
                        KeyCode::Esc => app.handle_escape(),
                        KeyCode::Enter => app.handle_enter().await?,
                        KeyCode::Tab => app.next_screen(),
                        KeyCode::BackTab => app.previous_screen(),
                        KeyCode::Up => app.handle_up(),
                        KeyCode::Down => app.handle_down(),
                        KeyCode::Left => app.handle_left(),
                        KeyCode::Right => app.handle_right(),
                        KeyCode::Char(c) => app.handle_char(c),
                        KeyCode::Backspace => app.handle_backspace(),
                        _ => {}
                    }
                }
            }
        }
    }
}
