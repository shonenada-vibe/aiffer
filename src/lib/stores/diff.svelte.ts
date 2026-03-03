import { invoke } from "@tauri-apps/api/core";
import type { DiffFile, FileEntry } from "../types/diff";

// Reactive state using module-level $state runes
let _folderPath = $state("");
let _files = $state<FileEntry[]>([]);
let _diffFiles = $state<DiffFile[]>([]);
let _selectedFile = $state<string | null>(null);
let _loading = $state(false);
let _error = $state<string | null>(null);
let _diffType = $state<"unstaged" | "staged">("unstaged");

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

  setDiffType(type: "unstaged" | "staged") {
    _diffType = type;
  },

  async refresh() {
    if (!_folderPath) return;

    _loading = true;
    _error = null;

    try {
      const [files, diffFiles] = await Promise.all([
        invoke<FileEntry[]>("get_file_status", { path: _folderPath }),
        invoke<DiffFile[]>("get_diff", {
          path: _folderPath,
          diffType: _diffType,
        }),
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
    await this.refresh();
  },
};
