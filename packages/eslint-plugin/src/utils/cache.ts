import {
  ArchlintAnalyzer,
  clearCache as coreClearCache,
  type JsScanResult,
  type JsSmellWithExplanation,
} from '@archlinter/core';
import { findProjectRoot } from './project-root';

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
  changedFiles: Set<string>;
  lastScanTime: number;
}

// Global analyzers and results map
const projectStates = new Map<string, ProjectState>();
let currentAnalysisPromise: Promise<void> | null = null;

/**
 * Check if analysis is ready for the project
 */
export function isAnalysisReady(filePath: string, projectRootOverride?: string): AnalysisState {
  const projectRoot = projectRootOverride ?? findProjectRoot(filePath);
  let state = projectStates.get(projectRoot);

  if (!state) {
    // Initialize analyzer and trigger background scan
    const analyzer = new ArchlintAnalyzer(projectRoot, { cache: true, git: false });
    state = {
      analyzer,
      result: null,
      state: AnalysisState.NotStarted,
      changedFiles: new Set(),
      lastScanTime: 0,
    };
    projectStates.set(projectRoot, state);
    triggerAnalysis(projectRoot);
    return AnalysisState.NotStarted;
  }

  // If we have changed files but no scan in progress, trigger incremental scan
  if (
    state.changedFiles.size > 0 &&
    state.state === AnalysisState.Ready &&
    !currentAnalysisPromise
  ) {
    triggerIncrementalAnalysis(projectRoot);
    return AnalysisState.InProgress;
  }

  return state.state;
}

/**
 * Notify that a file has changed (should be called by ESLint rule if possible or during check)
 */
export function notifyFileChanged(filePath: string, projectRootOverride?: string): void {
  const projectRoot = projectRootOverride ?? findProjectRoot(filePath);
  const state = projectStates.get(projectRoot);
  if (state) {
    state.changedFiles.add(filePath);
  }
}

/**
 * Trigger background initial analysis
 */
function triggerAnalysis(projectRoot: string): void {
  const state = projectStates.get(projectRoot);
  if (!state || currentAnalysisPromise) return;

  state.state = AnalysisState.InProgress;

  currentAnalysisPromise = (async () => {
    try {
      const result = await state.analyzer.scan();
      state.result = result;
      state.state = AnalysisState.Ready;
      state.lastScanTime = Date.now();
    } catch (error) {
      state.state = AnalysisState.Error;
      // eslint-disable-next-line no-console
      console.error('[archlint] Initial analysis failed:', error);
    } finally {
      currentAnalysisPromise = null;
    }
  })();
}

/**
 * Trigger background incremental analysis
 */
function triggerIncrementalAnalysis(projectRoot: string): void {
  const state = projectStates.get(projectRoot);
  if (!state || currentAnalysisPromise || state.changedFiles.size === 0) return;

  state.state = AnalysisState.InProgress;
  const changed = [...state.changedFiles];
  state.changedFiles.clear();

  currentAnalysisPromise = (async () => {
    try {
      const incResult = await state.analyzer.scanIncremental(changed);

      // Update the main result with new smells for affected files
      if (state.result) {
        const affectedFilesSet = new Set(incResult.affectedFiles);

        // 1. Remove old smells for affected files
        const otherSmells = state.result.smells.filter(
          (s) => !s.smell.files.some((f) => affectedFilesSet.has(f))
        );

        // 2. Add new smells
        state.result.smells = [...otherSmells, ...incResult.smells];
        state.lastScanTime = Date.now();
      }

      state.state = AnalysisState.Ready;
    } catch (error) {
      state.state = AnalysisState.Error;
      // eslint-disable-next-line no-console
      console.error('[archlint] Incremental analysis failed:', error);
    } finally {
      currentAnalysisPromise = null;
    }
  })();
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
 * Get smells for specific file and detector
 */
export function getSmellsForFile(
  filePath: string,
  detectorId: string,
  projectRootOverride?: string
): JsSmellWithExplanation[] {
  // Mark file as potentially changed when we check it
  // In watch mode, this will ensure it's re-analyzed if needed
  notifyFileChanged(filePath, projectRootOverride);

  const result = getAnalysis(filePath, projectRootOverride);
  if (!result) return [];

  const normalizedPath = filePath.replace(/\\/g, '/');

  return result.smells.filter((s) => {
    // Filter by detector (smellType contains detector id)
    if (!s.smell.smellType.toLowerCase().includes(detectorId.toLowerCase())) {
      return false;
    }
    // Filter by file
    return s.smell.files.some((f) => f.replace(/\\/g, '/') === normalizedPath);
  });
}

/**
 * Clear all caches
 */
export function clearAllCaches(): void {
  projectStates.clear();
  currentAnalysisPromise = null;
}

/**
 * Invalidate cache for specific project
 */
export function invalidateProject(projectRoot: string): void {
  projectStates.delete(projectRoot);
  coreClearCache(projectRoot);
}
