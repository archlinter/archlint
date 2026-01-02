import { JsScanResult } from '@archlinter/core';

export interface CacheKeyOptions {
  detectors?: string[];
  excludeDetectors?: string[];
  minSeverity?: string;
}

class McpCache {
  private cache = new Map<string, JsScanResult>();

  private generateKey(path: string, options?: CacheKeyOptions): string {
    if (!options) return path;
    const { detectors = [], excludeDetectors = [], minSeverity = '' } = options;
    const optsKey = `${detectors.sort().join(',')}|${excludeDetectors.sort().join(',')}|${minSeverity}`;
    return `${path}:${optsKey}`;
  }

  get(path: string, options?: CacheKeyOptions): JsScanResult | undefined {
    return this.cache.get(this.generateKey(path, options));
  }

  set(path: string, result: JsScanResult, options?: CacheKeyOptions): void {
    this.cache.set(this.generateKey(path, options), result);
  }

  delete(path: string): void {
    // Delete all keys starting with this path
    for (const key of this.cache.keys()) {
      if (key === path || key.startsWith(`${path}:`)) {
        this.cache.delete(key);
      }
    }
  }

  getAllPaths(): string[] {
    const paths = new Set<string>();
    for (const key of this.cache.keys()) {
      paths.add(key.split(':')[0]);
    }
    return Array.from(paths);
  }

  clear(): void {
    this.cache.clear();
  }
}

export const mcpCache = new McpCache();
