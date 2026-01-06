import * as core from '@actions/core';
import { JsDiffResult, JsRegression } from '@archlinter/core';
import { runArchlintDiff } from './runner.js';
import { formatMarkdownReport } from './formatter.js';
import { upsertComment, createAnnotations, setSummary } from './github.js';

async function run(): Promise<void> {
  try {
    const baseline = core.getInput('baseline');
    const failOn = core.getInput('fail-on');
    const workingDirectory = core.getInput('working-directory');
    const shouldComment = core.getInput('comment') === 'true';
    const shouldAnnotate = core.getInput('annotations') === 'true';
    const githubToken = core.getInput('github-token');

    core.info(`Running archlint diff against ${baseline}...`);

    const result = await runArchlintDiff({
      baseline,
      failOn,
      workingDirectory
    });

    const reportMd = formatMarkdownReport(result);

    // Always set summary
    await setSummary(reportMd);

    if (shouldComment && githubToken) {
      await upsertComment(githubToken, reportMd);
    }

    if (shouldAnnotate) {
      createAnnotations(result.regressions);
    }

    if (result.hasRegressions) {
      const minSeverity = core.getInput('fail-on');
      // The CLI already exits with 1 if there are regressions above fail-on,
      // but since we use ignoreReturnCode: true in the runner, we handle it here.
      // We check if any regression meets the criteria.
      const hasFailingRegressions = checkFailingRegressions(result, minSeverity);
      
      if (hasFailingRegressions) {
        core.setFailed(`Architectural regressions detected (fail-on: ${minSeverity}). See report for details.`);
      }
    }

    core.info('archlint check completed successfully.');
  } catch (error) {
    if (error instanceof Error) {
      core.setFailed(error.message);
    } else {
      core.setFailed('An unexpected error occurred.');
    }
  }
}

function checkFailingRegressions(result: JsDiffResult, failOn: string): boolean {
  if (!result.hasRegressions) return false;
  
  const severityScores: Record<string, number> = {
    'low': 0,
    'medium': 1,
    'high': 2,
    'critical': 3
  };
  
  const threshold = severityScores[failOn.toLowerCase()] ?? 1; // Default to medium
  
  return result.regressions.some((r: JsRegression) => {
    const score = severityScores[r.smell.severity.toLowerCase()] ?? 0;
    return score >= threshold;
  });
}

void run();
