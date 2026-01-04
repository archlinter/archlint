import { vi, describe, it } from 'vitest';
import { ruleTester } from '../setup';
import { noHubModules } from '../../rules/no-hub-modules';
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

describe('no-hub-modules', () => {
  it('should report hub modules', () => {
    (isAnalysisReady as any).mockReturnValue(AnalysisState.Ready);
    (getSmellsForFile as any).mockReturnValue([
      {
        smell: {
          smellType: 'HubModule',
          files: ['/project/hub.ts'],
          locations: [{ file: '/project/hub.ts', line: 1, column: 0 }],
        },
        explanation: {
          reason: 'Module is both highly coupled and highly depended upon',
        },
      },
    ]);

    ruleTester.run('no-hub-modules', noHubModules, {
      valid: [],
      invalid: [
        {
          code: 'import { x } from "./x";',
          filename: '/project/hub.ts',
          errors: [
            {
              messageId: 'smell',
              data: { reason: 'Module is both highly coupled and highly depended upon' },
              line: 1,
            },
          ],
        },
      ],
    });
  });
});
