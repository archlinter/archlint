import { vi, describe, it } from 'vitest';
import { ruleTester } from '../setup';
import { noCodeClone } from '../../rules/no-code-clone';
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

describe('no-code-clone', () => {
    it('should report code clone', () => {
    (isAnalysisReady as any).mockReturnValue(AnalysisState.Ready);
    (getSmellsForFile as any).mockImplementation((filename: string) => {
      if (filename === '/project/unique.ts') return [];
      return [
        {
          smell: {
            smellType: 'CodeClone { clone_hash: "hash123", token_count: 50 }',
            files: ['/project/file1.ts', '/project/file2.ts'],
            locations: [
              {
                file: '/project/file1.ts',
                line: 5,
                column: 0,
                description: 'Clone instance (lines 5-15)',
              },
            ],
          },
          explanation: {
            reason: 'Duplicated code found in multiple locations',
          },
        },
      ];
    });

    ruleTester.run('no-code-clone', noCodeClone, {
      valid: [
        {
          code: 'function unique() { return 1; }',
          filename: '/project/unique.ts',
        },
      ],
      invalid: [
        {
          code: 'function duplicated() { /* ... */ }',
          filename: '/project/file1.ts',
          errors: [
            {
              messageId: 'smell',
              data: { reason: 'Clone instance (lines 5-15)' },
              line: 5,
            },
          ],
        },
      ],
    });
  });
});
