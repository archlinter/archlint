import { JsScanResult } from '@archlinter/core';

export interface CacheKeyOptions {
  detectors?: string[];
  excludeDetectors?: string[];
  minSeverity?: string;
}

class McpCache {
  private cache = new Map<string, JsScanResult>();
  private readonly MAX_SIZE = 100;
  private readonly DELIMITER = '\0';

  private generateKey(path: string, options?: CacheKeyOptions): string {
    const { detectors = [], excludeDetectors = [], minSeverity = '' } = options || {};
    const optsKey = `${detectors.sort().join(',')}|${excludeDetectors.sort().join(',')}|${minSeverity}`;
    return `${path}${this.DELIMITER}${optsKey}`;
  }

  get(path: string, options?: CacheKeyOptions): JsScanResult | undefined {
    const key = this.generateKey(path, options);
    const result = this.cache.get(key);
    if (result) {
      // Refresh order for LRU
      this.cache.delete(key);
      this.cache.set(key, result);
    }
    return result;
  }

  set(path: string, result: JsScanResult, options?: CacheKeyOptions): void {
    const key = this.generateKey(path, options);
    if (this.cache.has(key)) {
      this.cache.delete(key);
    } else if (this.cache.size >= this.MAX_SIZE) {
      // Evict oldest (first inserted)
      const oldestKey = this.cache.keys().next().value;
      if (oldestKey !== undefined) {
        this.cache.delete(oldestKey);
      }
    }
    this.cache.set(key, result);
  }

  delete(path: string): void {
    // Delete all keys starting with this path
    for (const key of this.cache.keys()) {
      if (key === path || key.startsWith(`${path}${this.DELIMITER}`)) {
        this.cache.delete(key);
      }
    }
  }

  getAllPaths(): string[] {
    const paths = new Set<string>();
    for (const key of this.cache.keys()) {
      paths.add(key.split(this.DELIMITER)[0]);
    }
    return Array.from(paths);
  }

  clear(): void {
    this.cache.clear();
  }
}

export const mcpCache = new McpCache();
