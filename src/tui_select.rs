use std::io;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame, Terminal,
};
use unicode_width::UnicodeWidthStr;

enum InputMode {
    Select,
    Input,
}

struct App {
    input_mode: InputMode,
    search_string: String,
    search_results: Vec<String>,
    list_state: ListState,
}

impl App {
    fn new() -> Self {
        Self {
            input_mode: InputMode::Input,
            search_string: "".to_string(),
            search_results: Vec::new(),
            list_state: ListState::default(),
        }
    }

    fn select_next(&mut self) {
        if self.search_results.is_empty() {
            return;
        }
        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.search_results.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    fn select_previous(&mut self) {
        if self.search_results.is_empty() {
            return;
        }
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.search_results.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    fn unselect(&mut self) {
        self.list_state.select(None);
    }
}

fn ui<B: Backend>(frame: &mut Frame<B>, app: &mut App) {
    let size = frame.size();
    let application_block = Block::default()
        .title("NoteMD")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(application_block.inner(frame.size()));

    let input = Paragraph::new(format!("> {}", app.search_string)).block(
        Block::default()
            .title("Search")
            .borders(Borders::ALL)
            .border_style(match app.input_mode {
                InputMode::Select => Style::default(),
                InputMode::Input => Style::default().fg(Color::Yellow),
            }),
    );
    match app.input_mode {
        InputMode::Select => (),
        InputMode::Input => frame.set_cursor(
            chunks[0].x + app.search_string.width() as u16 + 3,
            chunks[0].y + 1,
        ),
    }

    let items: Vec<_> = app
        .search_results
        .iter()
        .map(|result| ListItem::new(result.as_str()))
        .collect();
    let result = List::new(items)
        .block(Block::default().title("Notes").borders(Borders::ALL))
        .highlight_style(
            Style::default()
                .fg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    frame.render_widget(application_block, size);
    frame.render_widget(input, chunks[0]);
    frame.render_stateful_widget(result, chunks[1], &mut app.list_state);
}

pub fn select_note_with_tui() -> anyhow::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    loop {
        terminal.draw(|frame| ui(frame, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Select => match key.code {
                    KeyCode::Char(character) => match character {
                        'q' => break,
                        'i' => {
                            app.input_mode = InputMode::Input;
                            app.unselect()
                        }
                        'j' => app.select_next(),
                        'k' => app.select_previous(),
                        _ => (),
                    },
                    KeyCode::Esc => break,
                    KeyCode::Down => app.select_next(),
                    KeyCode::Up => app.select_previous(),
                    _ => (),
                },
                InputMode::Input => match key.code {
                    KeyCode::Char(character) => app.search_string.push(character),
                    KeyCode::Backspace => {
                        app.search_string.pop();
                    }
                    KeyCode::Esc => {
                        app.input_mode = InputMode::Select;
                        app.select_next()
                    }
                    _ => (),
                },
            }
        }
    }

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
