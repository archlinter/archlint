import { vi, describe, it } from 'vitest';
import { ruleTester } from '../setup';
import { noDeadSymbols } from '../../rules/no-dead-symbols';
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

describe('no-dead-symbols', () => {
  it('should report dead symbols', () => {
    (isAnalysisReady as any).mockReturnValue(AnalysisState.Ready);
    (getSmellsForFile as any).mockReturnValue([
      {
        smell: {
          smellType: { DeadSymbol: { name: 'unusedMethod', kind: 'Class Method' } },
          files: ['/project/service.ts'],
          locations: [
            {
              file: '/project/service.ts',
              line: 10,
              column: 5,
              description: "Class Method 'unusedMethod' definition",
            },
          ],
        },
        explanation: {
          reason: "Method 'unusedMethod' is never called",
        },
      },
    ]);

    ruleTester.run('no-dead-symbols', noDeadSymbols, {
      valid: [],
      invalid: [
        {
          code: 'class S { unusedMethod() {} }',
          filename: '/project/service.ts',
          errors: [
            {
              messageId: 'smell',
              data: { reason: "Class Method 'unusedMethod' definition" },
              line: 10,
            },
          ],
        },
      ],
    });
  });
});
