use crossterm::event::KeyCode;
use crossterm::execute;
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm_keyreader;
use std::io;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;

    let (mut rc, shatdown) = crossterm_keyreader::spawn();
    loop {
        if let Ok(event) = rc.try_recv() {
            match event.code {
                KeyCode::Char('q') => break,
                _ => {}
            }
        }
    }

    shatdown.send(()).unwrap();

    execute!(stdout, LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
