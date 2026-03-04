//! End-to-end smoke tests for core user flow.
//!
//! These tests simulate the full user journey through the backend:
//! 1. Open a git repository
//! 2. Read diffs and file status
//! 3. Make changes and refresh to see updated diffs
//! 4. Build a review payload with comments and code context
//! 5. Validate settings persistence
//!
//! Run with: `cargo test --test smoke_test`

use std::fs;
use std::path::Path;

use git2::Repository;

use aiffer_lib::commands;
use aiffer_lib::models::{AppSettings, CommentInput, LineType};

/// Helper: create a temporary git repo with an initial commit.
fn setup_repo() -> (tempfile::TempDir, String) {
    let dir = tempfile::TempDir::new().expect("create temp dir");
    let repo = Repository::init(dir.path()).expect("init repo");

    let mut config = repo.config().expect("get config");
    config.set_str("user.name", "Test User").expect("set name");
    config
        .set_str("user.email", "test@example.com")
        .expect("set email");

    // Create initial files
    fs::write(dir.path().join("main.rs"), "fn main() {\n    println!(\"hello\");\n}\n")
        .expect("write main.rs");
    fs::write(dir.path().join("lib.rs"), "pub fn add(a: i32, b: i32) -> i32 {\n    a + b\n}\n")
        .expect("write lib.rs");
    fs::write(dir.path().join("README.md"), "# Test Project\n\nA test.\n")
        .expect("write README.md");

    // Stage and commit all files
    let mut index = repo.index().expect("get index");
    index.add_path(Path::new("main.rs")).expect("add main.rs");
    index.add_path(Path::new("lib.rs")).expect("add lib.rs");
    index
        .add_path(Path::new("README.md"))
        .expect("add README.md");
    index.write().expect("write index");

    let tree_id = index.write_tree().expect("write tree");
    let tree = repo.find_tree(tree_id).expect("find tree");
    let sig = repo.signature().expect("get signature");
    repo.commit(
        Some("HEAD"),
        &sig,
        &sig,
        "Initial commit with 3 files",
        &tree,
        &[],
    )
    .expect("create commit");

    let path = dir.path().to_str().unwrap().to_string();
    (dir, path)
}

// ============================================================
// Test 1: App launches and displays diff viewer (repo validation)
// ============================================================

#[test]
fn test_validate_git_repo_accepts_valid_repo() {
    let (_dir, path) = setup_repo();
    let result = commands::validate_git_repo(path);
    assert!(result.is_ok(), "Valid repo should be accepted");
}

#[test]
fn test_validate_git_repo_rejects_non_repo() {
    let dir = tempfile::TempDir::new().expect("create temp dir");
    let path = dir.path().to_str().unwrap().to_string();
    let result = commands::validate_git_repo(path);
    assert!(result.is_err(), "Non-repo should be rejected");
    let err = result.unwrap_err().to_string();
    assert!(
        err.contains("not a git repository") || err.contains("Not a git repository"),
        "Error should mention not a repo: {err}"
    );
}

#[test]
fn test_validate_git_repo_rejects_nonexistent_path() {
    let result = commands::validate_git_repo("/tmp/nonexistent_aiffer_test_path_12345".to_string());
    assert!(result.is_err());
}

// ============================================================
// Test 2: Changing a file and refreshing shows the diff correctly
// ============================================================

