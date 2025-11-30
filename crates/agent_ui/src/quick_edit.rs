/// Quick Edit Mode for Agent
///
/// This module provides quick inline editing capabilities that allow users to
/// select code and directly ask the agent to modify it, similar to Cursor's
/// quick edit functionality.

use std::ops::Range;
use std::rc::Rc;
use std::sync::Arc;

use anyhow::Result;
use editor::{Editor, MultiBuffer};
use gpui::{Action, App, Context, Entity, Subscription, WeakEntity, Window, actions};
use language::{Point};
use project::Project;
use text::ToPoint;
use workspace::Workspace;

use crate::inline_assistant::InlineAssistant;

/// Quick edit action triggered from editor with selected code
#[derive(Clone, PartialEq, Action)]
#[action(namespace = agent, no_json)]
pub struct QuickEdit {
    /// The prompt/instruction for editing
    pub prompt: String,
}

/// Send selected code to the agent panel in the main thread
#[derive(Clone, PartialEq, Action)]
#[action(namespace = agent)]
pub struct SendSelectionToAgent;

/// Open quick edit UI for current selection
#[derive(Clone, PartialEq, Action)]
#[action(namespace = agent)]
pub struct OpenQuickEditForSelection;

pub struct QuickEditState {
    editor: WeakEntity<Editor>,
    workspace: WeakEntity<Workspace>,
    /// The range of text to edit
    selection_range: Range<usize>,
    /// Current edit prompt
    prompt: String,
    /// Subscription to editor changes
    _subscriptions: Vec<Subscription>,
}

impl QuickEditState {
    pub fn new(
        editor: Entity<Editor>,
        workspace: Entity<Workspace>,
        selection_range: Range<usize>,
    ) -> Self {
        Self {
            editor: editor.downgrade(),
            workspace: workspace.downgrade(),
            selection_range,
            prompt: String::new(),
            _subscriptions: vec![],
        }
    }

    /// Get the selected text from the editor
    pub fn selected_text(&self, cx: &App) -> Option<String> {
        let editor = self.editor.upgrade()?;
        let snapshot = editor.read(cx).snapshot(cx);
        let buffer = snapshot.buffer_snapshot();

        let start_offset = self.selection_range.start;
        let end_offset = self.selection_range.end;

        if start_offset >= buffer.len() || end_offset > buffer.len() || start_offset > end_offset
        {
            return None;
        }

        Some(buffer.text_for_range(start_offset..end_offset).collect())
    }

    /// Get file context information for the selection
    pub fn get_context_info(&self, cx: &App) -> Option<ContextInfo> {
        let editor = self.editor.upgrade()?;
        let snapshot = editor.read(cx).snapshot(cx);
        let buffer = snapshot.buffer_snapshot();

        // Get file path
        let file_path = buffer.file().and_then(|file| {
            file.full_path(cx)
                .ok()
                .map(|p| p.to_string_lossy().to_string())
        });

        // Get line range for the selection
        let start_offset = self.selection_range.start;
        let end_offset = self.selection_range.end;

        let start_point = buffer.offset_to_point(start_offset);
        let end_point = buffer.offset_to_point(end_offset);

        Some(ContextInfo {
            file_path,
            start_line: start_point.row,
            end_line: end_point.row,
            start_column: start_point.column,
            end_column: end_point.column,
        })
    }
}

/// Information about the context of a selection
#[derive(Clone, Debug)]
pub struct ContextInfo {
    pub file_path: Option<String>,
    pub start_line: u32,
    pub end_line: u32,
    pub start_column: u32,
    pub end_column: u32,
}

impl ContextInfo {
    /// Format context info as a human-readable string
    pub fn format(&self) -> String {
        let mut parts = Vec::new();

        if let Some(path) = &self.file_path {
            parts.push(format!("File: {}", path));
        }

        if self.start_line == self.end_line {
            parts.push(format!("Line {}", self.start_line + 1));
        } else {
            parts.push(format!("Lines {}-{}", self.start_line + 1, self.end_line + 1));
        }

        parts.join(" • ")
    }

