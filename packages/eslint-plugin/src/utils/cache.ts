import { ArchlintAnalyzer, type JsScanResult, type JsSmellWithExplanation } from '@archlinter/core';
import { findProjectRoot } from './project-root';
import { statSync, readFileSync } from 'fs';
import * as crypto from 'crypto';
import xxhashInit from 'xxhash-wasm';

const { createHash } = crypto;

let xxhash: { h64: (str: string) => bigint } | null = null;

// Start loading immediately (non-blocking)
xxhashInit()
  .then((h) => {
    xxhash = h;
  })
  .catch(() => {
    /* fallback to MD5 */
  });

// Debug mode - use DEBUG=archlint:* or DEBUG=archlint:cache (standard ESLint pattern)
// Also supports legacy ARCHLINT_DEBUG=1
const DEBUG_PATTERN = process.env.DEBUG || '';
const DEBUG =
  process.env.ARCHLINT_DEBUG === '1' ||
  DEBUG_PATTERN.includes('archlint:') ||
  DEBUG_PATTERN === '*';

// Force rescan on every check - set ARCHLINT_FORCE_RESCAN=1 to enable
const FORCE_RESCAN = process.env.ARCHLINT_FORCE_RESCAN === '1';

// Disable buffer detection (always treat files as saved) - useful for CI/CLI
const NO_BUFFER_CHECK = process.env.ARCHLINT_NO_BUFFER_CHECK === '1';

function debug(namespace: string, ...args: unknown[]): void {
  if (DEBUG) {
    const ts = new Date().toISOString().split('T')[1].slice(0, 12);
    // eslint-disable-next-line no-console
    console.error(`  ${ts} archlint:${namespace}`, ...args);
  }
}

/**
 * Compute fast hash of content (for comparison, not security)
 */
function computeHash(content: string): string {
  if (xxhash) {
    return xxhash.h64(content).toString(16);
  }

  // Fallback for first 1-2 calls before WASM loads or if it fails
  if (typeof crypto.hash === 'function') {
    return crypto.hash('md5', content, 'hex');
  }
  return createHash('md5').update(content).digest('hex');
}

// Cache disk content hashes to avoid re-reading files
interface DiskFileInfo {
  hash: string;
  mtime: number;
  size: number;
}
const diskFileCache = new Map<string, DiskFileInfo>();

/**
 * Get disk file hash (cached by mtime)
 */
function getDiskFileHash(filePath: string): string | null {
  try {
    const stats = statSync(filePath);
    const mtime = stats.mtimeMs;
    const size = stats.size;
    const cached = diskFileCache.get(filePath);

    if (cached && cached.mtime === mtime && cached.size === size) {
      return cached.hash;
    }

    const content = readFileSync(filePath, 'utf8');
    const hash = computeHash(content);
    diskFileCache.set(filePath, { hash, mtime, size });
    return hash;
  } catch {
    return null; // file doesn't exist
  }
}

/**
 * Check if file is unsaved (buffer differs from disk)
 * Virtual files (untitled, stdin) are always considered unsaved
 */
export function isUnsavedFile(filename: string, bufferText?: string): boolean {
  if (NO_BUFFER_CHECK) {
    return false; // Disabled - treat all files as saved
  }

  // Virtual file patterns used by IDEs
  if (isVirtualFile(filename)) {
    debug('cache', 'Virtual file detected:', filename);
    return true;
  }

  // No buffer text provided - assume saved
  if (!bufferText) {
    return false;
  }

  const diskHash = getDiskFileHash(filename);
  if (diskHash === null) {
    // File doesn't exist on disk (new file)
    debug('cache', 'New file (not on disk):', filename);
    return true;
  }

  // FAST PATH: Check size first.
  // diskFileCache was just populated by getDiskFileHash
  const cached = diskFileCache.get(filename);
  if (cached && Buffer.byteLength(bufferText, 'utf8') !== cached.size) {
    debug('cache', 'Size mismatch detected:', filename);
    return true;
  }

  const bufferHash = computeHash(bufferText);
  const isUnsaved = bufferHash !== diskHash;

  if (isUnsaved) {
    debug('cache', 'Unsaved file detected (hash mismatch):', filename);
  }

  return isUnsaved;
}

/**
 * Check if file is virtual (no real path on disk)
 */
export function isVirtualFile(filename: string): boolean {
  return (
    filename === '<input>' ||
    filename === '<text>' ||
    filename.startsWith('untitled:') ||
    filename.includes('stdin')
  );
}

export enum AnalysisState {
  NotStarted = 'not_started',
  InProgress = 'in_progress',
  Ready = 'ready',
  Error = 'error',
}

interface ProjectState {
  analyzer: ArchlintAnalyzer;
  result: JsScanResult | null;
  state: AnalysisState;
  lastScanTime: number;
  fileMtimes: Map<string, number>; // Track file modification times
}

