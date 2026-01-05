import type { Rule } from 'eslint';
import { runDiff, type JsDiffResult } from '@archlinter/core';
import * as fs from 'fs';
import * as path from 'path';

// Cache to run diff only once per project
const diffCache = new Map<string, JsDiffResult>();

interface RuleOptions {
  baseline?: string;
  failOn?: 'low' | 'medium' | 'high' | 'critical';
}

export const noRegression: Rule.RuleModule = {
  meta: {
    type: 'problem',
    docs: {
      description: 'Detect architectural regressions compared to baseline',
      recommended: false,
    },
    messages: {
      regression: '{{message}}',
      noBaseline: 'Baseline snapshot not found at {{path}}. Run: archlint snapshot -o {{path}}',
    },
    schema: [
      {
        type: 'object',
        properties: {
          baseline: {
            type: 'string',
            description: 'Path to baseline snapshot',
          },
          failOn: {
            type: 'string',
            enum: ['low', 'medium', 'high', 'critical'],
            description: 'Minimum severity to report',
          },
        },
        additionalProperties: false,
      },
    ],
  },

  create(context) {
    const options: RuleOptions = context.options[0] ?? {};
    const baselinePath = options.baseline ?? '.archlint-snapshot.json';
    const failOn = options.failOn ?? 'low';

    const projectRoot = findProjectRoot(context.filename);

    // Only run once per project
    if (!diffCache.has(projectRoot)) {
      const absoluteBaseline = path.resolve(projectRoot, baselinePath);
      if (!fs.existsSync(absoluteBaseline)) {
        // Only report on the first file to avoid spamming
        if (isFirstFile(context, projectRoot)) {
          context.report({
            loc: { line: 1, column: 0 },
            messageId: 'noBaseline',
            data: { path: baselinePath },
          });
        }
        return {};
      }

      try {
        const result = runDiff({
          baselinePath: absoluteBaseline,
          projectPath: projectRoot,
        });
        diffCache.set(projectRoot, result);
      } catch (error) {
        if (isFirstFile(context, projectRoot)) {
          context.report({
            loc: { line: 1, column: 0 },
            messageId: 'regression',
            data: { message: `Diff failed: ${String(error)}` },
          });
        }
        return {};
      }
    }

    return reportFromCache(context, projectRoot, failOn);
  },
};

function getRegressionsToReport(result: JsDiffResult, failOn: string): JsDiffResult['regressions'] {
  const sevOrder = { low: 0, medium: 1, high: 2, critical: 3 };
  const minSev = sevOrder[failOn as keyof typeof sevOrder] ?? 0;

  return result.regressions.filter((r) => {
    const sev = r.smell.severity.toLowerCase();
    return (sevOrder[sev as keyof typeof sevOrder] ?? 0) >= minSev;
  });
}

function reportFromCache(
  context: Rule.RuleContext,
  projectRoot: string,
  failOn: string
): Rule.RuleListener {
  const result = diffCache.get(projectRoot);
  if (!result?.hasRegressions) {
    return {};
  }

  const regressions = getRegressionsToReport(result, failOn);
  if (regressions.length === 0) {
    return {};
  }

  return createVisitor(context, projectRoot, regressions);
}

function createVisitor(
  context: Rule.RuleContext,
  projectRoot: string,
  regressions: JsDiffResult['regressions']
): Rule.RuleListener {
  const filename = path.relative(projectRoot, context.filename);
  const isFirst = isFirstFile(context, projectRoot);

  return {
    Program(node) {
      for (const reg of regressions) {
        const affectsFile = reg.smell.files.some((f) => f === filename || filename.endsWith(f));

        if (affectsFile || isFirst) {
          context.report({
            node,
            messageId: 'regression',
            data: { message: formatRegressionMessage(reg) },
          });
        }
      }
    },
  };
}

function formatRegressionMessage(reg: JsDiffResult['regressions'][0]): string {
  const type = reg.smell.smellType;
  const files = reg.smell.files.slice(0, 3).join(', ');

  switch (reg.regressionType.type) {
    case 'NewSmell':
      return `New architectural smell: ${type} in ${files}`;
    case 'SeverityIncrease':
      return `Architectural smell ${type} worsened: severity increased from ${reg.regressionType.from} to ${reg.regressionType.to}`;
    case 'MetricWorsening':
      return `Architectural metric worsened: ${type} ${
        reg.regressionType.metric
      } ${reg.regressionType.from} → ${reg.regressionType.to} (+${(
        reg.regressionType.changePercent ?? 0
      ).toFixed(0)}%)`;
    default:
      return (reg as { message: string }).message;
  }
}

const firstFiles = new Set<string>();
function isFirstFile(context: Rule.RuleContext, projectRoot: string): boolean {
  if (!firstFiles.has(projectRoot)) {
    firstFiles.add(projectRoot);
    return true;
  }
  return false;
}

function findProjectRoot(filename: string): string {
  let dir = path.dirname(filename);
  while (dir !== path.parse(dir).root) {
    if (
      fs.existsSync(path.join(dir, 'package.json')) ||
      fs.existsSync(path.join(dir, 'archlint.yaml')) ||
      fs.existsSync(path.join(dir, '.archlint.yaml'))
    ) {
      return dir;
    }
    dir = path.dirname(dir);
  }
  return path.dirname(filename);
}

export default noRegression;
