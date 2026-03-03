import type { LineType } from "../types/diff";
import type { Comment } from "../types/comment";

let _comments = $state<Map<string, Comment[]>>(new Map());

let _allComments = $derived(
  Array.from(_comments.values())
    .flat()
    .sort((a, b) => a.createdAt.localeCompare(b.createdAt)),
);

let _commentCount = $derived(_allComments.length);

function makeKey(filePath: string, lineNumber: number): string {
  return `${filePath}:${lineNumber}`;
}

function generateId(): string {
  return crypto.randomUUID();
}

export const commentStore = {
  get allComments() {
    return _allComments;
  },

  get commentCount() {
    return _commentCount;
  },

  getCommentsForLine(filePath: string, lineNumber: number): Comment[] {
    const key = makeKey(filePath, lineNumber);
    return _comments.get(key) ?? [];
  },

  getCommentCountForLine(filePath: string, lineNumber: number): number {
    const key = makeKey(filePath, lineNumber);
    return _comments.get(key)?.length ?? 0;
  },

  getCommentedLines(): Map<string, Comment[]> {
    return _comments;
  },

  addComment(
    filePath: string,
    lineNumber: number,
    lineType: LineType,
    body: string,
  ): Comment {
    const comment: Comment = {
      id: generateId(),
      filePath,
      lineNumber,
      lineType,
      body,
      createdAt: new Date().toISOString(),
    };

    const key = makeKey(filePath, lineNumber);
    const existing = _comments.get(key) ?? [];
    const updated = new Map(_comments);
    updated.set(key, [...existing, comment]);
    _comments = updated;

    return comment;
  },

  removeComment(commentId: string): boolean {
    const updated = new Map(_comments);
    for (const [key, comments] of updated) {
      const filtered = comments.filter((c) => c.id !== commentId);
      if (filtered.length !== comments.length) {
        if (filtered.length === 0) {
          updated.delete(key);
        } else {
          updated.set(key, filtered);
        }
        _comments = updated;
        return true;
      }
    }
    return false;
  },

  editComment(commentId: string, newBody: string): boolean {
    const updated = new Map(_comments);
    for (const [key, comments] of updated) {
      const idx = comments.findIndex((c) => c.id === commentId);
      if (idx !== -1) {
        const newComments = [...comments];
        newComments[idx] = { ...newComments[idx], body: newBody };
        updated.set(key, newComments);
        _comments = updated;
        return true;
      }
    }
    return false;
  },

  clearAll() {
    _comments = new Map();
  },
};
