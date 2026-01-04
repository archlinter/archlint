import { vi, describe, it } from 'vitest';
import { ruleTester } from '../setup';
import { noDeadCode } from '../../rules/no-dead-code';
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

describe('no-dead-code', () => {
  it('should report dead code', () => {
    (isAnalysisReady as any).mockReturnValue(AnalysisState.Ready);
    (getSmellsForFile as any).mockReturnValue([
      {
        smell: {
          smellType: 'DeadCode',
          files: ['/project/unused.ts'],
          locations: [{ file: '/project/unused.ts', line: 5, column: 0 }],
        },
        explanation: {
          reason: 'Function "oldFn" is never called',
        },
      },
    ]);

    ruleTester.run('no-dead-code', noDeadCode, {
      valid: [],
      invalid: [
        {
          code: 'export function oldFn() {}',
          filename: '/project/unused.ts',
          errors: [
            {
              messageId: 'smell',
              data: { reason: 'Function "oldFn" is never called' },
              line: 5,
            },
          ],
        },
      ],
    });
  });
});
