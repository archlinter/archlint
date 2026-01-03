import { existsSync } from 'node:fs';
import { dirname, join } from 'node:path';

// Markers in priority order
const PROJECT_ROOT_MARKERS = [
  '.archlint.yaml', // Explicit archlint config
  '.archlint.yml',
  'pnpm-workspace.yaml', // pnpm monorepo
  'lerna.json', // Lerna monorepo
  'nx.json', // Nx monorepo
  'rush.json', // Rush monorepo
  '.git', // Git root (fallback)
];

// Cache for performance
const rootCache = new Map<string, string>();

/**
 * Find project root for monorepo analysis
 * Returns the topmost project root (monorepo root)
 */
export function findProjectRoot(filePath: string): string {
  // Check cache first
  const cached = rootCache.get(filePath);
  if (cached) return cached;

  let dir = dirname(filePath);
  let foundRoot: string | null = null;

  while (dir !== '/' && dir !== '.') {
    for (const marker of PROJECT_ROOT_MARKERS) {
      if (existsSync(join(dir, marker))) {
        foundRoot = dir;
        // Don't break - keep looking for higher root (monorepo)
        break;
      }
    }
    dir = dirname(dir);
  }

  const result = foundRoot ?? process.cwd();
  rootCache.set(filePath, result);
  return result;
}

/**
 * Clear project root cache (for testing)
 */
export function clearProjectRootCache(): void {
  rootCache.clear();
}
