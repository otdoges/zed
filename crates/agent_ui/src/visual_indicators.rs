/// Visual Indicators for Agent Edits
///
/// This module provides visual feedback for:
/// - Currently selected lines for agent
/// - Lines being edited by agent
/// - Agent's proposed changes
/// - Token count and context information

use gpui::{HighlightStyle, Hsla};

/// Style for code selected for agent
pub fn selected_for_agent_style() -> HighlightStyle {
    HighlightStyle {
        color: None,
        background_color: Some(Hsla {
            h: 60.0,
            s: 0.5,
            l: 0.7,
            a: 0.15,
        }),
        font_weight: None,
        font_style: None,
        underline: None,
        strikethrough: None,
        fade_out: None,
    }
}

/// Style for lines being edited by agent
pub fn agent_editing_style() -> HighlightStyle {
    HighlightStyle {
        color: None,
        background_color: Some(Hsla {
            h: 200.0,
            s: 0.6,
            l: 0.7,
            a: 0.2,
        }),
        font_weight: None,
        font_style: None,
        underline: None,
        strikethrough: None,
        fade_out: None,
    }
}

/// Style for proposed changes from agent
pub fn proposed_changes_style() -> HighlightStyle {
    HighlightStyle {
        color: None,
        background_color: Some(Hsla {
            h: 120.0,
            s: 0.6,
            l: 0.7,
            a: 0.15,
        }),
        font_weight: None,
        font_style: None,
        underline: None,
        strikethrough: None,
        fade_out: None,
    }
}

/// Information about token usage
#[derive(Clone, Debug, Default)]
pub struct TokenInfo {
    /// Approximate tokens in selection
    pub selection_tokens: u32,
    /// Total tokens used in thread
    pub total_tokens: u32,
    /// Model's context window size
    pub context_window: u32,
}

impl TokenInfo {
    /// Calculate percentage of context window used
    pub fn percentage_used(&self) -> f32 {
        if self.context_window == 0 {
            0.0
        } else {
            (self.total_tokens as f32 / self.context_window as f32) * 100.0
        }
    }

    /// Estimate tokens from text (rough approximation)
    pub fn estimate_tokens_from_text(text: &str) -> u32 {
        // Rough estimate: ~4 characters per token on average
        (text.len() / 4).max(1) as u32
    }

    /// Format token info as display string
    pub fn format_display(&self) -> String {
        let percentage = self.percentage_used();
        let warning = if percentage > 80.0 {
            " ⚠️"
        } else {
            ""
        };
        format!(
            "{}/{} tokens ({:.0}%)",
            self.total_tokens, self.context_window, percentage
        ) + warning
    }
}

/// Information about selected code for display
#[derive(Clone, Debug)]
pub struct SelectionInfo {
    pub char_count: usize,
    pub line_count: u32,
    pub token_estimate: u32,
}

impl SelectionInfo {
    /// Create selection info from text
    pub fn from_text(text: &str) -> Self {
        let char_count = text.len();
        let line_count = text.lines().count() as u32;
        let token_estimate = TokenInfo::estimate_tokens_from_text(text);

        Self {
            char_count,
            line_count,
            token_estimate,
        }
    }

    /// Format selection info for display
    pub fn format_display(&self) -> String {
        format!(
            "{} lines • {} chars • ~{} tokens",
            self.line_count, self.char_count, self.token_estimate
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_percentage() {
        let info = TokenInfo {
            selection_tokens: 100,
            total_tokens: 5000,
            context_window: 10000,
        };
        assert_eq!(info.percentage_used(), 50.0);
    }

    #[test]
    fn test_token_estimate() {
        let text = "fn hello() { println!(\"Hello, world!\"); }";
        let estimated = TokenInfo::estimate_tokens_from_text(text);
        assert!(estimated > 0);
        assert!(estimated < text.len());
    }

    #[test]
    fn test_selection_info_from_text() {
        let text = "line 1\nline 2\nline 3";
        let info = SelectionInfo::from_text(text);
        assert_eq!(info.line_count, 3);
        assert_eq!(info.char_count, text.len());
    }

    #[test]
    fn test_token_info_display() {
        let info = TokenInfo {
            selection_tokens: 100,
            total_tokens: 8000,
            context_window: 10000,
        };
        let display = info.format_display();
        assert!(display.contains("8000"));
        assert!(display.contains("10000"));
        assert!(display.contains("80"));
    }

    #[test]
    fn test_token_warning() {
        let info = TokenInfo {
            selection_tokens: 0,
            total_tokens: 9000,
            context_window: 10000,
        };
        let display = info.format_display();
        assert!(display.contains("⚠️"));
    }

    #[test]
    fn test_selection_info_display() {
        let info = SelectionInfo {
            char_count: 150,
            line_count: 5,
            token_estimate: 40,
        };
        let display = info.format_display();
        assert!(display.contains("5 lines"));
        assert!(display.contains("150 chars"));
        assert!(display.contains("40 tokens"));
    }
}
