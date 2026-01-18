import type { Rule } from 'eslint';
import { findProjectRoot } from '../utils/project-root';
import { runDiff, type JsDiffResult } from '@archlinter/core';
import * as fs from 'node:fs';
import * as path from 'node:path';

// Cache to run diff only once per project
const diffCache = new Map<string, JsDiffResult>();
const firstFiles = new Set<string>();

// Simple run stamp to clear cache per ESLint run (for watch mode)
let lastRunStamp = 0;

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
    // Clear cache if this is a new run (e.g. in watch mode)
    const currentRunStamp = Date.now();
    if (currentRunStamp - lastRunStamp > 5000) {
      diffCache.clear();
      firstFiles.clear();
    }
    lastRunStamp = currentRunStamp;

    const { baseline, failOn } = getOptions(context);
    const projectRoot = findProjectRoot(context.filename);

    if (!ensureDiffResult(context, projectRoot, baseline!)) {
      return {};
    }

    return reportFromCache(context, projectRoot, failOn!);
  },
};

function getOptions(context: Rule.RuleContext): RuleOptions {
  const options: RuleOptions = context.options[0] ?? {};
  return {
    baseline: options.baseline ?? '.archlint-snapshot.json',
    failOn: options.failOn ?? 'low',
  };
}

function ensureDiffResult(
  context: Rule.RuleContext,
  projectRoot: string,
  baselinePath: string
): boolean {
  if (diffCache.has(projectRoot)) {
    return true;
  }

  const absoluteBaseline = path.resolve(projectRoot, baselinePath);
  if (!fs.existsSync(absoluteBaseline)) {
    if (isFirstFile(context, projectRoot)) {
      context.report({
        loc: { line: 1, column: 0 },
        messageId: 'noBaseline',
        data: { path: baselinePath },
      });
    }
    return false;
  }

  try {
    const result = runDiff({
      baseline: absoluteBaseline,
      project: projectRoot,
    });
    diffCache.set(projectRoot, result);
    return true;
  } catch (error: unknown) {
    if (isFirstFile(context, projectRoot)) {
      context.report({
        loc: { line: 1, column: 0 },
        messageId: 'regression',
        data: { message: `Diff failed: ${error instanceof Error ? error.message : String(error)}` },
      });
    }
    return false;
  }
}

const SEVERITY_ORDER: Record<string, number> = { low: 0, medium: 1, high: 2, critical: 3 };

function getRegressionsToReport(result: JsDiffResult, failOn: string): JsDiffResult['regressions'] {
  const minSev = SEVERITY_ORDER[failOn] ?? 0;

  return result.regressions.filter((r) => {
    const sev = r.smell.severity.toLowerCase();
    return (SEVERITY_ORDER[sev] ?? 0) >= minSev;
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
        if (shouldReportRegression(reg, filename, isFirst)) {
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

function shouldReportRegression(
  reg: JsDiffResult['regressions'][0],
  filename: string,
  isFirst: boolean
): boolean {
  if (isFirst) {
    return true;
  }
  const files = reg.smell?.files;
  if (!files || !Array.isArray(files)) {
    return false;
  }
  return files.some((f) => f === filename || filename.endsWith(f));
}

function formatRegressionMessage(reg: JsDiffResult['regressions'][0]): string {
  const smell = reg.smell;
  const smellType = smell.smellType;
  const files = smell.files.slice(0, 3).join(', ');

  switch (reg.regressionType.type) {
    case 'NewSmell':
      return `New architectural smell: ${smellType} in ${files}`;
    case 'SeverityIncrease':
      return formatSeverityIncreaseMessage(reg.regressionType, smellType);
    case 'MetricWorsening':
      return formatMetricWorseningMessage(reg.regressionType, smellType);
    default:
      return String(reg.message);
  }
}

function formatSeverityIncreaseMessage(
  regType: JsDiffResult['regressions'][0]['regressionType'],
  smellType: string
): string {
  return `Architectural smell ${smellType} worsened: severity increased from ${regType.from} to ${regType.to}`;
}

function formatMetricWorseningMessage(
  regType: JsDiffResult['regressions'][0]['regressionType'],
  smellType: string
): string {
  const { metric, from, to, changePercent } = regType;
  return `Architectural metric worsened: ${smellType} ${metric} ${from} → ${to} (+${(changePercent ?? 0).toFixed(0)}%)`;
}

function isFirstFile(context: Rule.RuleContext, projectRoot: string): boolean {
  if (!firstFiles.has(projectRoot)) {
    firstFiles.add(projectRoot);
    return true;
  }
  return false;
}

export default noRegression;
