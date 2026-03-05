use std::path::Path;

use git2::{Delta, Diff, DiffOptions, Repository};

use crate::models::{
    CommitInfo, DiffFile, DiffHunk, DiffLine, DiffStats, FileEntry, FileStatus, LineType,
};

#[derive(Debug, thiserror::Error)]
pub enum GitError {
    #[error("Git error: {0}")]
    Git(#[from] git2::Error),

    #[error("Not a git repository: {0}")]
    NotARepo(String),

    #[error("Invalid diff type: {0}. Use 'unstaged', 'staged', or 'commits'")]
    InvalidDiffType(String),

    #[error("Invalid commit reference: {0}")]
    InvalidRef(String),
}

/// Open a git repository at the given path.
pub fn open_repo(path: &str) -> Result<Repository, GitError> {
    let repo_path = Path::new(path);
    Repository::discover(repo_path).map_err(|_| GitError::NotARepo(path.to_string()))
}

/// Get the diff for a repository.
/// `diff_type` can be "unstaged" (workdir vs index) or "staged" (index vs HEAD).
pub fn get_diff<'a>(repo: &'a Repository, diff_type: &str) -> Result<Diff<'a>, GitError> {
    let mut opts = DiffOptions::new();
    opts.include_untracked(true);
    opts.recurse_untracked_dirs(true);
    opts.show_untracked_content(true);

    match diff_type {
        "unstaged" => {
            let diff = repo.diff_index_to_workdir(None, Some(&mut opts))?;
            Ok(diff)
        }
        "staged" => {
            let head = repo.head()?.peel_to_tree()?;
            let diff = repo.diff_tree_to_index(Some(&head), None, Some(&mut opts))?;
            Ok(diff)
        }
        other => Err(GitError::InvalidDiffType(other.to_string())),
    }
}

/// Get the diff between two commit references (sha, branch, tag, etc.).
pub fn get_diff_between_commits<'a>(
    repo: &'a Repository,
    from_ref: &str,
    to_ref: &str,
) -> Result<Diff<'a>, GitError> {
    let from_obj = repo
        .revparse_single(from_ref)
        .map_err(|_| GitError::InvalidRef(from_ref.to_string()))?;
    let to_obj = repo
        .revparse_single(to_ref)
        .map_err(|_| GitError::InvalidRef(to_ref.to_string()))?;

    let from_tree = from_obj
        .peel_to_tree()
        .map_err(|_| GitError::InvalidRef(from_ref.to_string()))?;
    let to_tree = to_obj
        .peel_to_tree()
        .map_err(|_| GitError::InvalidRef(to_ref.to_string()))?;

    let mut opts = DiffOptions::new();
    let diff = repo.diff_tree_to_tree(Some(&from_tree), Some(&to_tree), Some(&mut opts))?;
    Ok(diff)
}

/// Get the list of recent commits (most recent first).
pub fn get_commits(repo: &Repository, max_count: usize) -> Result<Vec<CommitInfo>, GitError> {
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;
    revwalk.set_sorting(git2::Sort::TIME)?;

    let mut commits = Vec::new();
    for oid_result in revwalk.take(max_count) {
        let oid = oid_result?;
        let commit = repo.find_commit(oid)?;
        let short_id = oid.to_string()[..7].to_string();
        let summary = commit
            .summary()
            .unwrap_or("(no message)")
            .to_string();
        let author = commit.author().name().unwrap_or("Unknown").to_string();
        let timestamp = commit.time().seconds();

        commits.push(CommitInfo {
            oid: oid.to_string(),
            short_id,
            summary,
            author,
            timestamp,
        });
    }

    Ok(commits)
}

/// Convert a git2 Delta to our FileStatus enum.
fn delta_to_status(delta: Delta) -> FileStatus {
    match delta {
        Delta::Added | Delta::Untracked => FileStatus::Added,
        Delta::Deleted => FileStatus::Deleted,
        Delta::Modified => FileStatus::Modified,
        Delta::Renamed => FileStatus::Renamed,
        Delta::Copied => FileStatus::Copied,
        _ => FileStatus::Modified,
    }
}

