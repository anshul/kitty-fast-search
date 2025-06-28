use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};
use std::io;
use tokio::time::Duration;

use crate::search::SearchEngine;
use crate::kitty::KittyClient;

pub struct SearchUI {
    kitty_client: KittyClient,
    search_engine: SearchEngine,
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
    query: String,
    results: Vec<crate::search::engine::SearchResult>,
    selected_index: usize,
}

impl SearchUI {
    pub async fn new(kitty_client: KittyClient, search_engine: SearchEngine) -> Result<Self> {
        crossterm::terminal::enable_raw_mode()?;
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;

        Ok(Self {
            kitty_client,
            search_engine,
            terminal,
            query: String::new(),
            results: Vec::new(),
            selected_index: 0,
        })
    }

    pub fn set_initial_query(&mut self, query: String) {
        self.query = query;
    }

    pub async fn run(&mut self) -> Result<()> {
        crossterm::execute!(
            io::stdout(),
            crossterm::terminal::EnterAlternateScreen,
            crossterm::cursor::Hide
        )?;

        let result = self.run_app().await;

        crossterm::execute!(
            io::stdout(),
            crossterm::cursor::Show,
            crossterm::terminal::LeaveAlternateScreen
        )?;
        crossterm::terminal::disable_raw_mode()?;

        result
    }

    async fn run_app(&mut self) -> Result<()> {
        loop {
            self.draw()?;

            // Check for input with timeout
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Esc => break,
                            KeyCode::Enter => {
                                if let Some(result) = self.results.get(self.selected_index) {
                                    self.kitty_client.jump_to_line(result.line_number).await?;
                                }
                                break;
                            }
                            KeyCode::Up => {
                                if self.selected_index > 0 {
                                    self.selected_index -= 1;
                                }
                            }
                            KeyCode::Down => {
                                if self.selected_index + 1 < self.results.len() {
                                    self.selected_index += 1;
                                }
                            }
                            KeyCode::Backspace => {
                                self.query.pop();
                                self.update_search().await?;
                            }
                            KeyCode::Char(c) => {
                                self.query.push(c);
                                self.update_search().await?;
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        Ok(())
    }

    async fn update_search(&mut self) -> Result<()> {
        if self.query.is_empty() {
            self.results.clear();
            self.selected_index = 0;
            return Ok(());
        }

        // Get buffer content from kitty
        let buffer_content = self.kitty_client.get_buffer_content().await?;
        
        // Perform search
        self.results = self.search_engine.search_text(&buffer_content, &self.query)?;
        self.selected_index = 0;

        Ok(())
    }

    fn draw(&mut self) -> Result<()> {
        self.terminal.draw(|f| {
            let size = f.size();
            
            // Create layout
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3), // Search box
                    Constraint::Min(0),    // Results
                ])
                .split(size);

            // Search input
            let search_input = Paragraph::new(format!("Search: {}", self.query))
                .block(Block::default().borders(Borders::ALL).title("Kitty Fast Search"));
            f.render_widget(search_input, chunks[0]);

            // Results list
            let results_items: Vec<ListItem> = self.results
                .iter()
                .enumerate()
                .map(|(i, result)| {
                    let style = if i == self.selected_index {
                        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default()
                    };
                    
                    let content = vec![Line::from(vec![
                        Span::styled(format!("{:4}: ", result.line_number), Style::default().fg(Color::Blue)),
                        Span::styled(&result.line, style),
                    ])];
                    
                    ListItem::new(content)
                })
                .collect();

            let results_list = List::new(results_items)
                .block(Block::default().borders(Borders::ALL).title(format!("Results ({})", self.results.len())));
            
            f.render_widget(results_list, chunks[1]);
        })?;

        Ok(())
    }
}

impl Drop for SearchUI {
    fn drop(&mut self) {
        let _ = crossterm::terminal::disable_raw_mode();
    }
}