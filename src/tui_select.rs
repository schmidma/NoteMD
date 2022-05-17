use std::{
    io,
    path::{Path, PathBuf},
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use log::error;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame, Terminal,
};
use unicode_width::UnicodeWidthStr;

use crate::search::{search_for_files, SearchResult};

enum InputMode {
    Select,
    Input,
}

struct App<'a> {
    notes_directory: &'a Path,
    input_mode: InputMode,
    search_string: String,
    search_results: Vec<SearchResult>,
    list_state: ListState,
}

impl<'a> App<'a> {
    fn new(notes_directory: &'a Path) -> Self {
        Self {
            notes_directory,
            input_mode: InputMode::Input,
            search_string: String::new(),
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
                if i < self.search_results.len() - 1 {
                    i + 1
                }
                else {
                    i
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
                if i > 0 {
                    i - 1
                } else {
                    i
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    fn unselect(&mut self) {
        self.list_state.select(None);
    }

    fn search(&mut self) {
        let matched = match search_for_files(self.notes_directory, &self.search_string) {
            Ok(matched) => matched,
            Err(error) => {
                error!(
                    "Failed to search with string '{}' in notes directory: {}",
                    self.search_string, error
                );
                return;
            }
        };
        self.search_results = matched;
    }
}

fn ui<B>(frame: &mut Frame<B>, app: &mut App)
where
    B: Backend,
{
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
        .filter_map(|result| result.file_stem())
        .map(ListItem::new)
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

fn app_loop<B>(mut app: App, terminal: &mut Terminal<B>) -> anyhow::Result<Option<PathBuf>>
where
    B: Backend,
{
    loop {
        terminal.draw(|frame| ui(frame, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Select => match key.code {
                    KeyCode::Char(character) => match character {
                        'q' => return Ok(None),
                        'i' => {
                            app.input_mode = InputMode::Input;
                            app.unselect()
                        }
                        'j' => app.select_next(),
                        'k' => app.select_previous(),
                        _ => (),
                    },
                    KeyCode::Esc => return Ok(None),
                    KeyCode::Down => app.select_next(),
                    KeyCode::Up => app.select_previous(),
                    KeyCode::Enter => break,
                    _ => (),
                },
                InputMode::Input => match key.code {
                    KeyCode::Char(character) => {
                        app.search_string.push(character);
                        app.search();
                    }
                    KeyCode::Backspace => {
                        app.search_string.pop();
                        app.search();
                    }
                    KeyCode::Esc => {
                        app.input_mode = InputMode::Select;
                        app.select_next()
                    }
                    KeyCode::Enter => break,
                    _ => (),
                },
            }
        }
    }
    let note_to_open = match app.input_mode {
        InputMode::Select => match app.list_state.selected() {
            Some(i) => app.search_results.remove(i).path(),
            None => app
                .notes_directory
                .join(format!("{}.md", app.search_string)),
        },
        InputMode::Input => app
            .notes_directory
            .join(format!("{}.md", app.search_string)),
    };
    Ok(Some(note_to_open))
}

pub fn select_note_with_tui<P>(notes_directory: P) -> anyhow::Result<Option<PathBuf>>
where
    P: AsRef<Path>,
{
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(notes_directory.as_ref());
    app.search();

    let note_to_open = app_loop(app, &mut terminal)?;

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(note_to_open)
}
