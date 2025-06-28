use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct KittyCommand {
    pub cmd: String,
    pub args: Vec<String>,
    pub payload: Option<String>,
}

#[allow(dead_code)]
impl KittyCommand {
    pub fn new(cmd: &str) -> Self {
        Self {
            cmd: cmd.to_string(),
            args: Vec::new(),
            payload: None,
        }
    }

    pub fn with_args(mut self, args: Vec<String>) -> Self {
        self.args = args;
        self
    }

    pub fn with_payload(mut self, payload: String) -> Self {
        self.payload = Some(payload);
        self
    }

    pub fn get_text() -> Self {
        Self::new("get-text")
    }

    pub fn scroll_to_line(line: u64) -> Self {
        Self::new("scroll-to-line").with_args(vec![line.to_string()])
    }

    pub fn list_windows() -> Self {
        Self::new("ls")
    }

    pub fn send_text(text: &str) -> Self {
        Self::new("send-text").with_payload(text.to_string())
    }

    pub fn set_window_title(title: &str) -> Self {
        Self::new("set-window-title").with_args(vec![title.to_string()])
    }

    pub fn resize_window(width: u32, height: u32) -> Self {
        Self::new("resize-window").with_args(vec![
            format!("--width={}", width),
            format!("--height={}", height),
        ])
    }

    pub fn focus_window(window_id: &str) -> Self {
        Self::new("focus-window").with_args(vec![format!("--match=id:{}", window_id)])
    }

    pub fn get_colors() -> Self {
        Self::new("get-colors")
    }

    pub fn set_colors(colors: HashMap<String, String>) -> Self {
        let color_args: Vec<String> = colors
            .into_iter()
            .map(|(key, value)| format!("{}={}", key, value))
            .collect();
        
        Self::new("set-colors").with_args(color_args)
    }

    pub fn create_marker(text: &str) -> Self {
        Self::new("create-marker").with_args(vec![text.to_string()])
    }

    pub fn remove_marker() -> Self {
        Self::new("remove-marker")
    }

    pub fn scroll_to_prompt(direction: ScrollDirection) -> Self {
        let dir_arg = match direction {
            ScrollDirection::Previous => "prev",
            ScrollDirection::Next => "next",
        };
        Self::new("scroll-to-prompt").with_args(vec![dir_arg.to_string()])
    }

    pub fn to_command_line(&self) -> Vec<String> {
        let mut cmd_line = vec!["kitty".to_string(), "@".to_string(), self.cmd.clone()];
        cmd_line.extend(self.args.clone());
        cmd_line
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum ScrollDirection {
    Previous,
    Next,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct WindowInfo {
    pub id: u32,
    pub title: String,
    pub pid: u32,
    pub cwd: String,
    pub cmdline: Vec<String>,
    pub env: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct TabInfo {
    pub id: u32,
    pub title: String,
    pub layout: String,
    pub windows: Vec<WindowInfo>,
    pub active_window: u32,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct OSWindowInfo {
    pub id: u32,
    pub tabs: Vec<TabInfo>,
    pub active_tab: u32,
}