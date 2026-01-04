import { vi, describe, it } from 'vitest';
import { ruleTester } from '../setup';
import { noLayerViolations } from '../../rules/no-layer-violations';
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

describe('no-layer-violations', () => {
  it('should report layer violations', () => {
    (isAnalysisReady as any).mockReturnValue(AnalysisState.Ready);
    (getSmellsForFile as any).mockReturnValue([
      {
        smell: {
          smellType: 'LayerViolation { from_layer: "A", to_layer: "B" }',
          files: ['/project/src/domain/entity.ts', '/project/src/infra/db.ts'],
          locations: [{ file: '/project/src/domain/entity.ts', line: 2, column: 0 }],
        },
        explanation: {
          reason: 'Domain layer cannot import Infrastructure layer',
        },
      },
    ]);

    ruleTester.run('no-layer-violations', noLayerViolations, {
      valid: [],
      invalid: [
        {
          code: 'import { db } from "../infra/db";',
          filename: '/project/src/domain/entity.ts',
          errors: [
            {
              messageId: 'violation',
              data: { reason: 'Domain layer cannot import Infrastructure layer' },
              line: 2,
            },
          ],
        },
      ],
    });
  });
});
