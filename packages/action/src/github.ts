import * as github from '@actions/github';
import * as core from '@actions/core';
import { JsRegression } from '@archlinter/core';

export async function upsertComment(token: string, body: string): Promise<void> {
  const context = github.context;
  if (!context.payload.pull_request) {
    core.info('Not a pull request, skipping comment.');
    return;
  }

  const octokit = github.getOctokit(token);
  const { owner, repo } = context.repo;
  const issue_number = context.payload.pull_request.number;

  const { data: comments } = await octokit.rest.issues.listComments({
    owner,
    repo,
    issue_number,
  });

  const existingComment = comments.find(c => c.body?.includes('<!-- archlint-report -->'));

  if (existingComment) {
    await octokit.rest.issues.updateComment({
      owner,
      repo,
      comment_id: existingComment.id,
      body,
    });
    core.info('Updated existing PR comment.');
  } else {
    await octokit.rest.issues.createComment({
      owner,
      repo,
      issue_number,
      body,
    });
    core.info('Created new PR comment.');
  }
}

export function createAnnotations(regressions: JsRegression[]): void {
  for (const reg of regressions) {
    const severity = reg.smell.severity.toLowerCase();
    const title = `archlint: ${reg.smell.smellType}`;
    const message = reg.message;
    
    // SnapshotSmell in core/index.d.ts doesn't have locations? 
    // Wait, let me check JsSnapshotSmell again.
    const file = reg.smell.files && reg.smell.files.length > 0 ? reg.smell.files[0] : 'unknown';

    const annotationProperties: core.AnnotationProperties = {
      title,
      file,
      startLine: 1, // Default for now since JsSnapshotSmell lacks locations
    };

    if (severity === 'critical' || severity === 'high') {
      core.error(message, annotationProperties);
    } else if (severity === 'medium') {
      core.warning(message, annotationProperties);
    } else {
      core.notice(message, annotationProperties);
    }
  }
}

export async function setSummary(body: string): Promise<void> {
  await core.summary.addRaw(body).write();
}
