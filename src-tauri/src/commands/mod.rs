use crate::git;
use crate::models::{DiffFile, FileEntry};

/// Error type for Tauri commands — wraps git errors into user-friendly strings.
#[derive(Debug, thiserror::Error)]
pub enum CommandError {
    #[error("{0}")]
    Git(#[from] git::GitError),
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

/// Get the full diff for a repository path.
/// `diff_type` should be "unstaged" or "staged".
#[tauri::command]
pub fn get_diff(path: String, diff_type: String) -> Result<Vec<DiffFile>, CommandError> {
    let repo = git::open_repo(&path)?;
    let diff = git::get_diff(&repo, &diff_type)?;
    let files = git::parse_diff(&diff)?;
    Ok(files)
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
