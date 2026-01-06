import { JsDiffResult } from '@archlinter/core';
import { header } from './templates/header.js';
import { regressions } from './templates/regressions.js';
import { improvements } from './templates/improvements.js';
import { footer } from './templates/footer.js';

export function formatMarkdownReport(result: JsDiffResult): string {
  let md = '';
  
  md += header(result);
  md += regressions(result.regressions);
  md += improvements(result.improvements);
  md += footer();

  return md;
}
