import { invoke } from "@tauri-apps/api/core";
import type { CommitInfo, DiffFile, DiffType, FileEntry } from "../types/diff";

// Reactive state using module-level $state runes
let _folderPath = $state("");
let _files = $state<FileEntry[]>([]);
let _diffFiles = $state<DiffFile[]>([]);
let _selectedFile = $state<string | null>(null);
let _loading = $state(false);
let _error = $state<string | null>(null);
let _diffType = $state<DiffType>("unstaged");
let _commits = $state<CommitInfo[]>([]);
let _fromRef = $state<string | null>(null);
let _toRef = $state<string | null>(null);

// Derived totals
let _totalAdditions = $derived(
  _files.reduce((sum, f) => sum + f.additions, 0),
);
let _totalDeletions = $derived(
  _files.reduce((sum, f) => sum + f.deletions, 0),
);

export const diffStore = {
  get folderPath() {
    return _folderPath;
  },
  get files() {
    return _files;
  },
  get diffFiles() {
    return _diffFiles;
  },
  get selectedFile() {
    return _selectedFile;
  },
  get loading() {
    return _loading;
  },
  get error() {
    return _error;
  },
  get diffType() {
    return _diffType;
  },
  get commits() {
    return _commits;
  },
  get fromRef() {
    return _fromRef;
  },
  get toRef() {
    return _toRef;
  },
  get totalAdditions() {
    return _totalAdditions;
  },
  get totalDeletions() {
    return _totalDeletions;
  },

  setFolderPath(path: string) {
    _folderPath = path;
  },

  setSelectedFile(path: string | null) {
    _selectedFile = path;
  },

  setError(error: string | null) {
    _error = error;
  },

  setDiffType(type_: DiffType) {
    _diffType = type_;
  },

  setFromRef(ref_: string | null) {
    _fromRef = ref_;
  },

  setToRef(ref_: string | null) {
    _toRef = ref_;
  },

  async loadCommits() {
    if (!_folderPath) return;
    try {
      const commits = await invoke<CommitInfo[]>("get_commits", {
        path: _folderPath,
      });
      _commits = commits;
    } catch {
      _commits = [];
    }
  },

  async refresh() {
    if (!_folderPath) return;

    _loading = true;
    _error = null;

    try {
      const diffArgs: Record<string, unknown> = {
        path: _folderPath,
        diffType: _diffType,
      };
      if (_diffType === "commits") {
        diffArgs.fromRef = _fromRef;
        diffArgs.toRef = _toRef;
      }

      const [files, diffFiles] = await Promise.all([
        invoke<FileEntry[]>("get_file_status", { path: _folderPath }),
        invoke<DiffFile[]>("get_diff", diffArgs),
      ]);

      _files = files;
      _diffFiles = diffFiles;

      // Select first file if nothing selected
      if (!_selectedFile && files.length > 0) {
        _selectedFile = files[0].path;
      }
    } catch (e) {
      _error = typeof e === "string" ? e : "Failed to read diffs";
      _files = [];
      _diffFiles = [];
    } finally {
      _loading = false;
    }
  },

  async openFolder(path: string) {
    _folderPath = path;
    _selectedFile = null;
    _error = null;
    await this.loadCommits();
    await this.refresh();
  },
};
