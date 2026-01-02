import { scan } from '@archlinter/core';
import { GetSmellsInput } from '../schemas.js';
import { mcpCache } from '../cache.js';
import { formatResult, formatSmellsMd } from '../formatters.js';
import { SEVERITY_ORDER, Severity } from '../constants.js';

export async function archlintGetSmells(
  input: GetSmellsInput
): Promise<{ content: { type: 'text'; text: string }[] }> {
  const { path, types, severity, file, offset, limit, format } = input;

  let result = mcpCache.get(path);
  if (!result) {
    result = await scan(path);
    mcpCache.set(path, result);
  }

  let filteredSmells = result.smells;

  if (types && types.length > 0) {
    filteredSmells = filteredSmells.filter((s) => types.includes(s.smell.smellType));
  }

  if (severity) {
    const minSevValue = SEVERITY_ORDER[severity as Severity];
    filteredSmells = filteredSmells.filter(
      (s) => SEVERITY_ORDER[s.smell.severity as Severity] <= minSevValue
    );
  }

  if (file) {
    filteredSmells = filteredSmells.filter((s) => s.smell.files.some((f) => f.includes(file)));
  }

  const total = filteredSmells.length;
  const paginatedSmells = filteredSmells.slice(offset, offset + limit);

  const outputData = {
    smells: paginatedSmells,
    total,
    offset,
    limit,
    hasMore: offset + limit < total,
  };

  return {
    content: [
      {
        type: 'text',
        text: formatResult(outputData, format, (data) =>
          formatSmellsMd(data.smells, data.total, data.offset, data.limit)
        ),
      },
    ],
  };
}
