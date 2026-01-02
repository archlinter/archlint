export const SERVER_NAME = 'archlint-mcp';
export const SERVER_VERSION = '0.4.1';

export const SEVERITY_ORDER = {
  critical: 0,
  high: 1,
  medium: 2,
  low: 3,
  info: 4,
} as const;

export type Severity = keyof typeof SEVERITY_ORDER;
