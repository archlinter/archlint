import { relative } from 'node:path';
import type { JsSmellWithExplanation } from '@archlinter/core';

export type SmellLocationStrategy =
  | 'all-files' // Report in all files mentioned
  | 'primary-file' // Report only in first/primary file
  | 'critical-edges' // Report at critical edges (for cycles)
  | 'source-file'; // Report in source file (for layer violations)

export const SMELL_STRATEGIES: Record<string, SmellLocationStrategy> = {
  cycles: 'critical-edges',
  god_module: 'primary-file',
  dead_code: 'primary-file',
  high_coupling: 'primary-file',
  barrel_file_abuse: 'primary-file',
  layer_violation: 'source-file',
  sdp_violation: 'primary-file',
  hub_module: 'primary-file',
  deep_nesting: 'all-files',
  long_params: 'all-files',
};

export interface FileSmellLocation {
  line: number;
  column?: number;
  messageId: string;
  data?: Record<string, unknown>;
}

export function getSmellLocationsForFile(
  smell: JsSmellWithExplanation,
  filePath: string,
  strategy: SmellLocationStrategy,
  projectRoot: string
): FileSmellLocation[] {
  const normalized = filePath.replace(/\\/g, '/');

  switch (strategy) {
    case 'critical-edges':
      return getCriticalEdgeLocations(smell, normalized, projectRoot);
    case 'primary-file':
      return getPrimaryFileLocation(smell, normalized);
    case 'source-file':
      return getSourceFileLocation(smell, normalized);
    case 'all-files':
    default:
      return getAllFileLocations(smell, normalized);
  }
}

function getCriticalEdgeLocations(
  smell: JsSmellWithExplanation,
  filePath: string,
  projectRoot: string
): FileSmellLocation[] {
  const cluster = smell.smell.cluster;
  if (!cluster?.criticalEdges) return [];

  return cluster.criticalEdges
    .filter((edge) => edge.from.replace(/\\/g, '/') === filePath)
    .map((edge) => ({
      line: edge.line,
      column: 0,
      messageId: 'cycle',
      data: {
        target: relative(projectRoot, edge.to),
        impact: edge.impact,
      },
    }));
}

function getPrimaryFileLocation(
  smell: JsSmellWithExplanation,
  filePath: string
): FileSmellLocation[] {
  const firstFile = smell.smell.files[0];
  if (!firstFile || firstFile.replace(/\\/g, '/') !== filePath) return [];

  return [
    {
      line: smell.smell.locations[0]?.line ?? 1,
      column: smell.smell.locations[0]?.column ?? 0,
      messageId: 'smell',
      data: { reason: smell.explanation.reason },
    },
  ];
}

function getSourceFileLocation(
  smell: JsSmellWithExplanation,
  filePath: string
): FileSmellLocation[] {
  // Layer violation reports in the file that makes the illegal import
  const sourceLoc = smell.smell.locations.find((l) => l.file.replace(/\\/g, '/') === filePath);
  if (!sourceLoc) return [];

  return [
    {
      line: sourceLoc.line,
      column: sourceLoc.column ?? 0,
      messageId: 'violation',
      data: { reason: smell.explanation.reason },
    },
  ];
}

function getAllFileLocations(smell: JsSmellWithExplanation, filePath: string): FileSmellLocation[] {
  return smell.smell.locations
    .filter((l) => l.file.replace(/\\/g, '/') === filePath)
    .map((l) => ({
      line: l.line,
      column: l.column ?? 0,
      messageId: 'smell',
      data: { reason: l.description || smell.explanation.reason },
    }));
}