    /// Format selection with code block and context for agent
    pub fn format_for_agent(&self, code: &str) -> String {
        let mut result = String::new();

        // Add file/line context as a comment in the code block
        let context = self.format();
        result.push_str(&format!("```\n{}\n```\n\n", context));

        // Add the code block with syntax hint
        let language = self.infer_language();
        result.push_str(&format!("```{}\n{}\n```", language, code));

        result
    }

    /// Infer programming language from file path
    fn infer_language(&self) -> String {
        if let Some(path) = &self.file_path {
            if path.ends_with(".rs") {
                "rust".to_string()
            } else if path.ends_with(".ts") || path.ends_with(".tsx") {
                "typescript".to_string()
            } else if path.ends_with(".js") || path.ends_with(".jsx") {
                "javascript".to_string()
            } else if path.ends_with(".py") {
                "python".to_string()
            } else if path.ends_with(".go") {
                "go".to_string()
            } else if path.ends_with(".c") || path.ends_with(".h") {
                "c".to_string()
            } else if path.ends_with(".cpp") || path.ends_with(".cc") {
                "cpp".to_string()
            } else if path.ends_with(".java") {
                "java".to_string()
            } else if path.ends_with(".sql") {
                "sql".to_string()
            } else {
                String::new()
            }
        } else {
            String::new()
        }
    }
}

/// Handle quick edit action from editor
pub(crate) fn handle_quick_edit(
    _workspace: &mut Workspace,
    _action: &QuickEdit,
    _window: &mut Window,
    _cx: &mut Context<Workspace>,
) {
    // TODO: Implement quick edit with inline assistant
    // This will be connected to the InlineAssistant once fully integrated
}

/// Handle sending selection to agent panel
pub(crate) fn handle_send_selection_to_agent(
    _workspace: &mut Workspace,
    _action: &SendSelectionToAgent,
    _window: &mut Window,
    _cx: &mut Context<Workspace>,
) {
    // TODO: Add selection to agent panel message editor
    // This would require integrating with the agent panel directly
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_info_format() {
        let context = ContextInfo {
            file_path: Some("src/main.rs".to_string()),
            start_line: 5,
            end_line: 10,
            start_column: 0,
            end_column: 20,
        };

        let formatted = context.format();
        assert!(formatted.contains("src/main.rs"));
        assert!(formatted.contains("Lines 6-11")); // 1-indexed for display
    }

    #[test]
    fn test_context_info_single_line() {
        let context = ContextInfo {
            file_path: Some("utils.rs".to_string()),
            start_line: 42,
            end_line: 42,
            start_column: 10,
            end_column: 30,
        };

        let formatted = context.format();
        assert!(formatted.contains("Line 43")); // 1-indexed
        assert!(formatted.contains("utils.rs"));
    }

    #[test]
    fn test_format_for_agent_with_rust() {
        let context = ContextInfo {
            file_path: Some("src/lib.rs".to_string()),
            start_line: 0,
            end_line: 5,
            start_column: 0,
            end_column: 0,
        };

        let code = "fn hello() {\n    println!(\"Hello\");\n}";
        let formatted = context.format_for_agent(code);

        assert!(formatted.contains("```rust"));
        assert!(formatted.contains("src/lib.rs"));
        assert!(formatted.contains(code));
    }

    #[test]
    fn test_infer_language() {
        let contexts = vec![
            ("src/main.rs", "rust"),
            ("app.tsx", "typescript"),
            ("utils.py", "python"),
            ("main.go", "go"),
        ];

        for (path, expected_lang) in contexts {
            let context = ContextInfo {
                file_path: Some(path.to_string()),
                start_line: 0,
                end_line: 0,
                start_column: 0,
                end_column: 0,
            };
            assert_eq!(context.infer_language(), expected_lang);
        }
    }
}
