import { vi, describe, it } from 'vitest';
import { ruleTester } from '../setup';
import { noSdpViolations } from '../../rules/no-sdp-violations';
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

describe('no-sdp-violations', () => {
  it('should report SDP violations', () => {
    (isAnalysisReady as any).mockReturnValue(AnalysisState.Ready);
    (getSmellsForFile as any).mockReturnValue([
      {
        smell: {
          smellType: 'SdpViolation',
          files: ['/project/stable.ts'],
          locations: [{ file: '/project/stable.ts', line: 1, column: 0 }],
        },
        explanation: {
          reason: 'Stable module depends on unstable module "utils.ts"',
        },
      },
    ]);

    ruleTester.run('no-sdp-violations', noSdpViolations, {
      valid: [],
      invalid: [
        {
          code: 'import { util } from "./utils";',
          filename: '/project/stable.ts',
          errors: [
            {
              messageId: 'smell',
              data: { reason: 'Stable module depends on unstable module "utils.ts"' },
              line: 1,
            },
          ],
        },
      ],
    });
  });
});