// Global analyzers and results map
const projectStates = new Map<string, ProjectState>();

/**
 * Get file mtime in ms
 */
function getFileMtime(filePath: string): number {
  try {
    return statSync(filePath).mtimeMs;
  } catch {
    return 0;
  }
}

export interface AnalysisOptions {
  projectRoot?: string;
  bufferText?: string;
}

/**
 * Initialize analyzer and perform initial scan
 */
function initializeProjectState(
  projectRoot: string,
  filePath: string,
  currentMtime: number
): ProjectState {
  debug('cache', 'First scan for project:', projectRoot);
  const analyzer = new ArchlintAnalyzer(projectRoot, { cache: true, git: false });
  const state: ProjectState = {
    analyzer,
    result: null,
    state: AnalysisState.InProgress,
    lastScanTime: 0,
    fileMtimes: new Map(),
  };
  projectStates.set(projectRoot, state);

  try {
    const start = Date.now();
    state.result = analyzer.scanSync();
    state.state = AnalysisState.Ready;
    state.lastScanTime = Date.now();
    state.fileMtimes.set(filePath, currentMtime);
    debug('cache', 'Initial scan complete', {
      duration: Date.now() - start,
      smellCount: state.result?.smells?.length,
    });
  } catch (error) {
    state.state = AnalysisState.Error;
    // eslint-disable-next-line no-console
    console.error('[archlint] Analysis failed:', error);
  }

  return state;
}

/**
 * Perform incremental rescan for changed file
 */
function performRescan(state: ProjectState, filePath: string): void {
  debug('cache', 'Triggering rescan', { filePath });
  try {
    state.analyzer.invalidate([filePath]);
    const start = Date.now();
    state.result = state.analyzer.rescanSync();
    state.lastScanTime = Date.now();
    debug('cache', 'Rescan complete', {
      duration: Date.now() - start,
      smellCount: state.result?.smells?.length,
    });
  } catch (error) {
    debug('cache', 'Rescan error', error);
  }
}

/**
 * Check if file needs rescan based on modification time
 */
function shouldRescanFile(state: ProjectState, filePath: string, currentMtime: number): boolean {
  const lastMtime = state.fileMtimes.get(filePath);
  const fileChanged = lastMtime !== undefined && currentMtime > lastMtime;
  const isNewFile = lastMtime === undefined;
  return fileChanged || (FORCE_RESCAN && isNewFile);
}

/**
 * Handle file change tracking and rescan logic
 */
function handleFileChange(state: ProjectState, filePath: string, currentMtime: number): void {
  const lastMtime = state.fileMtimes.get(filePath);
  const fileChanged = lastMtime !== undefined && currentMtime > lastMtime;
  state.fileMtimes.set(filePath, currentMtime);

  if (shouldRescanFile(state, filePath, currentMtime) && state.state === AnalysisState.Ready) {
    performRescan(state, filePath);
  } else {
    debug('cache', 'Using cached', {
      file: filePath.split('/').pop(),
      lastMtime: lastMtime ?? 'new',
      fileChanged,
    });
  }
}

/**
 * Check if analysis is ready for the project.
 * If not ready, performs synchronous analysis (blocks until complete).
 * If file changed since last scan (and is saved), triggers incremental rescan.
 *
 * For unsaved files (buffer differs from disk), returns cached results without rescan.
 * This prevents stale project graph from IDE buffers.
 */
export function isAnalysisReady(filePath: string, options?: AnalysisOptions): AnalysisState {
  const { projectRoot: projectRootOverride, bufferText } = options ?? {};
  const projectRoot = projectRootOverride ?? findProjectRoot(filePath);
  let state = projectStates.get(projectRoot);
  const currentMtime = getFileMtime(filePath);
  const unsaved = isUnsavedFile(filePath, bufferText);

  debug('cache', 'isAnalysisReady', {
    filePath,
    projectRoot,
    currentMtime,
    hasState: !!state,
    unsaved,
  });

  if (!state) {
    state = initializeProjectState(projectRoot, filePath, currentMtime);
    return state.state;
  }

  if (unsaved) {
    debug('cache', 'Unsaved file - using cached results', { file: filePath.split('/').pop() });
    return state.state;
  }

  handleFileChange(state, filePath, currentMtime);
  return state.state;
}

/**
 * Notify that a file has changed - triggers re-scan on next check
 * @public API function for external use
 */
export function notifyFileChanged(filePath: string, projectRootOverride?: string): void {
  const projectRoot = projectRootOverride ?? findProjectRoot(filePath);
  const state = projectStates.get(projectRoot);
  if (state && state.state === AnalysisState.Ready) {
    // Mark for re-scan by resetting state
    // Next isAnalysisReady call will trigger incremental scan
    try {
      state.result = state.analyzer.rescanSync();
      state.lastScanTime = Date.now();
    } catch {
      // Ignore rescan errors, keep old results
    }
  }
}

