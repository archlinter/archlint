import { vi, describe, it } from 'vitest';
import { ruleTester } from '../setup';
import { noGodModules } from '../../rules/no-god-modules';
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

describe('no-god-modules', () => {
  it('should report god modules', () => {
    (isAnalysisReady as any).mockReturnValue(AnalysisState.Ready);
    (getSmellsForFile as any).mockReturnValue([
      {
        smell: {
          smellType: 'god_module',
          files: ['/project/god.ts'],
          locations: [{ file: '/project/god.ts', line: 1, column: 0 }],
        },
        explanation: {
          reason: 'Too many lines (1000) and methods (50)',
        },
      },
    ]);

    ruleTester.run('no-god-modules', noGodModules, {
      valid: [],
      invalid: [
        {
          code: 'class God { /* ... */ }',
          filename: '/project/god.ts',
          errors: [
            {
              messageId: 'smell',
              data: { reason: 'Too many lines (1000) and methods (50)' },
              line: 1,
            },
          ],
        },
      ],
    });
  });

  it('should not report when no smells', () => {
    (isAnalysisReady as any).mockReturnValue(AnalysisState.Ready);
    (getSmellsForFile as any).mockReturnValue([]);

    ruleTester.run('no-god-modules', noGodModules, {
      valid: [
        {
          code: 'class Small {}',
          filename: '/project/small.ts',
        },
      ],
      invalid: [],
    });
  });
});
