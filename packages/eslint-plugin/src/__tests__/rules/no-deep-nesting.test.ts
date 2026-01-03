import { vi, describe, it } from 'vitest';
import { ruleTester } from '../setup';
import { noDeepNesting } from '../../rules/no-deep-nesting';
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

describe('no-deep-nesting', () => {
  it('should report deep nesting', () => {
    (isAnalysisReady as any).mockReturnValue(AnalysisState.Ready);
    (getSmellsForFile as any).mockReturnValue([
      {
        smell: {
          smellType: 'deep_nesting',
          files: ['/project/nested.ts'],
          locations: [
            { file: '/project/nested.ts', line: 10, column: 4, description: 'Nested level 6' },
          ],
        },
        explanation: {
          reason: 'Deeply nested control structures',
        },
      },
    ]);

    ruleTester.run('no-deep-nesting', noDeepNesting, {
      valid: [],
      invalid: [
        {
          code: 'if (a) { if (b) { /* ... */ } }',
          filename: '/project/nested.ts',
          errors: [
            {
              messageId: 'smell',
              data: { reason: 'Nested level 6' },
              line: 10,
            },
          ],
        },
      ],
    });
  });
});