/**
 * Get analysis result
 */
export function getAnalysis(filePath: string, projectRootOverride?: string): JsScanResult | null {
  const projectRoot = projectRootOverride ?? findProjectRoot(filePath);
  const state = projectStates.get(projectRoot);

  if (!state || state.state !== AnalysisState.Ready) {
    return null;
  }

  return state.result;
}

/**
 * Analyze file with overlay content (for unsaved IDE buffers)
 * Does NOT affect the cache - results are temporary
 */
export function analyzeWithOverlay(
  filePath: string,
  content: string,
  projectRootOverride?: string
): JsSmellWithExplanation[] {
  const projectRoot = projectRootOverride ?? findProjectRoot(filePath);
  const state = projectStates.get(projectRoot);

  if (!state || state.state !== AnalysisState.Ready) {
    debug('overlay', 'Analysis not ready, skipping overlay analysis');
    return [];
  }

  try {
    const start = Date.now();
    // Call new NAPI method
    const result = state.analyzer.scanIncrementalWithOverlaySync([filePath], {
      [filePath]: content,
    });

    debug('overlay', 'Overlay analysis complete', {
      file: filePath.split('/').pop(),
      duration: Date.now() - start,
      smellCount: result.smells.length,
    });

    return result.smells;
  } catch (error) {
    debug('overlay', 'Overlay analysis failed', error);
    return [];
  }
}

/**
 * Convert snake_case detectorId to match PascalCase smellType
 * e.g. "dead_code" matches "DeadCode" or "DeadSymbol"
 */
function matchesSmellType(smellType: string, detectorId: string): boolean {
  // Convert snake_case to words: "dead_code" -> ["dead", "code"]
  const words = detectorId.toLowerCase().split('_');
  // smellType is like "DeadCode" or "DeadSymbol { ... }" - extract base type
  const baseType = smellType.split(/[\s{]/)[0].toLowerCase();

  // Check if all words appear in the smell type
  // "dead" + "code" should match "deadcode" or "deadsymbol" (for dead_code -> DeadSymbol)
  // Special cases for detector -> smellType mapping
  const mappings: Record<string, string[]> = {
    dead_code: ['deadcode', 'deadsymbol'],
    cycles: ['cyclicdependency', 'cyclicdependencycluster'],
    high_coupling: ['highcoupling'],
    high_complexity: ['highcomplexity'],
    long_params: ['longparameterlist'],
    deep_nesting: ['deepnesting'],
    god_module: ['godmodule'],
    barrel_file_abuse: ['barrelfileabuse'],
    layer_violation: ['layerviolation'],
    sdp_violation: ['sdpviolation'],
    hub_module: ['hubmodule'],
  };

  const patterns = mappings[detectorId] || [words.join('')];
  return patterns.some((p) => baseType.includes(p));
}

/**
 * Get smells for specific file and detector
 * If bufferText provided and differs from disk, uses overlay analysis
 */
export function getSmellsForFile(
  filePath: string,
  detectorId: string,
  projectRootOverride?: string,
  bufferText?: string
): JsSmellWithExplanation[] {
  const projectRoot = projectRootOverride ?? findProjectRoot(filePath);

  // Check if we need overlay analysis
  if (bufferText && isUnsavedFile(filePath, bufferText)) {
    debug('smell', 'Using overlay analysis for unsaved file', filePath.split('/').pop());
    const allSmells = analyzeWithOverlay(filePath, bufferText, projectRoot);
    return filterSmellsByDetector(allSmells, filePath, detectorId);
  }

  // Normal analysis (from cache)
  const result = getAnalysis(filePath, projectRoot);
  if (!result) {
    debug('smell', 'no result for', filePath);
    return [];
  }

  return filterSmellsByDetector(result.smells, filePath, detectorId);
}

/**
 * Filter smells by detector and file path
 */
function filterSmellsByDetector(
  smells: JsSmellWithExplanation[],
  filePath: string,
  detectorId: string
): JsSmellWithExplanation[] {
  const normalizedPath = filePath.replace(/\\/g, '/');

  const filtered = smells.filter((s) => {
    // Filter by detector
    if (!matchesSmellType(s.smell.smellType, detectorId)) {
      return false;
    }
    // Filter by file
    return s.smell.files.some((f) => f.replace(/\\/g, '/') === normalizedPath);
  });

  if (filtered.length > 0) {
    debug('smell', `${detectorId}: ${filtered.length} smells for`, filePath.split('/').pop());
  }
  return filtered;
}

/**
 * Clear all caches
 * Used in tests
 */
export function clearAllCaches(): void {
  projectStates.clear();
  diskFileCache.clear();
}
