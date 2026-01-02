import { scan } from '@archlinter/core';
import { ClearCacheInput } from '../schemas.js';
import { mcpCache } from '../cache.js';

export async function archlintClearCache(
  input: ClearCacheInput
): Promise<{ content: { type: 'text'; text: string }[] }> {
  const { path, level } = input;

  if (path) {
    mcpCache.delete(path);
    if (level === 'full') {
      await scan(path, { cache: false });
    }
  } else {
    mcpCache.clear();
  }

  return {
    content: [
      {
        type: 'text',
        text: JSON.stringify({ cleared: true, path: path || 'all', level }, null, 2),
      },
    ],
  };
}
