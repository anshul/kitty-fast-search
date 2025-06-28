use anyhow::Result;
use regex::Regex;

#[allow(dead_code)]
pub struct PatternMatcher {
    case_sensitive: bool,
    regex_enabled: bool,
}

#[allow(dead_code)]
impl PatternMatcher {
    pub fn new(case_sensitive: bool, regex_enabled: bool) -> Self {
        Self {
            case_sensitive,
            regex_enabled,
        }
    }

    pub fn compile_pattern(&self, pattern: &str) -> Result<Regex> {
        let mut regex_pattern = if self.regex_enabled {
            pattern.to_string()
        } else {
            regex::escape(pattern)
        };

        if !self.case_sensitive {
            regex_pattern = format!("(?i){}", regex_pattern);
        }

        Ok(Regex::new(&regex_pattern)?)
    }

    pub fn is_match(&self, pattern: &str, text: &str) -> Result<bool> {
        let regex = self.compile_pattern(pattern)?;
        Ok(regex.is_match(text))
    }

    pub fn find_matches(&self, pattern: &str, text: &str) -> Result<Vec<(usize, usize)>> {
        let regex = self.compile_pattern(pattern)?;
        let matches: Vec<(usize, usize)> = regex
            .find_iter(text)
            .map(|m| (m.start(), m.end()))
            .collect();
        Ok(matches)
    }
}