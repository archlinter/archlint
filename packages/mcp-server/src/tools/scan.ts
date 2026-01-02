import { scan } from '@archlinter/core';
import { ScanInput } from '../schemas.js';
import { mcpCache } from '../cache.js';
import { formatResult, formatScanResultMd } from '../formatters.js';

export async function archlintScan(
  input: ScanInput
): Promise<{ content: { type: 'text'; text: string }[] }> {
  const { path, detectors, excludeDetectors, minSeverity, force, format } = input;

  if (force) {
    mcpCache.delete(path);
  }

  // Check MCP cache first if not forced
  const cached = mcpCache.get(path, { detectors, excludeDetectors, minSeverity });
  if (cached && !force) {
    return {
      content: [
        {
          type: 'text',
          text: formatResult(cached, format, formatScanResultMd),
        },
      ],
    };
  }

  const result = await scan(path, {
    detectors,
    excludeDetectors,
    minSeverity,
    cache: !force, // Use Rust cache unless forced
  });

  mcpCache.set(path, result, { detectors, excludeDetectors, minSeverity });

  return {
    content: [
      {
        type: 'text',
        text: formatResult(result, format, formatScanResultMd),
      },
    ],
  };
}
