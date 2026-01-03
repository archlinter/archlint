import { vi, describe, it } from 'vitest';
import { ruleTester } from '../setup';
import { noBarrelAbuse } from '../../rules/no-barrel-abuse';
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

describe('no-barrel-abuse', () => {
  it('should report barrel file abuse', () => {
    (isAnalysisReady as any).mockReturnValue(AnalysisState.Ready);
    (getSmellsForFile as any).mockReturnValue([
      {
        smell: {
          smellType: 'barrel_file_abuse',
          files: ['/project/index.ts'],
          locations: [{ file: '/project/index.ts', line: 1, column: 0 }],
        },
        explanation: {
          reason: 'Barrel file exports too many symbols (50+)',
        },
      },
    ]);

    ruleTester.run('no-barrel-abuse', noBarrelAbuse, {
      valid: [],
      invalid: [
        {
          code: 'export * from "./a"; // ... many more',
          filename: '/project/index.ts',
          errors: [
            {
              messageId: 'smell',
              data: { reason: 'Barrel file exports too many symbols (50+)' },
              line: 1,
            },
          ],
        },
      ],
    });
  });
});
