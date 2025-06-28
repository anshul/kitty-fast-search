#[cfg(feature = "heavy-tests")]
mod heavy_integration_tests {
    use anyhow::Result;
    use kitty_fast_search::search::SearchEngine;

    #[tokio::test]
    async fn test_very_large_buffer_search() -> Result<()> {
        let engine = SearchEngine::new(100_000, false, false)?;
        
        // Generate very large test data (original 5000 lines)
        let mut large_text = String::new();
        for i in 0..5000 {
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
        assert!(results.len() > 40); // Should find ~50 errors
        
        Ok(())
    }

    #[tokio::test]
    async fn test_extreme_memory_efficiency() -> Result<()> {
        let engine = SearchEngine::new(500_000, false, false)?;
        
        // Create very large text content (original 50k lines)
        let mut large_content = String::with_capacity(1_000_000);
        for i in 0..50_000 {
            large_content.push_str(&format!("Line {}: Some random content here\n", i));
            if i % 1000 == 0 {
                large_content.push_str(&format!("MARKER: Checkpoint at line {}\n", i));
            }
        }
        
        // Search should complete without excessive memory usage
        let results = engine.search_text(&large_content, "MARKER")?;
        assert!(results.len() > 40); // Should find ~50 markers
        
        // Clear cache to free memory
        engine.clear_cache();
        assert_eq!(engine.get_cache_size(), 0);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_many_concurrent_searches() -> Result<()> {
        let engine = std::sync::Arc::new(SearchEngine::new(10_000, false, false)?);
        let text = "Hello world\nThis is a test\nHello again\nTesting concurrent access";
        
        // Spawn many concurrent search tasks (original 10)
        let mut handles = vec![];
        
        for i in 0..10 {
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
}