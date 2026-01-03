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
  lastScanTime: number;
}

// Global analyzers and results map
const projectStates = new Map<string, ProjectState>();

/**
 * Check if analysis is ready for the project.
 * If not ready, performs synchronous analysis (blocks until complete).
 */
export function isAnalysisReady(filePath: string, projectRootOverride?: string): AnalysisState {
  const projectRoot = projectRootOverride ?? findProjectRoot(filePath);
  let state = projectStates.get(projectRoot);

  if (!state) {
    // Initialize analyzer and run synchronous scan
    const analyzer = new ArchlintAnalyzer(projectRoot, { cache: true, git: false });
    state = {
      analyzer,
      result: null,
      state: AnalysisState.InProgress,
      lastScanTime: 0,
    };
    projectStates.set(projectRoot, state);

    try {
      // Synchronous scan - blocks until complete
      state.result = analyzer.scanSync();
      state.state = AnalysisState.Ready;
      state.lastScanTime = Date.now();
    } catch (error) {
      state.state = AnalysisState.Error;
      // eslint-disable-next-line no-console
      console.error('[archlint] Analysis failed:', error);
    }
  }

  return state.state;
}

/**
 * Notify that a file has changed - triggers re-scan on next check
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
 */
export function getSmellsForFile(
  filePath: string,
  detectorId: string,
  projectRootOverride?: string
): JsSmellWithExplanation[] {
  const result = getAnalysis(filePath, projectRootOverride);
  if (!result) return [];

  const normalizedPath = filePath.replace(/\\/g, '/');

  return result.smells.filter((s) => {
    // Filter by detector
    if (!matchesSmellType(s.smell.smellType, detectorId)) {
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
}

/**
 * Invalidate cache for specific project
 */
export function invalidateProject(projectRoot: string): void {
  projectStates.delete(projectRoot);
  coreClearCache(projectRoot);
}
