import { scan } from '@archlinter/core';
import { GetSmellsInput } from '../schemas.js';
import { mcpCache } from '../cache.js';
import { formatResult, formatSmellsMd } from '../formatters.js';
import { SEVERITY_ORDER, SMELL_SCORES, Severity } from '../constants.js';

export async function archlintGetSmells(
  input: GetSmellsInput
): Promise<{ content: { type: 'text'; text: string }[] }> {
  const { path, types, severity, file, minScore, offset, limit, format } = input;

  let result = mcpCache.get(path, {}); // Use empty options for "full" result
  if (!result) {
    result = await scan(path);
    mcpCache.set(path, result, {});
  }

  let filteredSmells = result.smells;

  if (types && types.length > 0) {
    filteredSmells = filteredSmells.filter((s) => types.includes(s.smell.smellType));
  }

  if (severity) {
    const minSevValue = SEVERITY_ORDER[severity as Severity];
    filteredSmells = filteredSmells.filter((s) => {
      const sev = s.smell.severity.toLowerCase() as Severity;
      return SEVERITY_ORDER[sev] <= minSevValue;
    });
  }

  if (file) {
    filteredSmells = filteredSmells.filter((s) => s.smell.files.some((f) => f.includes(file)));
  }

  if (minScore !== undefined) {
    filteredSmells = filteredSmells.filter((s) => {
      const score = SMELL_SCORES[s.smell.severity.toLowerCase() as Severity] || 0;
      return score >= minScore;
    });
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
