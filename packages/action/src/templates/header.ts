import { JsDiffResult } from '@archlinter/core';

export function header(result: JsDiffResult): string {
  const status = result.hasRegressions ? '❌ Failing' : '✅ Passing';
  
  return `## archlint Architecture Report <!-- archlint-report -->

| Metric | Value |
|--------|-------|
| **Status** | ${status} |
| **New Regressions** | ${result.summary.newSmells + result.summary.worsenedSmells} |
| **Fixed/Improved** | ${result.summary.fixedSmells + result.summary.improvedSmells} |
| **Baseline** | \`${result.baselineCommit || 'unknown'}\` |
| **Current** | \`${result.currentCommit || 'unknown'}\` |

---
`;
}
