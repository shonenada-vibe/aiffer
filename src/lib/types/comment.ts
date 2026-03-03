import type { LineType } from "./diff";

export interface Comment {
  id: string;
  filePath: string;
  lineNumber: number;
  lineType: LineType;
  body: string;
  createdAt: string;
}

export interface CommentWithContext {
  comment: Comment;
  lineContent: string;
  contextBefore: string[];
  contextAfter: string[];
}
