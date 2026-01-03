import { vi, describe, it } from 'vitest';
import { ruleTester } from '../setup';
import { noLongParams } from '../../rules/no-long-params';
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

describe('no-long-params', () => {
  it('should report long parameter lists', () => {
    (isAnalysisReady as any).mockReturnValue(AnalysisState.Ready);
    (getSmellsForFile as any).mockReturnValue([
      {
        smell: {
          smellType: 'long_params',
          files: ['/project/params.ts'],
          locations: [
            {
              file: '/project/params.ts',
              line: 1,
              column: 10,
              description: 'Function has 8 parameters',
            },
          ],
        },
        explanation: {
          reason: 'Functions should have fewer than 5 parameters',
        },
      },
    ]);

    ruleTester.run('no-long-params', noLongParams, {
      valid: [],
      invalid: [
        {
          code: 'function f(a, b, c, d, e, f, g, h) {}',
          filename: '/project/params.ts',
          errors: [
            {
              messageId: 'smell',
              data: { reason: 'Function has 8 parameters' },
              line: 1,
            },
          ],
        },
      ],
    });
  });
});
