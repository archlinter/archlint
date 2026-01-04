import { relative } from 'node:path';
import type { JsSmellWithExplanation } from '@archlinter/core';

export type SmellLocationStrategy =
  | 'all-files' // Report in all files mentioned
  | 'primary-file' // Report only in first/primary file
  | 'critical-edges' // Report at critical edges (for cycles)
  | 'source-file'; // Report in source file (for layer violations)


export interface FileSmellLocation {
  line: number;
  column?: number;
  endLine?: number;
  endColumn?: number;
  messageId: string;
  data?: Record<string, unknown>;
}

export function getSmellLocationsForFile(
  smell: JsSmellWithExplanation,
  filePath: string,
  strategy: SmellLocationStrategy,
  projectRoot: string
): FileSmellLocation[] {
  const normalized = filePath.replaceAll('\\', '/');

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
    .filter((edge) => edge.from.replaceAll('\\', '/') === filePath)
    .map((edge) => ({
      line: edge.line,
      column: edge.range?.startColumn ?? 0,
      endLine: edge.range?.endLine,
      endColumn: edge.range?.endColumn,
      messageId: 'cycle',
      data: {
        target: relative(projectRoot, edge.to),
        impact: edge.impact,
      },
    }));
}

function createLocationFromSmell(
  loc: { line?: number; column?: number; range?: { endLine?: number; endColumn?: number } } | undefined,
  reason: string
): FileSmellLocation {
  return {
    line: loc?.line ?? 1,
    column: loc?.column ?? 0,
    endLine: loc?.range?.endLine,
    endColumn: loc?.range?.endColumn,
    messageId: 'smell',
    data: { reason },
  };
}

function getPrimaryFileLocation(
  smell: JsSmellWithExplanation,
  filePath: string
): FileSmellLocation[] {
  const firstFile = smell.smell.files[0];
  if (!firstFile) return [];

  const normalizedFirstFile = firstFile.replaceAll('\\', '/');
  const normalizedPath = filePath.replaceAll('\\', '/');
  if (normalizedFirstFile !== normalizedPath) return [];

  const loc = smell.smell.locations[0];
  return [createLocationFromSmell(loc, smell.explanation.reason)];
}

function getSourceFileLocation(
  smell: JsSmellWithExplanation,
  filePath: string
): FileSmellLocation[] {
  // Layer violation reports in the file that makes the illegal import
  const sourceLoc = smell.smell.locations.find((l) => l.file.replaceAll('\\', '/') === filePath);
  if (!sourceLoc) return [];

  return [
    {
      line: sourceLoc.line,
      column: sourceLoc.column ?? 0,
      endLine: sourceLoc.range?.endLine,
      endColumn: sourceLoc.range?.endColumn,
      messageId: 'violation',
      data: { reason: smell.explanation.reason },
    },
  ];
}

function getAllFileLocations(smell: JsSmellWithExplanation, filePath: string): FileSmellLocation[] {
  return smell.smell.locations
    .filter((l) => l.file.replaceAll('\\', '/') === filePath)
    .map((l) => ({
      line: l.line,
      column: l.column ?? 0,
      endLine: l.range?.endLine,
      endColumn: l.range?.endColumn,
      messageId: 'smell',
      data: { reason: l.description || smell.explanation.reason },
    }));
}
