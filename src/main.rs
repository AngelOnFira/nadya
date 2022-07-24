use crossterm::event;
use crossterm::event::DisableMouseCapture;
use crossterm::event::EnableMouseCapture;
use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::execute;
use crossterm::terminal::disable_raw_mode;
use crossterm::terminal::enable_raw_mode;
use crossterm::terminal::EnterAlternateScreen;
use crossterm::terminal::LeaveAlternateScreen;
use prelude::*;
use std::fs;
use std::io;
use std::thread;
use std::time::Duration;
use std::time::Instant;
use tui::backend::Backend;
use tui::layout::Alignment;
use tui::layout::Constraint;
use tui::layout::Direction;
use tui::layout::Layout;
use tui::style::Color;
use tui::style::Modifier;
use tui::style::Style;
use tui::text::Span;
use tui::widgets::BarChart;
use tui::widgets::Block;
use tui::widgets::Borders;
use tui::widgets::Paragraph;
use tui::Frame;
use tui::{backend::CrosstermBackend, Terminal};

mod lexer;
mod parser;
mod place;
mod program;
mod simulation;
mod syntax;

mod prelude {
    pub use crate::{lexer::*, parser::*, place::*, program::*, simulation::*, syntax::*};
}

fn main() -> Result<(), io::Error> {
    // Parse the language
    // Load the file in tests/hello_world
    let contents = fs::read_to_string("tests/hello_world").unwrap();

    // Parse the file
    let mut program = parse(&contents);

    // Lex the file
    lexer(&mut program);

    setup_terminal(program)?;

    Ok(())
}

fn setup_terminal(program: Program) -> Result<(), io::Error> {
    // Set up the terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Set up the app
    let tick_rate = Duration::from_millis(250);
    let app = App::new(program);
    let res = run_app(&mut terminal, app, tick_rate);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    // Exit the program
    Ok(())
}

struct App {
    simulation: Simulation,
}

impl App {
    fn new(program: Program) -> App {
        App {
            simulation: Simulation::new(program),
        }
    }

    fn on_tick(&mut self) {
        // Run a simulation tick
        self.simulation.simulate();
    }
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.size());

    let paragraph = Paragraph::new(app.simulation.map_string())
        .style(Style::default().bg(Color::White).fg(Color::Black))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::White).fg(Color::Black))
                .title(Span::styled(
                    "Test",
                    Style::default().add_modifier(Modifier::BOLD),
                )),
        )
        .alignment(Alignment::Left);

    f.render_widget(paragraph, chunks[0]);
}
