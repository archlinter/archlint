import { vi, describe, it } from 'vitest';
import { ruleTester } from '../setup';
import { noHighCoupling } from '../../rules/no-high-coupling';
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

describe('no-high-coupling', () => {
  it('should report high coupling', () => {
    (isAnalysisReady as any).mockReturnValue(AnalysisState.Ready);
    (getSmellsForFile as any).mockReturnValue([
      {
        smell: {
          smellType: 'HighCoupling { cbo: 15 }',
          files: ['/project/coupled.ts'],
          locations: [{ file: '/project/coupled.ts', line: 1, column: 0 }],
        },
        explanation: {
          reason: 'Module imports 25 different modules',
        },
      },
    ]);

    ruleTester.run('no-high-coupling', noHighCoupling, {
      valid: [],
      invalid: [
        {
          code: 'import { a } from "./a"; // ... many more',
          filename: '/project/coupled.ts',
          errors: [
            {
              messageId: 'smell',
              data: { reason: 'Module imports 25 different modules' },
              line: 1,
            },
          ],
        },
      ],
    });
  });
});
