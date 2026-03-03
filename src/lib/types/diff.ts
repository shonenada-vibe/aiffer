export type FileStatus =
  | "added"
  | "modified"
  | "deleted"
  | "renamed"
  | "copied"
  | "untracked";

export type LineType = "context" | "addition" | "deletion";

export interface DiffLine {
  lineType: LineType;
  oldLineno: number | null;
  newLineno: number | null;
  content: string;
}

export interface DiffHunk {
  header: string;
  oldStart: number;
  oldLines: number;
  newStart: number;
  newLines: number;
  lines: DiffLine[];
}

export interface DiffStats {
  additions: number;
  deletions: number;
}

export interface DiffFile {
  path: string;
  oldPath: string | null;
  status: FileStatus;
  hunks: DiffHunk[];
  stats: DiffStats;
}

export interface FileEntry {
  path: string;
  status: FileStatus;
  additions: number;
  deletions: number;
}
