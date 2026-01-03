import { vi, describe, it } from 'vitest';
import { ruleTester } from '../setup';
import { noHighComplexity } from '../../rules/no-high-complexity';
import { AnalysisState } from '../../utils/cache';

vi.mock('../../utils/cache', async (importOriginal) => {
  const actual = await importOriginal<any>();
  return {
    ...actual,
    isAnalysisReady: vi.fn(),
    getSmellsForFile: vi.fn(),
    notifyFileChanged: vi.fn(),
  };
});

vi.mock('../../utils/project-root', () => ({
  findProjectRoot: vi.fn(() => '/project'),
}));

const { isAnalysisReady, getSmellsForFile } = await import('../../utils/cache');

describe('no-high-complexity', () => {
  it('should report high complexity', () => {
    (isAnalysisReady as any).mockReturnValue(AnalysisState.Ready);
    (getSmellsForFile as any).mockReturnValue([
      {
        smell: {
          smellType: 'HighComplexity { name: "complexFunction", line: 10 }',
          files: ['/project/complex.ts'],
          locations: [{ file: '/project/complex.ts', line: 10, column: 0 }],
        },
        explanation: {
          reason: 'Function "complexFunction" has cyclomatic complexity of 25',
        },
      },
    ]);

    ruleTester.run('no-high-complexity', noHighComplexity, {
      valid: [],
      invalid: [
        {
          code: 'function complexFunction() { /* ... */ }',
          filename: '/project/complex.ts',
          errors: [
            {
              messageId: 'smell',
              data: { reason: 'Function "complexFunction" has cyclomatic complexity of 25' },
              line: 10,
            },
          ],
        },
      ],
    });
  });
});
