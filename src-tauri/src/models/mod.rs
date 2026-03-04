use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum FileStatus {
    Added,
    Modified,
    Deleted,
    Renamed,
    Copied,
    Untracked,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum LineType {
    Context,
    Addition,
    Deletion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiffLine {
    pub line_type: LineType,
    pub old_lineno: Option<u32>,
    pub new_lineno: Option<u32>,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiffHunk {
    pub header: String,
    pub old_start: u32,
    pub old_lines: u32,
    pub new_start: u32,
    pub new_lines: u32,
    pub lines: Vec<DiffLine>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiffStats {
    pub additions: usize,
    pub deletions: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiffFile {
    pub path: String,
    pub old_path: Option<String>,
    pub status: FileStatus,
    pub hunks: Vec<DiffHunk>,
    pub stats: DiffStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileEntry {
    pub path: String,
    pub status: FileStatus,
    pub additions: usize,
    pub deletions: usize,
}

/// Comment input from the frontend.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentInput {
    pub file_path: String,
    pub line_number: u32,
    pub line_type: LineType,
    pub body: String,
}

/// A comment enriched with surrounding code context.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentWithContext {
    pub file_path: String,
    pub line_number: u32,
    pub line_type: LineType,
    pub body: String,
    pub line_content: String,
    pub context_before: Vec<String>,
    pub context_after: Vec<String>,
}

/// Summary of a changed file for the review payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileSummary {
    pub path: String,
    pub status: FileStatus,
    pub additions: usize,
    pub deletions: usize,
}

/// The full review payload sent to the AI agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReviewPayload {
    pub comments: Vec<CommentWithContext>,
    pub file_summary: Vec<FileSummary>,
    pub total_files_changed: usize,
    pub total_additions: usize,
    pub total_deletions: usize,
    pub formatted_text: String,
}

/// A single commit entry for the commit selector.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommitInfo {
    pub oid: String,
    pub short_id: String,
    pub summary: String,
    pub author: String,
    pub timestamp: i64,
}

/// Application settings persisted to disk.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    #[serde(default = "default_endpoint")]
    pub ai_endpoint: String,
    #[serde(default)]
    pub ai_api_key: String,
    #[serde(default = "default_model")]
    pub ai_model: String,
    #[serde(default = "default_diff_type")]
    pub diff_type: String,
    #[serde(default = "default_theme")]
    pub theme: String,
}

fn default_endpoint() -> String {
    "https://api.openai.com/v1".to_string()
}

fn default_model() -> String {
    "gpt-4o".to_string()
}

fn default_diff_type() -> String {
    "unstaged".to_string()
}

fn default_theme() -> String {
    "light".to_string()
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            ai_endpoint: default_endpoint(),
            ai_api_key: String::new(),
            ai_model: default_model(),
            diff_type: default_diff_type(),
            theme: default_theme(),
        }
    }
}