#[test]
fn test_modify_file_shows_diff() {
    let (dir, path) = setup_repo();

    // Modify main.rs
    fs::write(
        dir.path().join("main.rs"),
        "fn main() {\n    println!(\"hello world\");\n    println!(\"goodbye\");\n}\n",
    )
    .expect("modify main.rs");

    // Get unstaged diff
    let files = commands::get_diff(path.clone(), "unstaged".to_string(), None, None)
        .expect("get diff");

    assert_eq!(files.len(), 1, "Should have 1 changed file");
    assert_eq!(files[0].path, "main.rs");
    assert!(files[0].stats.additions > 0, "Should have additions");
    assert!(files[0].stats.deletions > 0, "Should have deletions");
    assert!(!files[0].hunks.is_empty(), "Should have hunks");

    // Verify line types in the hunk
    let hunk = &files[0].hunks[0];
    let has_addition = hunk.lines.iter().any(|l| l.line_type == LineType::Addition);
    let has_deletion = hunk.lines.iter().any(|l| l.line_type == LineType::Deletion);
    let has_context = hunk.lines.iter().any(|l| l.line_type == LineType::Context);

    assert!(has_addition, "Hunk should have additions");
    assert!(has_deletion, "Hunk should have deletions");
    assert!(has_context, "Hunk should have context lines");

    // Get file status
    let entries = commands::get_file_status(path).expect("get file status");
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].path, "main.rs");
}

#[test]
fn test_add_new_file_shows_in_diff() {
    let (dir, path) = setup_repo();

    // Add a new file and stage it (so it appears in staged diff with content)
    fs::write(dir.path().join("new_module.rs"), "pub fn greet() -> &'static str {\n    \"hello\"\n}\n")
        .expect("write new file");

    let repo = Repository::open(dir.path()).expect("open repo");
    let mut index = repo.index().expect("get index");
    index
        .add_path(Path::new("new_module.rs"))
        .expect("stage new_module.rs");
    index.write().expect("write index");

    let files = commands::get_diff(path.clone(), "staged".to_string(), None, None)
        .expect("get staged diff");

    assert_eq!(files.len(), 1);
    assert_eq!(files[0].path, "new_module.rs");
    assert_eq!(files[0].stats.additions, 3);
    assert_eq!(files[0].stats.deletions, 0);
}

#[test]
fn test_multiple_files_changed() {
    let (dir, path) = setup_repo();

    // Modify two files and add one
    fs::write(dir.path().join("main.rs"), "fn main() {}\n").expect("modify main.rs");
    fs::write(dir.path().join("lib.rs"), "pub fn add(a: i32, b: i32) -> i32 { a + b }\n")
        .expect("modify lib.rs");
    fs::write(dir.path().join("utils.rs"), "pub fn noop() {}\n").expect("add new file");

    let files = commands::get_diff(path.clone(), "unstaged".to_string(), None, None)
        .expect("get diff");

    assert_eq!(files.len(), 3, "Should have 3 changed files");

    let file_status = commands::get_file_status(path).expect("get status");
    assert_eq!(file_status.len(), 3);
}

// ============================================================
// Test 3: Staged vs unstaged diffs
// ============================================================

#[test]
fn test_staged_diff_separate_from_unstaged() {
    let (dir, path) = setup_repo();

    // Modify a file
    fs::write(
        dir.path().join("main.rs"),
        "fn main() {\n    println!(\"staged change\");\n}\n",
    )
    .expect("modify main.rs");

    // Stage it
    let repo = Repository::open(dir.path()).expect("open repo");
    let mut index = repo.index().expect("get index");
    index.add_path(Path::new("main.rs")).expect("stage main.rs");
    index.write().expect("write index");

    // Make another unstaged change
    fs::write(
        dir.path().join("lib.rs"),
        "pub fn add(a: i32, b: i32) -> i32 { a + b + 0 }\n",
    )
    .expect("modify lib.rs");

    // Staged diff should only show main.rs
    let staged = commands::get_diff(path.clone(), "staged".to_string(), None, None)
        .expect("get staged diff");
    assert_eq!(staged.len(), 1);
    assert_eq!(staged[0].path, "main.rs");

    // Unstaged diff should only show lib.rs
    let unstaged = commands::get_diff(path, "unstaged".to_string(), None, None)
        .expect("get unstaged diff");
    assert_eq!(unstaged.len(), 1);
    assert_eq!(unstaged[0].path, "lib.rs");
}

