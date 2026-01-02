import { scan } from '@archlinter/core';
import { GetStatsInput } from '../schemas.js';
import { mcpCache } from '../cache.js';
import { formatResult, formatStatsMd } from '../formatters.js';

export async function archlintGetStats(
  input: GetStatsInput
): Promise<{ content: { type: 'text'; text: string }[] }> {
  const { path, format } = input;

  let result = mcpCache.get(path, {}); // Use empty options for "full" result
  if (!result) {
    result = await scan(path);
    mcpCache.set(path, result, {});
  }

  return {
    content: [
      {
        type: 'text',
        text: formatResult(result, format, formatStatsMd),
      },
    ],
  };
}
