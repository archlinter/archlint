import { describe, it, expect, beforeEach, afterEach } from 'vitest';
import { writeFileSync, mkdirSync, rmSync } from 'fs';
import { join } from 'path';
import { tmpdir } from 'os';

// Import from cache module
import { isUnsavedFile, isVirtualFile, clearAllCaches } from '../../utils/cache';

describe('isUnsavedFile', () => {
  const testDir = join(tmpdir(), 'archlint-test-unsaved');
  const testFile = join(testDir, 'test.ts');

  beforeEach(() => {
    mkdirSync(testDir, { recursive: true });
    clearAllCaches();
  });

  afterEach(() => {
    rmSync(testDir, { recursive: true, force: true });
    clearAllCaches();
  });

  it('should return true for virtual file patterns', () => {
    expect(isUnsavedFile('<input>', 'const x = 1;')).toBe(true);
    expect(isUnsavedFile('<text>', 'const x = 1;')).toBe(true);
    expect(isUnsavedFile('untitled:Untitled-1', 'const x = 1;')).toBe(true);
    expect(isUnsavedFile('/dev/stdin', 'const x = 1;')).toBe(true);
  });

  it('should return false when bufferText is not provided', () => {
    writeFileSync(testFile, 'const x = 1;');
    expect(isUnsavedFile(testFile)).toBe(false);
  });

  it('should return false when buffer matches disk', () => {
    const content = 'const x = 1;';
    writeFileSync(testFile, content);
    expect(isUnsavedFile(testFile, content)).toBe(false);
  });

  it('should return true when buffer differs from disk', () => {
    writeFileSync(testFile, 'const x = 1;');
    expect(isUnsavedFile(testFile, 'const x = 2;')).toBe(true);
  });

  it('should return true for new file that does not exist on disk', () => {
    const newFile = join(testDir, 'new-file.ts');
    expect(isUnsavedFile(newFile, 'const x = 1;')).toBe(true);
  });

  it('should use cached hash for unchanged file (mtime)', () => {
    const content = 'const x = 1;';
    writeFileSync(testFile, content);

    // First check - reads file
    expect(isUnsavedFile(testFile, content)).toBe(false);

    // Second check - should use cache (same mtime)
    expect(isUnsavedFile(testFile, content)).toBe(false);

    // Modify buffer - should still work
    expect(isUnsavedFile(testFile, 'const x = 2;')).toBe(true);
  });

  it('should handle whitespace differences', () => {
    writeFileSync(testFile, 'const x = 1;');
    // Trailing newline is different
    expect(isUnsavedFile(testFile, 'const x = 1;\n')).toBe(true);
    // Same content should match
    expect(isUnsavedFile(testFile, 'const x = 1;')).toBe(false);
  });
});

describe('isVirtualFile', () => {
  it('should return true for virtual file patterns', () => {
    expect(isVirtualFile('<input>')).toBe(true);
    expect(isVirtualFile('<text>')).toBe(true);
    expect(isVirtualFile('untitled:Untitled-1')).toBe(true);
    expect(isVirtualFile('/dev/stdin')).toBe(true);
  });

  it('should return false for regular file paths', () => {
    expect(isVirtualFile('/abs/path/to/file.ts')).toBe(false);
    expect(isVirtualFile('./relative/path.ts')).toBe(false);
    expect(isVirtualFile('src/index.ts')).toBe(false);
  });
});
