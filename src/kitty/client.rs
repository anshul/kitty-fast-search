use anyhow::{Result, anyhow};
use serde_json::Value;
use std::process::Command;
use tokio::process::Command as AsyncCommand;

pub struct KittyClient {
    socket_path: Option<String>,
}

impl KittyClient {
    pub async fn new() -> Result<Self> {
        // Try to detect kitty socket
        let socket_path = Self::detect_kitty_socket().await?;
        Ok(Self { socket_path })
    }

    async fn detect_kitty_socket() -> Result<Option<String>> {
        // Check if we're running inside kitty
        if std::env::var("KITTY_WINDOW_ID").is_ok() {
            // Use default socket path
            Ok(None)
        } else {
            Err(anyhow!("Not running inside a Kitty terminal"))
        }
    }

    pub async fn get_buffer_content(&self) -> Result<String> {
        let mut cmd = AsyncCommand::new("kitty");
        cmd.arg("@").arg("get-text");
        
        if let Some(socket) = &self.socket_path {
            cmd.arg("--to").arg(socket);
        }

        let output = cmd.output().await?;
        
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(anyhow!("Failed to get buffer content: {}", 
                       String::from_utf8_lossy(&output.stderr)))
        }
    }

    pub async fn jump_to_line(&self, line_number: u64) -> Result<()> {
        let mut cmd = AsyncCommand::new("kitty");
        cmd.arg("@").arg("scroll-to-line").arg(line_number.to_string());
        
        if let Some(socket) = &self.socket_path {
            cmd.arg("--to").arg(socket);
        }

        let output = cmd.output().await?;
        
        if !output.status.success() {
            return Err(anyhow!("Failed to jump to line: {}", 
                              String::from_utf8_lossy(&output.stderr)));
        }

        Ok(())
    }

    pub async fn create_text_marker(&self, text: &str) -> Result<()> {
        let mut cmd = AsyncCommand::new("kitty");
        cmd.arg("@").arg("create-marker").arg("text").arg("1").arg(text);
        
        if let Some(socket) = &self.socket_path {
            cmd.arg("--to").arg(socket);
        }

        let output = cmd.output().await?;
        
        if !output.status.success() {
            return Err(anyhow!("Failed to create marker: {}", 
                              String::from_utf8_lossy(&output.stderr)));
        }

        Ok(())
    }

    pub async fn remove_marker(&self) -> Result<()> {
        let mut cmd = AsyncCommand::new("kitty");
        cmd.arg("@").arg("remove-marker");
        
        if let Some(socket) = &self.socket_path {
            cmd.arg("--to").arg(socket);
        }

        let output = cmd.output().await?;
        
        if !output.status.success() {
            return Err(anyhow!("Failed to remove marker: {}", 
                              String::from_utf8_lossy(&output.stderr)));
        }

        Ok(())
    }

    #[allow(dead_code)]
    pub async fn get_window_info(&self) -> Result<Value> {
        let mut cmd = AsyncCommand::new("kitty");
        cmd.arg("@").arg("ls");
        
        if let Some(socket) = &self.socket_path {
            cmd.arg("--to").arg(socket);
        }

        let output = cmd.output().await?;
        
        if output.status.success() {
            let json_str = String::from_utf8_lossy(&output.stdout);
            Ok(serde_json::from_str(&json_str)?)
        } else {
            Err(anyhow!("Failed to get window info: {}", 
                       String::from_utf8_lossy(&output.stderr)))
        }
    }

    #[allow(dead_code)]
    pub fn is_available() -> bool {
        Command::new("kitty")
            .arg("@")
            .arg("ls")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }
}