import { JsScanResult } from '@archlinter/core';

class McpCache {
  private cache = new Map<string, JsScanResult>();

  get(path: string): JsScanResult | undefined {
    return this.cache.get(path);
  }

  set(path: string, result: JsScanResult): void {
    this.cache.set(path, result);
  }

  delete(path: string): void {
    this.cache.delete(path);
  }

  getAllPaths(): string[] {
    return Array.from(this.cache.keys());
  }

  clear(): void {
    this.cache.clear();
  }
}

export const mcpCache = new McpCache();
