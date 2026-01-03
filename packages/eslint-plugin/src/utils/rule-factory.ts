import type { Rule } from 'eslint';
import { getSmellsForFile, isAnalysisReady, AnalysisState } from '../utils/cache';
import { getSmellLocationsForFile, type SmellLocationStrategy } from '../utils/smell-filter';
import { findProjectRoot } from '../utils/project-root';

export interface RuleOptions {
  detectorId: string;
  messageId: string;
  description: string;
  category: string;
  recommended: boolean;
  strategy: SmellLocationStrategy;
  messages: Record<string, string>;
}

export function createArchlintRule(options: RuleOptions): Rule.RuleModule {
  return {
    meta: {
      type: 'problem',
      docs: {
        description: options.description,
        category: options.category,
        recommended: options.recommended,
        url: `https://archlinter.dev/rules/${options.detectorId.replace(/_/g, '-')}`,
      },
      schema: [
        {
          type: 'object',
          properties: {
            projectRoot: {
              type: 'string',
              description: 'Override project root detection',
            },
          },
          additionalProperties: false,
        },
      ],
      messages: {
        analyzing: 'archlint: analyzing project architecture...',
        ...options.messages,
      },
    },

    create(context) {
      const filename = context.filename ?? context.getFilename();
      const ruleOptions = context.options[0] ?? {};
      const projectRoot = ruleOptions.projectRoot ?? findProjectRoot(filename);

      return {
        Program(node) {
          const state = isAnalysisReady(filename, ruleOptions.projectRoot);

          if (state === AnalysisState.NotStarted) {
            context.report({
              node,
              messageId: 'analyzing',
            });
            return;
          }

          if (state === AnalysisState.InProgress) {
            return;
          }

          const smells = getSmellsForFile(filename, options.detectorId, ruleOptions.projectRoot);

          for (const smell of smells) {
            const locations = getSmellLocationsForFile(
              smell,
              filename,
              options.strategy,
              projectRoot
            );

            for (const loc of locations) {
              context.report({
                loc: { line: loc.line, column: loc.column ?? 0 },
                messageId: loc.messageId,
                data: loc.data,
              });
            }
          }
        },
      };
    },
  };
}
