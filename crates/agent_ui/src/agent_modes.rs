/// Agent Modes Enhancement
///
/// This module defines and manages different agent modes for optimal UX:
/// - Write: Full tool access for comprehensive edits
/// - Ask: Read-only mode for questions and analysis
/// - Quick Edit: Scoped edits focused on specific code ranges
/// - Manual: User-controlled mode similar to Cursor's approach

use serde::{Deserialize, Serialize};

/// Different agent modes available
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AgentMode {
    /// Full write access with all tools enabled
    Write,
    /// Read-only mode for asking questions
    Ask,
    /// Scoped edits for quick modifications
    QuickEdit,
    /// Manual mode where user controls all actions
    Manual,
}

impl AgentMode {
    /// Get display name for the mode
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Write => "Write",
            Self::Ask => "Ask",
            Self::QuickEdit => "Quick Edit",
            Self::Manual => "Manual",
        }
    }

    /// Get description for the mode
    pub fn description(&self) -> &'static str {
        match self {
            Self::Write => {
                "Full access to tools for comprehensive edits, refactoring, and code generation"
            }
            Self::Ask => "Read-only mode for analyzing code, answering questions, and understanding",
            Self::QuickEdit => {
                "Focused mode for making quick, scoped edits to selected code ranges"
            }
            Self::Manual => "User controls all actions - agent suggests, you decide what to apply",
        }
    }

    /// Get which tools are enabled in this mode
    pub fn enabled_tools(&self) -> Vec<&'static str> {
        match self {
            Self::Write => vec![
                "read_file",
                "write_file",
                "edit_file",
                "search_files",
                "run_command",
                "list_files",
            ],
            Self::Ask => vec![
                "read_file",
                "search_files",
                "list_files",
                "get_file_outline",
            ],
            Self::QuickEdit => vec![
                "read_file",
                "edit_file",
                "search_files",
            ],
            Self::Manual => vec![
                "read_file",
                "suggest_edit",
                "search_files",
            ],
        }
    }

    /// Get recommended use cases for this mode
    pub fn use_cases(&self) -> Vec<&'static str> {
        match self {
            Self::Write => vec![
                "Feature implementation",
                "Large refactoring",
                "Bug fixing with broad changes",
                "Test generation",
            ],
            Self::Ask => vec![
                "Code review",
                "Understanding codebase",
                "Performance analysis",
                "Documentation generation",
            ],
            Self::QuickEdit => vec![
                "Quick bug fixes",
                "Small refactoring",
                "Variable renaming",
                "Comment updates",
            ],
            Self::Manual => vec![
                "Precise control needed",
                "Critical code changes",
                "Learning from agent suggestions",
            ],
        }
    }

    /// Get keyboard shortcut hint for this mode
    pub fn shortcut_hint(&self) -> Option<&'static str> {
        match self {
            Self::QuickEdit => Some("cmd-k or ctrl-alt-k"),
            _ => None,
        }
    }
}

impl Default for AgentMode {
    fn default() -> Self {
        Self::Write
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mode_display_names() {
        assert_eq!(AgentMode::Write.display_name(), "Write");
        assert_eq!(AgentMode::Ask.display_name(), "Ask");
        assert_eq!(AgentMode::QuickEdit.display_name(), "Quick Edit");
        assert_eq!(AgentMode::Manual.display_name(), "Manual");
    }

    #[test]
    fn test_write_mode_tools() {
        let tools = AgentMode::Write.enabled_tools();
        assert!(tools.contains(&"write_file"));
        assert!(tools.contains(&"run_command"));
    }

    #[test]
    fn test_ask_mode_readonly() {
        let tools = AgentMode::Ask.enabled_tools();
        assert!(!tools.contains(&"write_file"));
        assert!(!tools.contains(&"run_command"));
        assert!(tools.contains(&"read_file"));
    }

    #[test]
    fn test_quick_edit_focused() {
        let tools = AgentMode::QuickEdit.enabled_tools();
        assert!(tools.contains(&"edit_file"));
        assert!(!tools.contains(&"run_command"));
    }

    #[test]
    fn test_mode_descriptions_not_empty() {
        assert!(!AgentMode::Write.description().is_empty());
        assert!(!AgentMode::Ask.description().is_empty());
        assert!(!AgentMode::QuickEdit.description().is_empty());
        assert!(!AgentMode::Manual.description().is_empty());
    }

    #[test]
    fn test_default_mode() {
        assert_eq!(AgentMode::default(), AgentMode::Write);
    }
}
