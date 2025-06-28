use std::collections::VecDeque;

#[allow(dead_code)]
pub struct TerminalBuffer {
    lines: VecDeque<String>,
    max_lines: usize,
    current_position: usize,
}

#[allow(dead_code)]
impl TerminalBuffer {
    pub fn new(max_lines: usize) -> Self {
        Self {
            lines: VecDeque::with_capacity(max_lines),
            max_lines,
            current_position: 0,
        }
    }

    pub fn add_line(&mut self, line: String) {
        if self.lines.len() >= self.max_lines {
            self.lines.pop_front();
        }
        self.lines.push_back(line);
    }

    pub fn add_lines(&mut self, lines: Vec<String>) {
        for line in lines {
            self.add_line(line);
        }
    }

    pub fn get_all_text(&self) -> String {
        self.lines.iter().cloned().collect::<Vec<_>>().join("\n")
    }

    pub fn get_lines(&self) -> &VecDeque<String> {
        &self.lines
    }

    pub fn get_line(&self, index: usize) -> Option<&String> {
        self.lines.get(index)
    }

    pub fn len(&self) -> usize {
        self.lines.len()
    }

    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }

    pub fn clear(&mut self) {
        self.lines.clear();
        self.current_position = 0;
    }

    pub fn set_position(&mut self, position: usize) {
        self.current_position = position.min(self.lines.len());
    }

    pub fn get_position(&self) -> usize {
        self.current_position
    }

    pub fn get_context_around(&self, line_number: usize, context_lines: usize) -> Vec<(usize, &String)> {
        let start = line_number.saturating_sub(context_lines);
        let end = (line_number + context_lines + 1).min(self.lines.len());
        
        (start..end)
            .filter_map(|i| self.lines.get(i).map(|line| (i, line)))
            .collect()
    }

    pub fn search_in_buffer(&self, pattern: &str, case_sensitive: bool) -> Vec<(usize, String)> {
        let search_pattern = if case_sensitive {
            pattern.to_string()
        } else {
            pattern.to_lowercase()
        };

        self.lines
            .iter()
            .enumerate()
            .filter_map(|(i, line)| {
                let search_line = if case_sensitive {
                    line.clone()
                } else {
                    line.to_lowercase()
                };
                
                if search_line.contains(&search_pattern) {
                    Some((i, line.clone()))
                } else {
                    None
                }
            })
            .collect()
    }
}