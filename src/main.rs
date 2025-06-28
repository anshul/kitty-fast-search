use anyhow::Result;
use clap::Parser;
use tracing::{info, Level};
use tracing_subscriber;

mod search;
mod ui;
mod kitty;

use search::SearchEngine;
use ui::SearchUI;
use kitty::KittyClient;

#[derive(Parser)]
#[command(name = "kitty-fast-search")]
#[command(about = "Blazing-fast terminal search plugin for Kitty")]
#[command(version)]
struct Args {
    /// Initial search query
    #[arg(short, long)]
    query: Option<String>,
    
    /// Maximum buffer size to search (in lines)
    #[arg(long, default_value = "1000000")]
    buffer_size: usize,
    
    /// Enable debug logging
    #[arg(long)]
    debug: bool,
    
    /// Case-sensitive search
    #[arg(long)]
    case_sensitive: bool,
    
    /// Use regex patterns
    #[arg(long)]
    regex: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    
    // Initialize logging
    let log_level = if args.debug { Level::DEBUG } else { Level::INFO };
    tracing_subscriber::fmt()
        .with_max_level(log_level)
        .init();
    
    info!("Starting Kitty Fast Search v{}", env!("CARGO_PKG_VERSION"));
    
    // Initialize components
    let kitty_client = KittyClient::new().await?;
    let search_engine = SearchEngine::new(args.buffer_size, args.case_sensitive, args.regex)?;
    let mut search_ui = SearchUI::new(kitty_client, search_engine).await?;
    
    // Set initial query if provided
    if let Some(query) = args.query {
        search_ui.set_initial_query(query);
    }
    
    // Run the search interface
    search_ui.run().await?;
    
    Ok(())
}
