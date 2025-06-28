use anyhow::Result;
use crossterm::event::{Event, KeyEvent, KeyEventKind};
use std::time::Duration;

use crate::search::SearchEngine;
use crate::kitty::KittyClient;
use crate::ui::screen::Screen;
use crate::ui::input::{InputHandler, InputAction};

pub struct SearchUI {
    kitty_client: KittyClient,
    search_engine: SearchEngine,
    screen: Screen,
    input: InputHandler,
    results: Vec<crate::search::engine::SearchResult>,
    current_idx: usize,
    dirty: bool,
}

impl SearchUI {
    pub async fn new(kitty_client: KittyClient, search_engine: SearchEngine) -> Result<Self> {
        Ok(Self {
            kitty_client,
            search_engine,
            screen: Screen::new()?,
            input: InputHandler::new(),
            results: Vec::new(),
            current_idx: 0,
            dirty: true,
        })
    }

    pub fn set_initial_query(&mut self, query: String) {
        self.input.set_query(query);
        self.dirty = true;
    }

    pub async fn run(&mut self) -> Result<()> {
        // Initial search if query is set
        if !self.input.query().is_empty() {
            self.recompute_matches().await?;
        }

        loop {
            // Redraw if needed
            if self.dirty {
                self.screen.draw_panel(
                    self.input.query(),
                    self.current_idx + 1,
                    self.results.len(),
                )?;
                self.dirty = false;
            }

            // Wait for next event
            if let Some(evt) = Screen::poll_event(Duration::from_millis(400))? {
                match evt {
                    Event::Key(key) => {
                        if let Some(action) = self.handle_key(key).await? {
                            match action {
                                InputAction::Exit => break,
                                InputAction::Select => {
                                    if let Some(result) = self.results.get(self.current_idx) {
                                        self.kitty_client.jump_to_line(result.line_number).await?;
                                    }
                                    break;
                                }
                                _ => {}
                            }
                        }
                    }
                    Event::Resize(_, _) => self.dirty = true,
                    _ => {}
                }
            }
        }

        // Cleanup
        self.remove_marker().await;
        Ok(())
    }

    async fn handle_key(&mut self, key: KeyEvent) -> Result<Option<InputAction>> {
        if key.kind != KeyEventKind::Press {
            return Ok(None);
        }

        let action = self.input.handle_key_event(key)?;
        match action {
            InputAction::QueryChanged => {
                self.recompute_matches().await?;
                self.dirty = true;
            }
            InputAction::NavigateUp => {
                if self.current_idx > 0 {
                    self.current_idx -= 1;
                    self.dirty = true;
                }
            }
            InputAction::NavigateDown => {
                if self.current_idx + 1 < self.results.len() {
                    self.current_idx += 1;
                    self.dirty = true;
                }
            }
            _ => {}
        }
        Ok(Some(action))
    }

    async fn recompute_matches(&mut self) -> Result<()> {
        self.remove_marker().await;
        
        if self.input.query().is_empty() {
            self.results.clear();
            self.current_idx = 0;
            return Ok(());
        }

        let buffer_content = self.kitty_client.get_buffer_content().await?;
        self.results = self.search_engine.search_text(&buffer_content, self.input.query())?;
        self.current_idx = 0;
        
        if !self.results.is_empty() {
            self.refresh_marker().await?;
        }

        Ok(())
    }

    async fn refresh_marker(&self) -> Result<()> {
        self.kitty_client.create_text_marker(self.input.query()).await
    }

    async fn remove_marker(&self) {
        let _ = self.kitty_client.remove_marker().await;
    }
}

impl Drop for SearchUI {
    fn drop(&mut self) {
        // Screen handles terminal cleanup automatically
    }
}