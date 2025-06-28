use crate::search::engine::SearchResult;

#[allow(dead_code)]
pub struct UIRenderer {
    pub show_line_numbers: bool,
    pub max_results_displayed: usize,
}

#[allow(dead_code)]
impl UIRenderer {
    pub fn new() -> Self {
        Self {
            show_line_numbers: true,
            max_results_displayed: 100,
        }
    }

    pub fn format_result_line(&self, result: &SearchResult, _is_selected: bool, max_width: usize) -> String {
        let line_prefix = if self.show_line_numbers {
            format!("{:4}: ", result.line_number)
        } else {
            String::new()
        };
        
        let available_width = max_width.saturating_sub(line_prefix.len());
        let content = if result.line.len() > available_width {
            format!("{:.width$}…", result.line, width = available_width.saturating_sub(1))
        } else {
            result.line.clone()
        };
        
        format!("{}{}", line_prefix, content)
    }
}