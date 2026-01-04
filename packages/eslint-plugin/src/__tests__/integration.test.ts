import { describe, it, expect } from 'vitest';
import { ruleTester } from './setup';
import { noCycles } from '../rules/no-cycles';
import { isAnalysisReady, AnalysisState, clearAllCaches } from '../utils/cache';
import path from 'node:path';
import { writeFileSync } from 'node:fs';

describe('ESLint Plugin Integration', () => {
  const fixtureDir = path.join(__dirname, 'fixtures/cycle');

  it('should detect real cyclic dependency in fixtures', () => {
    clearAllCaches();

    const fileA = path.join(fixtureDir, 'a.ts');

    // Synchronous analysis - should be Ready immediately
    const state = isAnalysisReady(fileA, { projectRoot: fixtureDir });
    expect(state).toBe(AnalysisState.Ready);

    // Run the rule - it should find the cycle
    ruleTester.run('no-cycles-integration', noCycles, {
      valid: [],
      invalid: [
        {
          code: "import { b } from './b';\nexport const a = b + 1;",
          filename: fileA,
          options: [{ projectRoot: fixtureDir }],
          errors: [
            {
              messageId: 'cycle',
              data: { target: 'b.ts', impact: 'critical' },
            },
          ],
        },
      ],
    });
  });

  it('should handle broken cycles after file change', async () => {
    const fileA = path.join(fixtureDir, 'a.ts');
    const fileB = path.join(fixtureDir, 'b.ts');

    // Ensure we are ready
    expect(isAnalysisReady(fileA, { projectRoot: fixtureDir })).toBe(AnalysisState.Ready);

    // Break the cycle in file b
    const originalContentB = "// @ts-nocheck\nimport { a } from './a';\nexport const b = a + 1;";
    const brokenContentB = '// @ts-nocheck\nexport const b = 1;';

    writeFileSync(fileB, brokenContentB);

    try {
      // Trigger re-scan
      const { notifyFileChanged } = await import('../utils/cache');
      notifyFileChanged(fileB, fixtureDir);

      // Now fileA should be clean (no cycle)
      ruleTester.run('no-cycles-incremental', noCycles, {
        valid: [
          {
            code: "import { b } from './b';\nexport const a = b + 1;",
            filename: fileA,
            options: [{ projectRoot: fixtureDir }],
          },
        ],
        invalid: [],
      });
    } finally {
      // Restore file B
      writeFileSync(fileB, originalContentB);
    }
  });
});
