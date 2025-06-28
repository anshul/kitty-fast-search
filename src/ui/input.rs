use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use anyhow::Result;

#[allow(dead_code)]
pub struct InputHandler {
    pub query: String,
    pub cursor_pos: usize,
}

#[allow(dead_code)]
impl InputHandler {
    pub fn new() -> Self {
        Self {
            query: String::new(),
            cursor_pos: 0,
        }
    }

    pub fn handle_key_event(&mut self, key: KeyEvent) -> Result<InputAction> {
        if key.kind != KeyEventKind::Press {
            return Ok(InputAction::None);
        }

        match key.code {
            KeyCode::Char(c) => {
                self.query.insert(self.cursor_pos, c);
                self.cursor_pos += 1;
                Ok(InputAction::QueryChanged)
            }
            KeyCode::Backspace => {
                if self.cursor_pos > 0 {
                    self.cursor_pos -= 1;
                    self.query.remove(self.cursor_pos);
                    Ok(InputAction::QueryChanged)
                } else {
                    Ok(InputAction::None)
                }
            }
            KeyCode::Delete => {
                if self.cursor_pos < self.query.len() {
                    self.query.remove(self.cursor_pos);
                    Ok(InputAction::QueryChanged)
                } else {
                    Ok(InputAction::None)
                }
            }
            KeyCode::Left => {
                if self.cursor_pos > 0 {
                    self.cursor_pos -= 1;
                }
                Ok(InputAction::None)
            }
            KeyCode::Right => {
                if self.cursor_pos < self.query.len() {
                    self.cursor_pos += 1;
                }
                Ok(InputAction::None)
            }
            KeyCode::Home => {
                self.cursor_pos = 0;
                Ok(InputAction::None)
            }
            KeyCode::End => {
                self.cursor_pos = self.query.len();
                Ok(InputAction::None)
            }
            KeyCode::Up => Ok(InputAction::NavigateUp),
            KeyCode::Down => Ok(InputAction::NavigateDown),
            KeyCode::Enter => Ok(InputAction::Select),
            KeyCode::Esc => {
                if self.query.is_empty() {
                    Ok(InputAction::Exit)
                } else {
                    self.clear();
                    Ok(InputAction::QueryChanged)
                }
            }
            _ => Ok(InputAction::None),
        }
    }

    pub fn set_query(&mut self, query: String) {
        self.query = query;
        self.cursor_pos = self.query.len();
    }

    pub fn clear(&mut self) {
        self.query.clear();
        self.cursor_pos = 0;
    }

    pub fn query(&self) -> &str {
        &self.query
    }
}

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum InputAction {
    None,
    QueryChanged,
    NavigateUp,
    NavigateDown,
    Select,
    Exit,
}