import type { FileStatus, LineType } from "./diff";

export interface Comment {
  id: string;
  filePath: string;
  lineNumber: number;
  lineType: LineType;
  body: string;
  createdAt: string;
}

/** Input sent to the Rust build_review_payload command. */
export interface CommentInput {
  filePath: string;
  lineNumber: number;
  lineType: LineType;
  body: string;
}

/** A comment enriched with surrounding code context (from Rust). */
export interface CommentWithContext {
  filePath: string;
  lineNumber: number;
  lineType: LineType;
  body: string;
  lineContent: string;
  contextBefore: string[];
  contextAfter: string[];
}

export interface FileSummary {
  path: string;
  status: FileStatus;
  additions: number;
  deletions: number;
}

/** The full review payload returned by the Rust backend. */
export interface ReviewPayload {
  comments: CommentWithContext[];
  fileSummary: FileSummary[];
  totalFilesChanged: number;
  totalAdditions: number;
  totalDeletions: number;
  formattedText: string;
}
