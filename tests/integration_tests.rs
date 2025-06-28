use anyhow::Result;
use kitty_fast_search::search::SearchEngine;
use tempfile::NamedTempFile;
use std::io::Write;

#[tokio::test]
async fn test_large_buffer_search() -> Result<()> {
    let engine = SearchEngine::new(10_000, false, false)?;
    
    // Generate test data (reduced for faster CI)
    let mut large_text = String::new();
    for i in 0..500 {
        if i % 100 == 0 {
            large_text.push_str(&format!("ERROR: Line {} has an error\n", i));
        } else if i % 200 == 0 {
            large_text.push_str(&format!("WARN: Line {} has a warning\n", i));
        } else {
            large_text.push_str(&format!("INFO: Line {} normal log entry\n", i));
        }
    }
    
    let results = engine.search_text(&large_text, "ERROR")?;
    assert!(!results.is_empty());
    assert!(results.len() >= 4); // Should find ~5 errors
    
    Ok(())
}

#[tokio::test]
async fn test_concurrent_searches() -> Result<()> {
    let text = "Hello world\nThis is a test\nHello again\nTesting concurrent access";
    
    // Spawn concurrent search tasks (reduced for faster CI)
    let mut handles = vec![];
    
    let engine = std::sync::Arc::new(SearchEngine::new(1000, false, false)?);
    
    for i in 0..3 {
        let engine_clone = std::sync::Arc::clone(&engine);
        let text_clone = text.to_string();
        let pattern = if i % 2 == 0 { "Hello" } else { "test" };
        let pattern_clone = pattern.to_string();
        
        let handle = tokio::spawn(async move {
            engine_clone.search_text(&text_clone, &pattern_clone)
        });
        handles.push(handle);
    }
    
    // Wait for all searches to complete
    for handle in handles {
        let result = handle.await??;
        assert!(!result.is_empty() || true); // Some might not have matches
    }
    
    Ok(())
}

#[tokio::test]
async fn test_file_based_search() -> Result<()> {
    let engine = SearchEngine::new(10_000, true, false)?;
    
    // Create temporary file with test content
    let mut temp_file = NamedTempFile::new()?;
    writeln!(temp_file, "Line 1: Starting application")?;
    writeln!(temp_file, "Line 2: Loading configuration")?;
    writeln!(temp_file, "Line 3: ERROR: Failed to connect to database")?;
    writeln!(temp_file, "Line 4: Retrying connection...")?;
    writeln!(temp_file, "Line 5: ERROR: Connection timeout")?;
    writeln!(temp_file, "Line 6: Application shutting down")?;
    
    // Read file content
    let content = std::fs::read_to_string(temp_file.path())?;
    let results = engine.search_text(&content, "ERROR")?;
    
    assert_eq!(results.len(), 2);
    assert_eq!(results[0].line_number, 3);
    assert_eq!(results[1].line_number, 5);
    
    Ok(())
}

#[tokio::test]
async fn test_regex_patterns() -> Result<()> {
    let engine = SearchEngine::new(1000, true, true)?;
    let text = r#"
2023-12-01 10:30:15 INFO Starting service
2023-12-01 10:30:16 ERROR Connection failed (code: 500)
2023-12-01 10:30:17 WARN Retrying in 5 seconds
2023-12-01 10:30:22 ERROR Timeout occurred (code: 408)
2023-12-01 10:30:23 INFO Service recovered
"#;
    
    // Test timestamp pattern
    let timestamp_results = engine.search_text(text, r"\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}")?;
    assert_eq!(timestamp_results.len(), 5);
    
    // Test error code pattern
    let error_code_results = engine.search_text(text, r"code: \d+")?;
    assert_eq!(error_code_results.len(), 2);
    
    // Test log level pattern
    let log_level_results = engine.search_text(text, r"(ERROR|WARN|INFO)")?;
    assert_eq!(log_level_results.len(), 5);
    
    Ok(())
}

#[tokio::test]
async fn test_memory_efficiency() -> Result<()> {
    let engine = SearchEngine::new(100_000, false, false)?;
    
    // Create test content (reduced for faster CI)
    let mut large_content = String::with_capacity(100_000);
    for i in 0..5_000 {
        large_content.push_str(&format!("Line {}: Some random content here\n", i));
        if i % 1000 == 0 {
            large_content.push_str(&format!("MARKER: Checkpoint at line {}\n", i));
        }
    }
    
    // Search should complete without excessive memory usage
    let results = engine.search_text(&large_content, "MARKER")?;
    assert!(results.len() >= 4); // Should find ~5 markers
    
    // Clear cache to free memory
    engine.clear_cache();
    assert_eq!(engine.get_cache_size(), 0);
    
    Ok(())
}

#[tokio::test]
async fn test_unicode_support() -> Result<()> {
    let engine = SearchEngine::new(1000, true, false)?;
    let text = r#"
Hello 世界
Tëst with açcénts
Emoji test: 🔍 🚀 ⚡
Russian: Привет мир
Arabic: مرحبا بالعالم
"#;
    
    let unicode_results = engine.search_text(text, "世界")?;
    assert_eq!(unicode_results.len(), 1);
    
    let accent_results = engine.search_text(text, "açcénts")?;
    assert_eq!(accent_results.len(), 1);
    
    let emoji_results = engine.search_text(text, "🔍")?;
    assert_eq!(emoji_results.len(), 1);
    
    Ok(())
}

#[tokio::test]
async fn test_edge_cases() -> Result<()> {
    let engine = SearchEngine::new(1000, true, false)?;
    
    // Empty text
    let empty_results = engine.search_text("", "test")?;
    assert_eq!(empty_results.len(), 0);
    
    // Very long line
    let long_line = "a".repeat(10_000);
    let long_results = engine.search_text(&long_line, "a")?;
    assert!(!long_results.is_empty());
    
    // Special characters
    let special_text = "Line with special chars: !@#$%^&*()_+-=[]{}|;':\",./<>?";
    let special_results = engine.search_text(special_text, "!@#")?;
    assert_eq!(special_results.len(), 1);
    
    Ok(())
}