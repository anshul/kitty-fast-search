use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Gauge},
    Frame,
};

use crate::search::engine::SearchResult;

#[allow(dead_code)]
pub struct UIRenderer {
    pub show_line_numbers: bool,
    pub highlight_color: Color,
    pub max_results_displayed: usize,
}

#[allow(dead_code)]
impl UIRenderer {
    pub fn new() -> Self {
        Self {
            show_line_numbers: true,
            highlight_color: Color::Yellow,
            max_results_displayed: 100,
        }
    }

    pub fn render_search_interface(
        &self,
        f: &mut Frame,
        query: &str,
        results: &[SearchResult],
        selected_index: usize,
        is_searching: bool,
    ) {
        let size = f.size();
        
        // Create main layout
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Search input
                Constraint::Length(1), // Status line
                Constraint::Min(0),    // Results
            ])
            .split(size);

        // Render search input
        self.render_search_input(f, chunks[0], query);
        
        // Render status line
        self.render_status_line(f, chunks[1], results.len(), is_searching);
        
        // Render results
        self.render_results(f, chunks[2], results, selected_index);
    }

    fn render_search_input(&self, f: &mut Frame, area: Rect, query: &str) {
        let input = Paragraph::new(format!("Search: {}", query))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Kitty Fast Search")
                    .title_style(Style::default().fg(Color::Cyan))
            )
            .style(Style::default().fg(Color::White));
        
        f.render_widget(input, area);
    }

    fn render_status_line(&self, f: &mut Frame, area: Rect, result_count: usize, is_searching: bool) {
        let status_text = if is_searching {
            "Searching...".to_string()
        } else {
            format!("{} results found", result_count)
        };

        let status = Paragraph::new(status_text)
            .style(Style::default().fg(Color::Gray));
        
        f.render_widget(status, area);
    }

    fn render_results(&self, f: &mut Frame, area: Rect, results: &[SearchResult], selected_index: usize) {
        let displayed_results = results.iter()
            .take(self.max_results_displayed)
            .enumerate()
            .map(|(i, result)| {
                let is_selected = i == selected_index;
                self.create_result_item(result, is_selected)
            })
            .collect::<Vec<_>>();

        let results_list = List::new(displayed_results)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Results")
                    .title_style(Style::default().fg(Color::Green))
            );
        
        f.render_widget(results_list, area);
    }

    fn create_result_item<'a>(&self, result: &'a SearchResult, is_selected: bool) -> ListItem<'a> {
        let line_style = if is_selected {
            Style::default()
                .bg(Color::Blue)
                .fg(Color::White)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        };

        let line_number_style = Style::default().fg(Color::Cyan);
        
        let content = if self.show_line_numbers {
            vec![Line::from(vec![
                Span::styled(format!("{:6}: ", result.line_number), line_number_style),
                Span::styled(&result.line, line_style),
            ])]
        } else {
            vec![Line::from(Span::styled(&result.line, line_style))]
        };

        ListItem::new(content)
    }

    pub fn render_progress(&self, f: &mut Frame, area: Rect, progress: f64) {
        let gauge = Gauge::default()
            .block(Block::default().borders(Borders::ALL).title("Search Progress"))
            .gauge_style(Style::default().fg(Color::Blue))
            .ratio(progress);
        
        f.render_widget(gauge, area);
    }
}