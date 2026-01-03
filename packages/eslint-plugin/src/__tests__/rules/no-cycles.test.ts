import { vi, describe, it } from 'vitest';
import { ruleTester } from '../setup';
import { noCycles } from '../../rules/no-cycles';
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

describe('no-cycles', () => {
  it('should report cyclic dependencies', () => {
    (isAnalysisReady as any).mockReturnValue(AnalysisState.Ready);
    (getSmellsForFile as any).mockReturnValue([
      {
        smell: {
          smellType: 'cycles',
          files: ['/project/a.ts', '/project/b.ts'],
          locations: [
            { file: '/project/a.ts', line: 1, column: 0 },
            { file: '/project/b.ts', line: 1, column: 0 },
          ],
          cluster: {
            criticalEdges: [
              { from: '/project/a.ts', to: '/project/b.ts', line: 1, impact: 'high' },
            ],
          },
        },
        explanation: {
          reason: 'Cyclic dependency',
        },
      },
    ]);

    ruleTester.run('no-cycles', noCycles, {
      valid: [],
      invalid: [
        {
          code: 'import { b } from "./b";',
          filename: '/project/a.ts',
          errors: [
            {
              messageId: 'cycle',
              data: { target: 'b.ts', impact: 'high' },
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

    ruleTester.run('no-cycles', noCycles, {
      valid: [
        {
          code: 'import { b } from "./b";',
          filename: '/project/clean.ts',
        },
      ],
      invalid: [],
    });
  });

  it('should report analyzing state', () => {
    (isAnalysisReady as any).mockReturnValue(AnalysisState.NotStarted);

    ruleTester.run('no-cycles', noCycles, {
      valid: [],
      invalid: [
        {
          code: 'import { b } from "./b";',
          filename: '/project/a.ts',
          errors: [{ messageId: 'analyzing' }],
        },
      ],
    });
  });
});
