use crossterm::{
  cursor::{Hide, MoveTo, Show},
  event::{self, Event},
  style::{Print, Stylize},
  terminal::{self, Clear, ClearType},
  ExecutableCommand, QueueableCommand,
};
use std::io::{stdout, Stdout, Write, Result as IoResult};
use std::time::Duration;

pub struct Screen {
  out: Stdout,
}

impl Screen {
  pub fn new() -> IoResult<Self> {
    let mut out = stdout();
    terminal::enable_raw_mode()?;
    out.execute(Hide)?;
    Ok(Self { out })
  }

  pub fn draw_panel(&mut self, query: &str, idx: usize, total: usize) -> IoResult<()> {
    let (cols, rows) = terminal::size()?;
    let x = cols.saturating_sub(30);
    let y = rows.saturating_sub(4);

    // Clear panel rectangle (4 rows × 30 cols)
    for row in 0..4 {
      self
        .out
        .queue(MoveTo(x, y + row))?
        .queue(Clear(ClearType::UntilNewLine))?;
    }

    // First line: prompt
    self
      .out
      .queue(MoveTo(x, y))?
      .queue(Print("🔍 ".to_string()))?
      .queue(Print(query.bold()))?
      .queue(Print("▌"))?;

    // Second line: blank separator
    self.out.queue(MoveTo(x, y + 1))?;

    // Third line: status
    self
      .out
      .queue(MoveTo(x, y + 2))?
      .queue(Print(format!("{}/{}  ↑↓ jump  Esc quit", idx, total)))?;

    self.out.flush()
  }

  pub fn poll_event(timeout: Duration) -> IoResult<Option<Event>> {
    if event::poll(timeout)? {
      Ok(Some(event::read()?))
    } else {
      Ok(None)
    }
  }
}

impl Drop for Screen {
  fn drop(&mut self) {
    let _ = self.out.execute(Show);
    let _ = terminal::disable_raw_mode();
  }
}