// ============================================================
// Test 4: Build review payload with comments and code context
// ============================================================

#[test]
fn test_build_review_payload_with_comments() {
    let (dir, path) = setup_repo();

    // Modify a file to create a diff
    fs::write(
        dir.path().join("main.rs"),
        "fn main() {\n    let name = \"world\";\n    println!(\"hello {name}\");\n}\n",
    )
    .expect("modify main.rs");

    // Build payload with a comment
    let comments = vec![CommentInput {
        file_path: "main.rs".to_string(),
        line_number: 2,
        line_type: LineType::Addition,
        body: "Consider using a constant for this string".to_string(),
    }];

    let payload = commands::build_review_payload(path, comments).expect("build payload");

    // Verify payload structure
    assert_eq!(payload.total_files_changed, 1);
    assert!(payload.total_additions > 0);
    assert!(!payload.comments.is_empty());
    assert!(!payload.file_summary.is_empty());
    assert!(!payload.formatted_text.is_empty());

    // Verify the comment has context
    let comment = &payload.comments[0];
    assert_eq!(comment.file_path, "main.rs");
    assert_eq!(comment.body, "Consider using a constant for this string");
    assert!(!comment.line_content.is_empty(), "Should have line content");

    // Verify formatted text contains key info
    assert!(payload.formatted_text.contains("main.rs"));
    assert!(payload.formatted_text.contains("Consider using a constant"));
}

#[test]
fn test_build_review_payload_multiple_comments() {
    let (dir, path) = setup_repo();

    // Create a more complex diff
    fs::write(
        dir.path().join("main.rs"),
        "fn main() {\n    let x = 1;\n    let y = 2;\n    let z = x + y;\n    println!(\"{z}\");\n}\n",
    )
    .expect("modify main.rs");

    let comments = vec![
        CommentInput {
            file_path: "main.rs".to_string(),
            line_number: 2,
            line_type: LineType::Addition,
            body: "Use descriptive variable names".to_string(),
        },
        CommentInput {
            file_path: "main.rs".to_string(),
            line_number: 4,
            line_type: LineType::Addition,
            body: "Extract this into a function".to_string(),
        },
    ];

    let payload = commands::build_review_payload(path, comments).expect("build payload");

    assert_eq!(payload.comments.len(), 2);
    assert!(payload.formatted_text.contains("Use descriptive variable names"));
    assert!(payload.formatted_text.contains("Extract this into a function"));
}

// ============================================================
// Test 5: Commit history and between-commits diff
// ============================================================

#[test]
fn test_get_commits() {
    let (_dir, path) = setup_repo();

    let commits = commands::get_commits(path, Some(10)).expect("get commits");
    assert!(!commits.is_empty(), "Should have at least one commit");

    let first = &commits[0];
    assert!(!first.oid.is_empty());
    assert!(!first.short_id.is_empty());
    assert!(!first.summary.is_empty());
    assert_eq!(first.author, "Test User");
}

#[test]
fn test_diff_between_commits() {
    let (dir, path) = setup_repo();

    // Get first commit
    let repo = Repository::open(dir.path()).expect("open repo");
    let first_commit = repo
        .head()
        .expect("get head")
        .peel_to_commit()
        .expect("peel to commit")
        .id()
        .to_string();

    // Make a second commit
    fs::write(
        dir.path().join("main.rs"),
        "fn main() {\n    println!(\"second commit\");\n}\n",
    )
    .expect("modify");
    let mut index = repo.index().expect("index");
    index.add_path(Path::new("main.rs")).expect("add");
    index.write().expect("write");
    let tree_id = index.write_tree().expect("write tree");
    let tree = repo.find_tree(tree_id).expect("find tree");
    let sig = repo.signature().expect("sig");
    let parent = repo
        .head()
        .unwrap()
        .peel_to_commit()
        .unwrap();
    let second_commit = repo
        .commit(Some("HEAD"), &sig, &sig, "Second commit", &tree, &[&parent])
        .expect("commit")
        .to_string();

    // Get diff between commits
    let files = commands::get_diff(
        path,
        "commits".to_string(),
        Some(first_commit),
        Some(second_commit),
    )
    .expect("get commit diff");

    assert_eq!(files.len(), 1);
    assert_eq!(files[0].path, "main.rs");
}

