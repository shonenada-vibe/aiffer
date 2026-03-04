use crate::ai::{self, AiConfig};
use crate::git;
use crate::models::{
    AppSettings, CommentInput, CommentWithContext, CommitInfo, DiffFile, DiffLine, FileEntry,
    FileSummary, ReviewPayload,
};
use std::fs;
use std::path::PathBuf;

/// Error type for Tauri commands — wraps git errors into user-friendly strings.
#[derive(Debug, thiserror::Error)]
pub enum CommandError {
    #[error("{0}")]
    Git(#[from] git::GitError),
    #[error("{0}")]
    Ai(#[from] ai::AiError),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

// Tauri requires command errors to implement Serialize.
impl serde::Serialize for CommandError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to Aiffer.", name)
}

/// Validate whether a path is a valid git repository.
/// Returns Ok(canonical_path) if valid, or an error message if not.
#[tauri::command]
pub fn validate_git_repo(path: String) -> Result<String, CommandError> {
    let p = std::path::Path::new(&path);
    if !p.exists() {
        return Err(CommandError::Git(git::GitError::NotARepo(format!(
            "Path does not exist: {path}"
        ))));
    }
    if !p.is_dir() {
        return Err(CommandError::Git(git::GitError::NotARepo(format!(
            "Not a directory: {path}"
        ))));
    }
    let _repo = git::open_repo(&path)?;
    // Return the canonical path
    let canonical = p
        .canonicalize()
        .unwrap_or_else(|_| p.to_path_buf())
        .to_string_lossy()
        .to_string();
    Ok(canonical)
}

/// Get the initial folder path from CLI arguments (if provided).
#[tauri::command]
pub fn get_initial_path() -> Option<String> {
    let args: Vec<String> = std::env::args().collect();
    // Skip the first arg (binary name). The second arg, if present, is the path.
    if args.len() > 1 {
        let path = &args[1];
        // Ignore Tauri internal flags
        if !path.starts_with('-') {
            let p = std::path::Path::new(path);
            if p.exists() && p.is_dir() {
                return Some(
                    p.canonicalize()
                        .unwrap_or_else(|_| p.to_path_buf())
                        .to_string_lossy()
                        .to_string(),
                );
            }
        }
    }
    None
}

/// Get the full diff for a repository path.
/// `diff_type` should be "unstaged", "staged", or "commits".
/// When `diff_type` is "commits", `from_ref` and `to_ref` must be provided.
#[tauri::command]
pub fn get_diff(
    path: String,
    diff_type: String,
    from_ref: Option<String>,
    to_ref: Option<String>,
) -> Result<Vec<DiffFile>, CommandError> {
    let repo = git::open_repo(&path)?;
    let diff = if diff_type == "commits" {
        let from = from_ref.unwrap_or_default();
        let to = to_ref.unwrap_or_default();
        git::get_diff_between_commits(&repo, &from, &to)?
    } else {
        git::get_diff(&repo, &diff_type)?
    };
    let files = git::parse_diff(&diff)?;
    Ok(files)
}

/// Get the list of recent commits for a repository.
#[tauri::command]
pub fn get_commits(path: String, max_count: Option<usize>) -> Result<Vec<CommitInfo>, CommandError> {
    let repo = git::open_repo(&path)?;
    let commits = git::get_commits(&repo, max_count.unwrap_or(50))?;
    Ok(commits)
}

/// Build a review payload that enriches comments with surrounding code context.
#[tauri::command]
pub fn build_review_payload(
    path: String,
    comments: Vec<CommentInput>,
) -> Result<ReviewPayload, CommandError> {
    let repo = git::open_repo(&path)?;
    let diff = git::get_diff(&repo, "unstaged")?;
    let files = git::parse_diff(&diff)?;

    // Build file summary
    let file_summary: Vec<FileSummary> = files
        .iter()
        .map(|f| FileSummary {
            path: f.path.clone(),
            status: f.status.clone(),
            additions: f.stats.additions,
            deletions: f.stats.deletions,
        })
        .collect();

    let total_files_changed = files.len();
    let total_additions: usize = files.iter().map(|f| f.stats.additions).sum();
    let total_deletions: usize = files.iter().map(|f| f.stats.deletions).sum();

    // Enrich each comment with code context
    let enriched: Vec<CommentWithContext> = comments
        .iter()
        .map(|c| enrich_comment(c, &files))
        .collect();

    // Build formatted text for LLM
    let formatted_text = format_review_text(&enriched, &file_summary, total_additions, total_deletions);

    Ok(ReviewPayload {
        comments: enriched,
        file_summary,
        total_files_changed,
        total_additions,
        total_deletions,
        formatted_text,
    })
}

/// Find the commented line and its surrounding context from the diff data.
fn enrich_comment(comment: &CommentInput, files: &[DiffFile]) -> CommentWithContext {
    let mut line_content = String::new();
    let mut context_before: Vec<String> = Vec::new();
    let mut context_after: Vec<String> = Vec::new();

    if let Some(file) = files.iter().find(|f| f.path == comment.file_path) {
        // Collect all lines from all hunks into a flat list
        let all_lines: Vec<&DiffLine> = file.hunks.iter().flat_map(|h| h.lines.iter()).collect();

        // Find the target line by line number
        if let Some(idx) = all_lines.iter().position(|l| {
            let ln = l.new_lineno.or(l.old_lineno).unwrap_or(0);
            ln == comment.line_number
        }) {
            line_content = all_lines[idx].content.clone();

            // Gather up to 3 lines before
            let start = idx.saturating_sub(3);
            for l in &all_lines[start..idx] {
                let ln = l.new_lineno.or(l.old_lineno).unwrap_or(0);
                let prefix = match l.line_type {
                    crate::models::LineType::Addition => "+",
                    crate::models::LineType::Deletion => "-",
                    crate::models::LineType::Context => " ",
                };
                context_before.push(format!("{:>4} | {}{}", ln, prefix, l.content));
            }

            // Gather up to 3 lines after
            let end = (idx + 4).min(all_lines.len());
            for l in &all_lines[idx + 1..end] {
                let ln = l.new_lineno.or(l.old_lineno).unwrap_or(0);
                let prefix = match l.line_type {
                    crate::models::LineType::Addition => "+",
                    crate::models::LineType::Deletion => "-",
                    crate::models::LineType::Context => " ",
                };
                context_after.push(format!("{:>4} | {}{}", ln, prefix, l.content));
            }
        }
    }

    CommentWithContext {
        file_path: comment.file_path.clone(),
        line_number: comment.line_number,
        line_type: comment.line_type.clone(),
        body: comment.body.clone(),
        line_content,
        context_before,
        context_after,
    }
}

/// Format the review into a clean, readable string for LLM consumption.
fn format_review_text(
    comments: &[CommentWithContext],
    file_summary: &[FileSummary],
    total_additions: usize,
    total_deletions: usize,
) -> String {
    let mut out = String::new();

    out.push_str("# Code Review Comments\n\n");
    out.push_str(&format!(
        "## Summary: {} file(s) changed, +{} -{}\n\n",
        file_summary.len(),
        total_additions,
        total_deletions
    ));

    out.push_str("### Changed Files:\n");
    for f in file_summary {
        out.push_str(&format!(
            "- {} ({:?}, +{} -{})\n",
            f.path, f.status, f.additions, f.deletions
        ));
    }
    out.push('\n');

    out.push_str(&format!("## Review Comments ({})\n\n", comments.len()));

    for (i, c) in comments.iter().enumerate() {
        let type_label = match c.line_type {
            crate::models::LineType::Addition => "addition",
            crate::models::LineType::Deletion => "deletion",
            crate::models::LineType::Context => "context",
        };

        out.push_str(&format!(
            "### Comment {} — {}:{} ({})\n\n",
            i + 1,
            c.file_path,
            c.line_number,
            type_label
        ));

        if !c.context_before.is_empty() || !c.context_after.is_empty() {
            out.push_str("```\n");
            for line in &c.context_before {
                out.push_str(&format!("{}\n", line));
            }
            let prefix = match c.line_type {
                crate::models::LineType::Addition => "+",
                crate::models::LineType::Deletion => "-",
                crate::models::LineType::Context => " ",
            };
            out.push_str(&format!(
                ">{:>3} | {}{}\n",
                c.line_number, prefix, c.line_content
            ));
            for line in &c.context_after {
                out.push_str(&format!("{}\n", line));
            }
            out.push_str("```\n\n");
        }

        out.push_str(&format!("**Comment:** {}\n\n", c.body));
        out.push_str("---\n\n");
    }

    out
}

/// Submit a review to an AI agent and return its response.
#[tauri::command]
pub async fn submit_review(
    review_text: String,
    config: AiConfig,
) -> Result<String, CommandError> {
    let response = ai::submit_review(&config, &review_text).await?;
    Ok(response)
}

/// Get a lightweight list of changed files with statuses.
#[tauri::command]
pub fn get_file_status(path: String) -> Result<Vec<FileEntry>, CommandError> {
    let repo = git::open_repo(&path)?;
    let diff = git::get_diff(&repo, "unstaged")?;
    let mut entries = git::get_file_entries(&diff)?;

    // Also include staged changes
    if let Ok(staged_diff) = git::get_diff(&repo, "staged") {
        if let Ok(staged_entries) = git::get_file_entries(&staged_diff) {
            for entry in staged_entries {
                if !entries.iter().any(|e| e.path == entry.path) {
                    entries.push(entry);
                }
            }
        }
    }

    entries.sort_by(|a, b| a.path.cmp(&b.path));
    Ok(entries)
}

/// Get the path to the settings file.
fn settings_path() -> PathBuf {
    let config_dir = dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("aiffer");
    config_dir.join("settings.json")
}

/// Load application settings from disk.
#[tauri::command]
pub fn load_settings() -> Result<AppSettings, CommandError> {
    let path = settings_path();
    if !path.exists() {
        return Ok(AppSettings::default());
    }
    let content = fs::read_to_string(&path)?;
    let settings: AppSettings = serde_json::from_str(&content)?;
    Ok(settings)
}

/// Save application settings to disk.
#[tauri::command]
pub fn save_settings(settings: AppSettings) -> Result<(), CommandError> {
    let path = settings_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let content = serde_json::to_string_pretty(&settings)?;
    fs::write(&path, content)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{DiffFile, DiffHunk, DiffLine, DiffStats, LineType};

    fn make_test_files() -> Vec<DiffFile> {
        vec![DiffFile {
            path: "src/main.rs".to_string(),
            old_path: None,
            status: crate::models::FileStatus::Modified,
            hunks: vec![DiffHunk {
                header: "@@ -1,5 +1,6 @@".to_string(),
                old_start: 1,
                old_lines: 5,
                new_start: 1,
                new_lines: 6,
                lines: vec![
                    DiffLine { line_type: LineType::Context, old_lineno: Some(1), new_lineno: Some(1), content: "fn main() {".to_string() },
                    DiffLine { line_type: LineType::Context, old_lineno: Some(2), new_lineno: Some(2), content: "    let x = 1;".to_string() },
                    DiffLine { line_type: LineType::Context, old_lineno: Some(3), new_lineno: Some(3), content: "    let y = 2;".to_string() },
                    DiffLine { line_type: LineType::Addition, old_lineno: None, new_lineno: Some(4), content: "    let z = x + y;".to_string() },
                    DiffLine { line_type: LineType::Context, old_lineno: Some(4), new_lineno: Some(5), content: "    println!(\"done\");".to_string() },
                    DiffLine { line_type: LineType::Context, old_lineno: Some(5), new_lineno: Some(6), content: "}".to_string() },
                ],
            }],
            stats: DiffStats { additions: 1, deletions: 0 },
        }]
    }

    #[test]
    fn test_enrich_comment_finds_context() {
        let files = make_test_files();
        let comment = CommentInput {
            file_path: "src/main.rs".to_string(),
            line_number: 4,
            line_type: LineType::Addition,
            body: "Use a descriptive name".to_string(),
        };

        let enriched = enrich_comment(&comment, &files);

        assert_eq!(enriched.line_content, "    let z = x + y;");
        assert_eq!(enriched.context_before.len(), 3);
        assert_eq!(enriched.context_after.len(), 2);
        assert!(enriched.context_before[0].contains("fn main()"));
        assert!(enriched.context_after[0].contains("println!"));
    }

    #[test]
    fn test_enrich_comment_missing_file() {
        let files = make_test_files();
        let comment = CommentInput {
            file_path: "nonexistent.rs".to_string(),
            line_number: 1,
            line_type: LineType::Context,
            body: "test".to_string(),
        };

        let enriched = enrich_comment(&comment, &files);
        assert_eq!(enriched.line_content, "");
        assert!(enriched.context_before.is_empty());
        assert!(enriched.context_after.is_empty());
    }

    #[test]
    fn test_format_review_text_structure() {
        let files = make_test_files();
        let comment = CommentInput {
            file_path: "src/main.rs".to_string(),
            line_number: 4,
            line_type: LineType::Addition,
            body: "Rename this variable".to_string(),
        };
        let enriched = vec![enrich_comment(&comment, &files)];
        let summary = vec![FileSummary {
            path: "src/main.rs".to_string(),
            status: crate::models::FileStatus::Modified,
            additions: 1,
            deletions: 0,
        }];

        let text = format_review_text(&enriched, &summary, 1, 0);

        assert!(text.contains("# Code Review Comments"));
        assert!(text.contains("1 file(s) changed"));
        assert!(text.contains("src/main.rs"));
        assert!(text.contains("Rename this variable"));
        assert!(text.contains("let z = x + y;"));
    }
}
