use clap::Parser;
use tui::{*, widgets::*, style::*};
use crossterm::{terminal::*, event::*, execute};

#[derive(Parser)]
struct Args {
    #[arg(help = "File to open", default_value = "file.hex")]
    file: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = backend::CrosstermBackend::new(stdout);
    let mut term = Terminal::new(backend)?;

    loop {
        term.draw(|f| {
            let size = f.size();
            let block = Block::default()
                .title("Block")
                .borders(Borders::ALL);
            f.render_widget(block, size);
        })?;

        if poll(std::time::Duration::from_millis(33))? {
            match read()? {
                Event::Key(KeyEvent { code: KeyCode::Esc, .. }) => break,
                _ => {},
            }
        }
    }

    disable_raw_mode()?;
    execute!(
        term.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    Ok(())
}