/// Parse a git2 Diff into our structured Vec<DiffFile>.
pub fn parse_diff(diff: &Diff<'_>) -> Result<Vec<DiffFile>, GitError> {
    let mut files: Vec<DiffFile> = Vec::new();

    // We need to iterate through deltas and collect info per file.
    // First pass: collect file metadata from deltas.
    let num_deltas = diff.deltas().len();
    for i in 0..num_deltas {
        let delta = diff.get_delta(i).expect("delta should exist");
        let new_file = delta.new_file();
        let old_file = delta.old_file();

        let path = new_file
            .path()
            .or_else(|| old_file.path())
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default();

        let old_path = if delta.status() == Delta::Renamed {
            old_file.path().map(|p| p.to_string_lossy().to_string())
        } else {
            None
        };

        let status = delta_to_status(delta.status());

        files.push(DiffFile {
            path,
            old_path,
            status,
            hunks: Vec::new(),
            stats: DiffStats {
                additions: 0,
                deletions: 0,
            },
        });
    }

    // Second pass: iterate through patches to collect hunks and lines.
    for (file_idx, patch_result) in (0..num_deltas)
        .map(|i| git2::Patch::from_diff(diff, i))
        .enumerate()
    {
        let patch = match patch_result? {
            Some(p) => p,
            None => continue,
        };

        let file = &mut files[file_idx];
        let mut additions: usize = 0;
        let mut deletions: usize = 0;

        let num_hunks = patch.num_hunks();
        for hunk_idx in 0..num_hunks {
            let (hunk, _) = patch.hunk(hunk_idx)?;

            let header = String::from_utf8_lossy(hunk.header()).trim().to_string();
            let old_start = hunk.old_start();
            let old_lines = hunk.old_lines();
            let new_start = hunk.new_start();
            let new_lines = hunk.new_lines();

            let mut lines: Vec<DiffLine> = Vec::new();
            let num_lines = patch.num_lines_in_hunk(hunk_idx)?;

            for line_idx in 0..num_lines {
                let line = patch.line_in_hunk(hunk_idx, line_idx)?;
                let content = String::from_utf8_lossy(line.content()).to_string();
                // Remove trailing newline for cleaner display
                let content = content.trim_end_matches('\n').to_string();

                let (line_type, old_lineno, new_lineno) = match line.origin() {
                    '+' => {
                        additions += 1;
                        (LineType::Addition, None, Some(line.new_lineno().unwrap_or(0)))
                    }
                    '-' => {
                        deletions += 1;
                        (LineType::Deletion, Some(line.old_lineno().unwrap_or(0)), None)
                    }
                    _ => (
                        LineType::Context,
                        Some(line.old_lineno().unwrap_or(0)),
                        Some(line.new_lineno().unwrap_or(0)),
                    ),
                };

                lines.push(DiffLine {
                    line_type,
                    old_lineno,
                    new_lineno,
                    content,
                });
            }

            file.hunks.push(DiffHunk {
                header,
                old_start,
                old_lines,
                new_start,
                new_lines,
                lines,
            });
        }

        file.stats = DiffStats {
            additions,
            deletions,
        };
    }

    Ok(files)
}

