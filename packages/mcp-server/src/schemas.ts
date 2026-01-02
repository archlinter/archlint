import { z } from 'zod';

export const FormatSchema = z.enum(['json', 'markdown']).default('json');

export const ScanInputSchema = z.object({
  path: z.string().describe('Path to the project to scan'),
  detectors: z.array(z.string()).optional().describe('Only run these detectors (by ID)'),
  excludeDetectors: z.array(z.string()).optional().describe('Exclude these detectors (by ID)'),
  minSeverity: z
    .enum(['info', 'low', 'medium', 'high', 'critical'])
    .optional()
    .describe('Minimum severity to report'),
  force: z.boolean().optional().describe('Force re-scan and ignore MCP cache'),
  format: FormatSchema,
});

export const GetSmellsInputSchema = z.object({
  path: z.string().describe('Path to the project'),
  types: z.array(z.string()).optional().describe('Filter smells by type'),
  severity: z
    .enum(['info', 'low', 'medium', 'high', 'critical'])
    .optional()
    .describe('Minimum severity to report'),
  file: z.string().optional().describe('Filter smells by file path'),
  minScore: z.number().optional().describe('Filter smells by minimum score'),
  offset: z.number().int().min(0).default(0).describe('Pagination: offset'),
  limit: z.number().int().min(1).max(500).default(50).describe('Pagination: limit'),
  format: FormatSchema,
});

export const GetStatsInputSchema = z.object({
  path: z.string().describe('Path to the project'),
  format: FormatSchema,
});

export const ListDetectorsInputSchema = z.object({
  format: FormatSchema,
});

export const ClearCacheInputSchema = z.object({
  path: z.string().optional().describe('Path to the project (optional, if omitted clears all)'),
  level: z
    .enum(['mcp', 'full'])
    .default('mcp')
    .describe('Cache level to clear: mcp (memory) or full (mcp + rust disk cache)'),
});

export type ScanInput = z.infer<typeof ScanInputSchema>;
export type GetSmellsInput = z.infer<typeof GetSmellsInputSchema>;
export type GetStatsInput = z.infer<typeof GetStatsInputSchema>;
export type ListDetectorsInput = z.infer<typeof ListDetectorsInputSchema>;
export type ClearCacheInput = z.infer<typeof ClearCacheInputSchema>;
