import type { Rule } from 'eslint';
import type { JsSmellWithExplanation } from '@archlinter/core';
import { getSmellsForFile, isAnalysisReady, AnalysisState, isVirtualFile } from '../utils/cache';
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

function createRuleMeta(options: RuleOptions): Rule.RuleModule['meta'] {
  return {
    type: 'problem',
    docs: {
      description: options.description,
      category: options.category,
      recommended: options.recommended,
      url: `https://archlinter.dev/rules/${options.detectorId.replaceAll('_', '-')}`,
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
  };
}

interface ReportLocation {
  start: { line: number; column: number };
  end?: { line: number; column: number };
}

function reportSmellLocations(
  context: Readonly<Rule.RuleContext>,
  smell: JsSmellWithExplanation,
  filename: string,
  strategy: SmellLocationStrategy,
  projectRoot: string
): void {
  const locations = getSmellLocationsForFile(smell, filename, strategy, projectRoot);

  for (const loc of locations) {
    const reportLoc: ReportLocation = {
      start: { line: loc.line, column: Math.max(0, (loc.column ?? 1) - 1) },
    };

    if (loc.endLine !== undefined && loc.endColumn !== undefined) {
      reportLoc.end = { line: loc.endLine, column: Math.max(0, loc.endColumn - 1) };
    }

    context.report({
      loc: reportLoc as any,
      messageId: loc.messageId,
      data: loc.data,
    });
  }
}

interface RuleOptionsConfig {
  projectRoot?: string;
}

export function createArchlintRule(options: RuleOptions): Rule.RuleModule {
  return {
    meta: createRuleMeta(options),
    create(context: Readonly<Rule.RuleContext>) {
      const filename = context.filename;
      const ruleOptions = (context.options[0] ?? {}) as RuleOptionsConfig;
      const projectRoot = ruleOptions.projectRoot ?? findProjectRoot(filename);
      const sourceCode = context.sourceCode;
      const bufferText = sourceCode.text;

      return {
        Program(node) {
          if (isVirtualFile(filename)) {
            return;
          }

          const state = isAnalysisReady(filename, {
            projectRoot: ruleOptions.projectRoot,
            bufferText,
          });

          if (state === AnalysisState.NotStarted) {
            context.report({ node, messageId: 'analyzing' });
            return;
          }

          if (state === AnalysisState.InProgress) {
            return;
          }

          const smells = getSmellsForFile(
            filename,
            options.detectorId,
            ruleOptions.projectRoot,
            bufferText
          );

          for (const smell of smells) {
            reportSmellLocations(context, smell, filename, options.strategy, projectRoot);
          }
        },
      };
    },
  };
}