/// Get a list of changed files with their statuses (lighter than full diff).
pub fn get_file_entries(diff: &Diff<'_>) -> Result<Vec<FileEntry>, GitError> {
    let mut entries: Vec<FileEntry> = Vec::new();

    let num_deltas = diff.deltas().len();
    for i in 0..num_deltas {
        let delta = diff.get_delta(i).expect("delta should exist");
        let new_file = delta.new_file();
        let old_file = delta.old_file();

        let path = new_file
            .path()
            .or_else(|| old_file.path())
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default();

        let status = delta_to_status(delta.status());

        // Count additions/deletions from patch
        let (additions, deletions) = match git2::Patch::from_diff(diff, i)? {
            Some(patch) => {
                let (_, adds, dels) = patch.line_stats()?;
                (adds, dels)
            }
            None => (0, 0),
        };

        entries.push(FileEntry {
            path,
            status,
            additions,
            deletions,
        });
    }

    Ok(entries)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn setup_test_repo() -> (TempDir, Repository) {
        let dir = TempDir::new().expect("create temp dir");
        let repo = Repository::init(dir.path()).expect("init repo");

        // Configure test user
        let mut config = repo.config().expect("get config");
        config.set_str("user.name", "Test User").expect("set name");
        config
            .set_str("user.email", "test@example.com")
            .expect("set email");

        // Create an initial commit so HEAD exists
        let file_path = dir.path().join("initial.txt");
        fs::write(&file_path, "initial content\n").expect("write file");

        let mut index = repo.index().expect("get index");
        index
            .add_path(Path::new("initial.txt"))
            .expect("add to index");
        index.write().expect("write index");

        let tree_id = index.write_tree().expect("write tree");
        {
            let tree = repo.find_tree(tree_id).expect("find tree");
            let sig = repo.signature().expect("get signature");
            repo.commit(Some("HEAD"), &sig, &sig, "Initial commit", &tree, &[])
                .expect("create commit");
        }

        (dir, repo)
    }

    #[test]
    fn test_open_repo() {
        let (dir, _repo) = setup_test_repo();
        let result = open_repo(dir.path().to_str().unwrap());
        assert!(result.is_ok());
    }

    #[test]
    fn test_open_non_repo() {
        let dir = TempDir::new().expect("create temp dir");
        let result = open_repo(dir.path().to_str().unwrap());
        assert!(result.is_err());
    }

    #[test]
    fn test_unstaged_diff() {
        let (dir, repo) = setup_test_repo();

        // Modify a tracked file
        let file_path = dir.path().join("initial.txt");
        fs::write(&file_path, "initial content\nmodified line\n").expect("modify file");

        let diff = get_diff(&repo, "unstaged").expect("get diff");
        let files = parse_diff(&diff).expect("parse diff");

        assert_eq!(files.len(), 1);
        assert_eq!(files[0].path, "initial.txt");
        assert_eq!(files[0].status, FileStatus::Modified);
        assert_eq!(files[0].stats.additions, 1);
        assert_eq!(files[0].stats.deletions, 0);
        assert!(!files[0].hunks.is_empty());
    }

    #[test]
    fn test_staged_diff() {
        let (dir, repo) = setup_test_repo();

        // Create and stage a new file
        let new_path = dir.path().join("new_file.txt");
        fs::write(&new_path, "new file content\n").expect("write new file");

        let mut index = repo.index().expect("get index");
        index
            .add_path(Path::new("new_file.txt"))
            .expect("add to index");
        index.write().expect("write index");

        let diff = get_diff(&repo, "staged").expect("get diff");
        let files = parse_diff(&diff).expect("parse diff");

        assert_eq!(files.len(), 1);
        assert_eq!(files[0].path, "new_file.txt");
        assert_eq!(files[0].status, FileStatus::Added);
        assert_eq!(files[0].stats.additions, 1);
    }

    #[test]
    fn test_deleted_file_diff() {
        let (dir, repo) = setup_test_repo();

        // Delete the tracked file
        let file_path = dir.path().join("initial.txt");
        fs::remove_file(&file_path).expect("delete file");

        let diff = get_diff(&repo, "unstaged").expect("get diff");
        let files = parse_diff(&diff).expect("parse diff");

        assert_eq!(files.len(), 1);
        assert_eq!(files[0].path, "initial.txt");
        assert_eq!(files[0].status, FileStatus::Deleted);
        assert_eq!(files[0].stats.deletions, 1);
    }

    #[test]
    fn test_untracked_file() {
        let (dir, repo) = setup_test_repo();

        // Create an untracked file
        let new_path = dir.path().join("untracked.txt");
        fs::write(&new_path, "untracked content\n").expect("write untracked");

        let diff = get_diff(&repo, "unstaged").expect("get diff");
        let files = parse_diff(&diff).expect("parse diff");

        assert_eq!(files.len(), 1);
        assert_eq!(files[0].path, "untracked.txt");
        assert_eq!(files[0].status, FileStatus::Added);
    }

    #[test]
    fn test_untracked_file_shows_full_content() {
        let (dir, repo) = setup_test_repo();

        // Create an untracked file with multiple lines
        let new_path = dir.path().join("new_file.rs");
        fs::write(
            &new_path,
            "fn main() {\n    println!(\"hello\");\n    let x = 42;\n}\n",
        )
        .expect("write new file");

        let diff = get_diff(&repo, "unstaged").expect("get diff");
        let files = parse_diff(&diff).expect("parse diff");

        assert_eq!(files.len(), 1);
        assert_eq!(files[0].status, FileStatus::Added);
        assert!(!files[0].hunks.is_empty(), "Untracked file should have hunks with content");
        assert_eq!(
            files[0].stats.additions, 4,
            "All 4 lines should be additions"
        );

        // Verify line content is present
        let all_lines: Vec<&DiffLine> = files[0]
            .hunks
            .iter()
            .flat_map(|h| h.lines.iter())
            .collect();
        assert!(
            all_lines.iter().any(|l| l.content.contains("fn main()")),
            "Should contain full file content"
        );
    }

    #[test]
    fn test_get_file_entries() {
        let (dir, repo) = setup_test_repo();

        // Modify a file and add a new one
        let file_path = dir.path().join("initial.txt");
        fs::write(&file_path, "modified\n").expect("modify file");

        let new_path = dir.path().join("new.txt");
        fs::write(&new_path, "new\n").expect("write new");

        let diff = get_diff(&repo, "unstaged").expect("get diff");
        let entries = get_file_entries(&diff).expect("get entries");

        assert_eq!(entries.len(), 2);
    }

    #[test]
    fn test_invalid_diff_type() {
        let (_dir, repo) = setup_test_repo();
        let result = get_diff(&repo, "invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_multi_hunk_diff() {
        let (dir, repo) = setup_test_repo();

        // Write a file with many lines, then modify lines far apart to get multiple hunks
        let file_path = dir.path().join("initial.txt");
        let mut content = String::new();
        for i in 1..=30 {
            content.push_str(&format!("line {i}\n"));
        }
        fs::write(&file_path, &content).expect("write many lines");

        // Stage and commit
        let mut index = repo.index().expect("get index");
        index
            .add_path(Path::new("initial.txt"))
            .expect("add to index");
        index.write().expect("write index");
        let tree_id = index.write_tree().expect("write tree");
        {
            let tree = repo.find_tree(tree_id).expect("find tree");
            let sig = repo.signature().expect("get signature");
            let head = repo.head().expect("get head");
            let parent = head.peel_to_commit().expect("peel to commit");
            repo.commit(
                Some("HEAD"),
                &sig,
                &sig,
                "Add many lines",
                &tree,
                &[&parent],
            )
            .expect("commit");
        }

        // Now modify lines 2 and 28 (far apart) to create two hunks
        let mut lines: Vec<String> = content.lines().map(String::from).collect();
        lines[1] = "MODIFIED line 2".to_string();
        lines[27] = "MODIFIED line 28".to_string();
        let modified = lines.join("\n") + "\n";
        fs::write(&file_path, &modified).expect("modify file");

        let diff = get_diff(&repo, "unstaged").expect("get diff");
        let files = parse_diff(&diff).expect("parse diff");

        assert_eq!(files.len(), 1);
        assert!(
            files[0].hunks.len() >= 2,
            "Expected at least 2 hunks, got {}",
            files[0].hunks.len()
        );
        assert_eq!(files[0].stats.additions, 2);
        assert_eq!(files[0].stats.deletions, 2);
    }
}