// ============================================================
// Test 6: Settings persistence
// ============================================================

#[test]
fn test_settings_roundtrip() {
    // Use a temp dir for settings
    let dir = tempfile::TempDir::new().expect("create temp dir");
    let settings_path = dir.path().join("settings.json");

    let settings = AppSettings {
        ai_endpoint: "https://api.example.com/v1".to_string(),
        ai_api_key: "sk-test-key-12345".to_string(),
        ai_model: "gpt-4o".to_string(),
        diff_type: "staged".to_string(),
        theme: "dark".to_string(),
        last_opened_folder: "/path/to/repo".to_string(),
    };

    // Write settings
    let content = serde_json::to_string_pretty(&settings).expect("serialize");
    fs::write(&settings_path, &content).expect("write settings");

    // Read them back
    let read_content = fs::read_to_string(&settings_path).expect("read settings");
    let loaded: AppSettings = serde_json::from_str(&read_content).expect("deserialize");

    assert_eq!(loaded.ai_endpoint, "https://api.example.com/v1");
    assert_eq!(loaded.ai_api_key, "sk-test-key-12345");
    assert_eq!(loaded.ai_model, "gpt-4o");
    assert_eq!(loaded.diff_type, "staged");
    assert_eq!(loaded.theme, "dark");
    assert_eq!(loaded.last_opened_folder, "/path/to/repo");
}

#[test]
fn test_settings_backward_compatibility() {
    // Old settings without last_opened_folder should still deserialize
    let json = r#"{
        "aiEndpoint": "https://api.openai.com/v1",
        "aiApiKey": "",
        "aiModel": "gpt-4o",
        "diffType": "unstaged",
        "theme": "light"
    }"#;

    let settings: AppSettings = serde_json::from_str(json).expect("deserialize old settings");
    assert_eq!(settings.last_opened_folder, "");
    assert_eq!(settings.theme, "light");
}

#[test]
fn test_settings_defaults() {
    let settings = AppSettings::default();
    assert_eq!(settings.ai_endpoint, "https://api.openai.com/v1");
    assert_eq!(settings.ai_api_key, "");
    assert_eq!(settings.ai_model, "gpt-4o");
    assert_eq!(settings.diff_type, "unstaged");
    assert_eq!(settings.theme, "light");
    assert_eq!(settings.last_opened_folder, "");
}

// ============================================================
// Test 7: Error handling for invalid operations
// ============================================================

#[test]
fn test_get_diff_on_non_repo_returns_error() {
    let dir = tempfile::TempDir::new().expect("create temp dir");
    let path = dir.path().to_str().unwrap().to_string();

    let result = commands::get_diff(path, "unstaged".to_string(), None, None);
    assert!(result.is_err());
}

#[test]
fn test_get_diff_invalid_type_returns_error() {
    let (_dir, path) = setup_repo();

    let result = commands::get_diff(path, "invalid_type".to_string(), None, None);
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("Invalid diff type"), "Error: {err}");
}

#[test]
fn test_get_commits_on_non_repo_returns_error() {
    let dir = tempfile::TempDir::new().expect("create temp dir");
    let path = dir.path().to_str().unwrap().to_string();

    let result = commands::get_commits(path, Some(10));
    assert!(result.is_err());
}

#[test]
fn test_build_review_payload_on_non_repo_returns_error() {
    let dir = tempfile::TempDir::new().expect("create temp dir");
    let path = dir.path().to_str().unwrap().to_string();

    let result = commands::build_review_payload(path, vec![]);
    assert!(result.is_err());
}
