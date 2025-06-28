use anyhow::Result;
use grep::regex::RegexMatcher;
use grep::searcher::{BinaryDetection, SearcherBuilder};
use grep::searcher::sinks::UTF8;
use grep::matcher::Matcher;
use std::sync::Arc;
use lru::LruCache;
use std::num::NonZeroUsize;

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub line_number: u64,
    pub line: String,
    #[allow(dead_code)]
    pub match_start: usize,
    #[allow(dead_code)]
    pub match_end: usize,
}

pub struct SearchEngine {
    #[allow(dead_code)]
    max_buffer_size: usize,
    case_sensitive: bool,
    regex_enabled: bool,
    cache: Arc<std::sync::Mutex<LruCache<String, Vec<SearchResult>>>>,
}

impl SearchEngine {
    pub fn new(max_buffer_size: usize, case_sensitive: bool, regex_enabled: bool) -> Result<Self> {
        let cache_size = NonZeroUsize::new(100).unwrap();
        Ok(Self {
            max_buffer_size,
            case_sensitive,
            regex_enabled,
            cache: Arc::new(std::sync::Mutex::new(LruCache::new(cache_size))),
        })
    }

    pub fn search_text(&self, text: &str, pattern: &str) -> Result<Vec<SearchResult>> {
        if pattern.is_empty() {
            return Ok(Vec::new());
        }

        // Check cache first
        let cache_key = format!("{}:{}:{}:{}", pattern, self.case_sensitive, self.regex_enabled, text.len());
        if let Ok(mut cache) = self.cache.lock() {
            if let Some(cached_results) = cache.get(&cache_key) {
                return Ok(cached_results.clone());
            }
        }

        let mut results = Vec::new();
        
        // Create regex pattern
        let regex_pattern = if self.regex_enabled {
            pattern.to_string()
        } else {
            regex::escape(pattern)
        };

        let final_pattern = if self.case_sensitive {
            regex_pattern
        } else {
            format!("(?i){}", regex_pattern)
        };
        
        // Create regex matcher
        let matcher = RegexMatcher::new(&final_pattern)?;
        
        // Configure searcher
        let mut searcher = SearcherBuilder::new()
            .binary_detection(BinaryDetection::quit(b'\x00'))
            .line_number(true)
            .build();

        // Search the text
        searcher.search_slice(
            &matcher,
            text.as_bytes(),
            UTF8(|lnum, line| {
                let line_bytes = line.as_bytes();
                let line_str = std::str::from_utf8(line_bytes).unwrap_or("");
                // Find all matches in this line
                let mut match_start = 0;
                while let Some(mat) = matcher.find_at(line_bytes, match_start)? {
                    results.push(SearchResult {
                        line_number: lnum,
                        line: line_str.to_string(),
                        match_start: mat.start(),
                        match_end: mat.end(),
                    });
                    match_start = mat.end();
                    if match_start >= line_bytes.len() {
                        break;
                    }
                }
                Ok(true)
            }),
        )?;

        // Cache the results
        if let Ok(mut cache) = self.cache.lock() {
            cache.put(cache_key, results.clone());
        }

        Ok(results)
    }

    #[allow(dead_code)]
    pub fn search_buffer(&self, buffer: &[u8], pattern: &str) -> Result<Vec<SearchResult>> {
        let text = String::from_utf8_lossy(buffer);
        self.search_text(&text, pattern)
    }

    #[allow(dead_code)]
    pub fn clear_cache(&self) {
        if let Ok(mut cache) = self.cache.lock() {
            cache.clear();
        }
    }

    #[allow(dead_code)]
    pub fn get_cache_size(&self) -> usize {
        if let Ok(cache) = self.cache.lock() {
            cache.len()
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_search() {
        let engine = SearchEngine::new(1000, true, false).unwrap();
        let text = "Hello world\nThis is a test\nHello again";
        let results = engine.search_text(text, "Hello").unwrap();
        
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].line_number, 1);
        assert_eq!(results[1].line_number, 3);
    }

    #[test]
    fn test_case_insensitive_search() {
        let engine = SearchEngine::new(1000, false, false).unwrap();
        let text = "Hello World\nthis is a TEST\nhello again";
        let results = engine.search_text(text, "hello").unwrap();
        
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].line_number, 1);
        assert_eq!(results[1].line_number, 3);
    }

    #[test]
    fn test_regex_search() {
        let engine = SearchEngine::new(1000, true, true).unwrap();
        let text = "Error: 404\nWarning: timeout\nError: 500";
        let results = engine.search_text(text, r"Error: \d+").unwrap();
        
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].line_number, 1);
        assert_eq!(results[1].line_number, 3);
    }

    #[test]
    fn test_empty_pattern() {
        let engine = SearchEngine::new(1000, true, false).unwrap();
        let text = "Hello world";
        let results = engine.search_text(text, "").unwrap();
        
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_no_matches() {
        let engine = SearchEngine::new(1000, true, false).unwrap();
        let text = "Hello world\nThis is a test";
        let results = engine.search_text(text, "nonexistent").unwrap();
        
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_cache_functionality() {
        let engine = SearchEngine::new(1000, true, false).unwrap();
        let text = "Hello world\nThis is a test";
        
        // First search
        let _results1 = engine.search_text(text, "Hello").unwrap();
        assert_eq!(engine.get_cache_size(), 1);
        
        // Second search with same pattern (should use cache)
        let _results2 = engine.search_text(text, "Hello").unwrap();
        assert_eq!(engine.get_cache_size(), 1);
        
        // Different pattern
        let _results3 = engine.search_text(text, "test").unwrap();
        assert_eq!(engine.get_cache_size(), 2);
    }

    #[test]
    fn test_buffer_search() {
        let engine = SearchEngine::new(1000, true, false).unwrap();
        let buffer = b"Hello world\nThis is a test\nHello again";
        let results = engine.search_buffer(buffer, "Hello").unwrap();
        
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].line_number, 1);
        assert_eq!(results[1].line_number, 3);
    }
}